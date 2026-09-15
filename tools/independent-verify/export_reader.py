"""Raw reader #2 for the T02-07 independent verifier: reads a published
`.fehrest-export/` directory as plain JSON/filesystem, independently
re-verifying every hash `docs/formats/portable-export-v1.md` documents
before trusting any of it.

Independence: no Flake binary, no Flake library, no Rust source is
imported or executed by this file, and this file does not import
`sqlite_reader.py` (a genuinely separate raw-parsing code path, per
README.md). Every rule below -- the manifest shape, `integrity_root`'s
exact hashed JSON object, `payload_sha256`'s definition, the
`revisions/<object_id>/<recorded_seq>-<revision_id>.json` path shape -- is
transcribed from that document's own published text.
"""
import hashlib
import json
from pathlib import Path


class ExportFormatError(Exception):
    pass


def sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _read_bytes(path: Path) -> bytes:
    if not path.exists():
        raise ExportFormatError(f"missing declared member: {path}")
    return path.read_bytes()


def recompute_integrity_root(manifest: dict) -> str:
    """`integrity_root` = SHA-256 of the JSON object
    `{schema, kind, vault_id, project_id, snapshot_head_seq, members}`,
    members sorted by path -- per the format document. The producing Rust
    code builds this value through `serde_json::json!` backed by a
    `BTreeMap` (no `preserve_order` feature), which serializes object keys
    in sorted order; `sort_keys=True` with compact, non-ASCII-escaping
    JSON here reproduces that exact byte sequence for the ASCII/hex/UUID
    content this format actually contains."""
    members = sorted(manifest.get("members", []), key=lambda m: m["path"])
    value = {
        "schema": manifest["schema"],
        "kind": manifest["kind"],
        "vault_id": manifest["vault_id"],
        "project_id": manifest.get("project_id"),
        "snapshot_head_seq": manifest["snapshot_head_seq"],
        "members": [
            {"length": m["length"], "path": m["path"], "sha256": m["sha256"]}
            for m in members
        ],
    }
    blob = json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    return sha256_hex(blob.encode("utf-8"))


def verify_manifest_and_members(export_dir: Path) -> dict:
    manifest_path = export_dir / "export-manifest.json"
    if not manifest_path.exists():
        raise ExportFormatError(f"missing export-manifest.json in {export_dir}")
    try:
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as e:
        raise ExportFormatError(f"export-manifest.json is not valid JSON: {e}") from e

    if manifest.get("schema") != "flake-export-manifest-v1":
        raise ExportFormatError(f"unexpected manifest schema: {manifest.get('schema')!r}")
    if manifest.get("kind") not in ("export-full", "export-project"):
        raise ExportFormatError(f"unexpected manifest kind: {manifest.get('kind')!r}")
    if manifest["kind"] == "export-full" and manifest.get("project_id") is not None:
        raise ExportFormatError("export-full manifest must not declare a project_id")
    if manifest["kind"] == "export-project" and manifest.get("project_id") is None:
        raise ExportFormatError("export-project manifest must declare a project_id")

    seen_paths = set()
    for member in manifest.get("members", []):
        path = member["path"]
        if path in seen_paths:
            raise ExportFormatError(f"duplicate member path declared twice: {path}")
        seen_paths.add(path)
        data = _read_bytes(export_dir / path)
        if len(data) != member["length"]:
            raise ExportFormatError(
                f"member {path}: declared length {member['length']} != actual {len(data)}"
            )
        actual_sha = sha256_hex(data)
        if actual_sha != member["sha256"]:
            raise ExportFormatError(
                f"member {path}: declared sha256 {member['sha256']} != actual {actual_sha}"
            )

    recomputed_root = recompute_integrity_root(manifest)
    if recomputed_root != manifest.get("integrity_root"):
        raise ExportFormatError(
            f"integrity_root mismatch: declared={manifest.get('integrity_root')} "
            f"recomputed={recomputed_root}"
        )

    return manifest


def read_revision_envelopes(export_dir: Path, manifest: dict) -> list[dict]:
    """Every `revisions/<object_id>/<seq>-<revision_id>.json` member
    declared in the manifest, independently re-verified and cross-checked
    against its own path for object_id/seq/revision_id consistency."""
    envelopes = []
    seen_pairs = set()
    for member in manifest.get("members", []):
        path = member["path"]
        if not path.startswith("revisions/") or not path.endswith(".json"):
            continue
        full_path = export_dir / path
        try:
            rev_doc = json.loads(full_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as e:
            raise ExportFormatError(f"revision file {path} is not valid JSON: {e}") from e

        if rev_doc.get("schema") != "flake-export-revision-v1":
            raise ExportFormatError(f"{path}: unexpected revision schema {rev_doc.get('schema')!r}")

        payload_raw = rev_doc.get("payload_raw")
        if payload_raw is None:
            raise ExportFormatError(f"{path}: missing payload_raw")
        recomputed_payload_sha = sha256_hex(payload_raw.encode("utf-8"))
        if recomputed_payload_sha != rev_doc.get("payload_sha256"):
            raise ExportFormatError(
                f"{path}: payload_sha256 mismatch: declared={rev_doc.get('payload_sha256')} "
                f"recomputed={recomputed_payload_sha}"
            )
        try:
            payload = json.loads(payload_raw)
        except json.JSONDecodeError as e:
            raise ExportFormatError(f"{path}: payload_raw is not valid JSON: {e}") from e

        payload_kind = payload.get("kind")
        envelope_kind = rev_doc.get("kind")
        if payload_kind != envelope_kind:
            raise ExportFormatError(
                f"{path}: envelope kind {envelope_kind!r} disagrees with payload's own "
                f"kind {payload_kind!r} -- the format document says these must agree"
            )

        object_id = rev_doc.get("object_id")
        revision_id = rev_doc.get("revision_id")
        recorded_seq = rev_doc.get("recorded_seq")

        # Cross-check the path itself against the declared content -- a
        # mismatch here would mean the export directory structure lies
        # about what it contains.
        expected_path = f"revisions/{object_id}/{recorded_seq}-{revision_id}.json"
        if path != expected_path:
            raise ExportFormatError(
                f"revision file path {path} does not match its own declared "
                f"object_id/recorded_seq/revision_id (expected {expected_path})"
            )

        pair = (object_id, revision_id)
        if pair in seen_pairs:
            raise ExportFormatError(f"duplicate (object_id, revision_id) pair: {pair}")
        seen_pairs.add(pair)

        envelopes.append({
            "object_id": object_id,
            "revision_id": revision_id,
            "parent_revision_id": rev_doc.get("parent_revision_id"),
            "recorded_seq": recorded_seq,
            "recorded_at": rev_doc.get("recorded_at"),
            "actor": rev_doc.get("actor"),
            "origin": rev_doc.get("origin"),
            "kind": payload_kind,
            "payload": payload,
        })
    return envelopes


def verify_no_dangling_references(envelopes: list[dict]) -> list[str]:
    """Detect references that point outside the package itself: a
    Relation endpoint or an Action dependency naming an object_id this
    package does not declare. Returns a list of problems found (empty if
    none) -- callers decide whether that is fatal (a full-store export
    should never have one; a project-scoped export legitimately narrows
    scope, so this is informational there unless the task explicitly
    calls it self-contained)."""
    known_ids = {e["object_id"] for e in envelopes}
    problems = []
    for env in envelopes:
        if env["kind"] == "relation":
            payload = env["payload"]
            for field in ("from_object_id", "to_object_id"):
                target = payload.get(field)
                if target is not None and target not in known_ids:
                    problems.append(
                        f"relation {env['object_id']}.{field}={target} is not in this package"
                    )
        elif env["kind"] == "action":
            for dep in env["payload"].get("dependency_ids") or []:
                if dep not in known_ids:
                    problems.append(
                        f"action {env['object_id']} depends_on {dep}, not in this package"
                    )
    return problems


def verify_source_capture_digests(envelopes: list[dict]) -> None:
    """A `Source` record's `capture.bytes_hex` must actually hash to its
    own declared `capture.sha256` at its own declared `capture.byte_length`
    -- an internal payload-level digest the manifest's own member hashing
    cannot catch by itself, since the manifest only proves the JSON file's
    outer bytes are unmodified, not that the source-evidence bytes inside
    it are internally self-consistent."""
    for env in envelopes:
        if env["kind"] != "source":
            continue
        capture = env["payload"].get("capture")
        if capture is None:
            continue
        bytes_hex = capture.get("bytes_hex", "")
        try:
            raw = bytes.fromhex(bytes_hex)
        except ValueError as e:
            raise ExportFormatError(
                f"source {env['object_id']}: capture.bytes_hex is not valid hex: {e}"
            ) from e
        if len(raw) != capture.get("byte_length"):
            raise ExportFormatError(
                f"source {env['object_id']}: capture.byte_length={capture.get('byte_length')} "
                f"!= actual decoded length {len(raw)}"
            )
        actual_sha = sha256_hex(raw)
        if actual_sha != capture.get("sha256"):
            raise ExportFormatError(
                f"source {env['object_id']}: capture.sha256={capture.get('sha256')} "
                f"!= recomputed {actual_sha} (source evidence bytes do not match their own digest)"
            )


def read_export(export_dir: Path) -> dict:
    """Top-level entry point. `export_dir` is the `.fehrest-export`
    directory itself (not its parent)."""
    manifest = verify_manifest_and_members(export_dir)
    envelopes = read_revision_envelopes(export_dir, manifest)
    verify_source_capture_digests(envelopes)
    dangling = verify_no_dangling_references(envelopes)
    return {
        "manifest": manifest,
        "envelopes": envelopes,
        "dangling_references": dangling,
    }
