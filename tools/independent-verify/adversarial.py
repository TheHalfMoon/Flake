"""Adversarial suite for the T02-07 independent verifier: hand-corrupts
copies of a valid export package (plain file/byte edits with this script,
never through Flake's own export/import code) and asserts `export_reader`
detects and clearly reports each one -- never silently drops a bad record.

Each case function takes a fresh copy of a valid export directory, mutates
it, and returns (expected_outcome, detail) where expected_outcome is one
of "raises" (export_reader.read_export must raise ExportFormatError,
matching a given substring) or "dangling" (read_export succeeds but
`dangling_references` must be non-empty and mention the given substring).
"""
import hashlib
import json
import shutil
from pathlib import Path

import export_reader


def sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _copy_export(src: Path, dst: Path) -> Path:
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(src, dst)
    return dst


def _load_manifest(export_dir: Path) -> dict:
    return json.loads((export_dir / "export-manifest.json").read_text(encoding="utf-8"))


def _save_manifest(export_dir: Path, manifest: dict, *, recompute_root: bool = True) -> None:
    if recompute_root:
        manifest["integrity_root"] = export_reader.recompute_integrity_root(manifest)
    (export_dir / "export-manifest.json").write_text(
        json.dumps(manifest, indent=2), encoding="utf-8"
    )


def _first_member_path(manifest: dict, *, suffix: str = ".json", contains: str = "revisions/") -> str:
    for m in manifest["members"]:
        if contains in m["path"] and m["path"].endswith(suffix):
            return m["path"]
    raise AssertionError(f"no member matching contains={contains!r} suffix={suffix!r}")


def _member_of_kind(export_dir: Path, manifest: dict, kind: str) -> str:
    for m in manifest["members"]:
        if not m["path"].endswith(".json") or "revisions/" not in m["path"]:
            continue
        doc = json.loads((export_dir / m["path"]).read_text(encoding="utf-8"))
        if doc.get("kind") == kind:
            return m["path"]
    raise AssertionError(f"no revision member of kind {kind!r}")


def _update_member_hash(export_dir: Path, manifest: dict, path: str) -> None:
    data = (export_dir / path).read_bytes()
    for m in manifest["members"]:
        if m["path"] == path:
            m["length"] = len(data)
            m["sha256"] = sha256_hex(data)
            return
    raise AssertionError(f"member {path} not found in manifest")


# --- case A: missing exported member ---
def case_missing_member(export_dir: Path) -> tuple[str, str]:
    manifest = _load_manifest(export_dir)
    path = _first_member_path(manifest)
    (export_dir / path).unlink()
    return "raises", "missing declared member"


# --- case B: manifest/member length mismatch ---
def case_member_length_mismatch(export_dir: Path) -> tuple[str, str]:
    manifest = _load_manifest(export_dir)
    path = _first_member_path(manifest)
    full = export_dir / path
    full.write_bytes(full.read_bytes() + b" ")  # length changes, manifest not updated
    return "raises", "declared length"


# --- case C: manifest/member digest mismatch (same length) ---
def case_member_digest_mismatch(export_dir: Path) -> tuple[str, str]:
    manifest = _load_manifest(export_dir)
    path = _first_member_path(manifest)
    full = export_dir / path
    data = bytearray(full.read_bytes())
    # Flip a printable ASCII byte in place, same length, so only the
    # digest check (not the length check) can catch this.
    for i, b in enumerate(data):
        if 0x30 <= b <= 0x39:  # a digit
            data[i] = 0x30 if b != 0x30 else 0x31
            break
    else:
        data[0] ^= 0x01
    full.write_bytes(bytes(data))
    return "raises", "declared sha256"


# --- case D: tampered manifest integrity_root (a field the root actually covers) ---
def case_tampered_integrity_root(export_dir: Path) -> tuple[str, str]:
    manifest = _load_manifest(export_dir)
    manifest["snapshot_head_seq"] = manifest["snapshot_head_seq"] + 1000
    _save_manifest(export_dir, manifest, recompute_root=False)
    return "raises", "integrity_root mismatch"


# --- case E: duplicate (object_id, revision_id) pair ---
def case_duplicate_object_revision_pair(export_dir: Path) -> tuple[str, str]:
    manifest = _load_manifest(export_dir)
    src_path = _first_member_path(manifest)
    src_full = export_dir / src_path
    doc = json.loads(src_full.read_text(encoding="utf-8"))
    # Same object_id/revision_id, placed at a *different* recorded_seq path
    # -- a structurally different file the manifest will treat as a
    # distinct, additional member, but whose declared identity collides.
    fake_seq = doc["recorded_seq"] + 500
    dup_path = f"revisions/{doc['object_id']}/{fake_seq}-{doc['revision_id']}.json"
    dup_full = export_dir / dup_path
    dup_full.parent.mkdir(parents=True, exist_ok=True)
    dup_doc = dict(doc)
    dup_doc["recorded_seq"] = fake_seq
    dup_text = json.dumps(dup_doc)
    dup_full.write_text(dup_text, encoding="utf-8")
    manifest["members"].append({
        "path": dup_path, "length": len(dup_text.encode("utf-8")), "sha256": sha256_hex(dup_text.encode("utf-8")),
    })
    _save_manifest(export_dir, manifest)
    return "raises", "duplicate (object_id, revision_id)"


# --- case F: broken relation endpoint (points outside the package) ---
def case_broken_relation_endpoint(export_dir: Path) -> tuple[str, str]:
    manifest = _load_manifest(export_dir)
    path = _member_of_kind(export_dir, manifest, "relation")
    full = export_dir / path
    doc = json.loads(full.read_text(encoding="utf-8"))
    payload = json.loads(doc["payload_raw"])
    payload["to_object_id"] = "01a0a3c4-0000-7000-8000-000000000000"  # not in package
    new_payload_raw = json.dumps(payload)
    doc["payload_raw"] = new_payload_raw
    doc["payload_sha256"] = sha256_hex(new_payload_raw.encode("utf-8"))
    text = json.dumps(doc)
    full.write_text(text, encoding="utf-8")
    _update_member_hash(export_dir, manifest, path)
    _save_manifest(export_dir, manifest)
    return "dangling", "not in this package"


# --- case G: action dependency reference absent from the package ---
def case_action_dependency_absent(export_dir: Path) -> tuple[str, str]:
    manifest = _load_manifest(export_dir)
    path = _member_of_kind(export_dir, manifest, "action")
    full = export_dir / path
    doc = json.loads(full.read_text(encoding="utf-8"))
    payload = json.loads(doc["payload_raw"])
    payload["dependency_ids"] = ["01a0a3c4-1111-7000-8000-000000000000"]
    new_payload_raw = json.dumps(payload)
    doc["payload_raw"] = new_payload_raw
    doc["payload_sha256"] = sha256_hex(new_payload_raw.encode("utf-8"))
    full.write_text(json.dumps(doc), encoding="utf-8")
    _update_member_hash(export_dir, manifest, path)
    _save_manifest(export_dir, manifest)
    return "dangling", "depends_on"


# --- case H: truncated artifact (valid per manifest, invalid JSON) ---
def case_truncated_artifact(export_dir: Path) -> tuple[str, str]:
    manifest = _load_manifest(export_dir)
    path = _first_member_path(manifest)
    full = export_dir / path
    data = full.read_bytes()
    truncated = data[: len(data) // 2]
    full.write_bytes(truncated)
    _update_member_hash(export_dir, manifest, path)  # attacker controls the whole package
    _save_manifest(export_dir, manifest)
    return "raises", "not valid JSON"


# --- case I: missing object required by another record (referenced object fully removed) ---
def case_missing_referenced_object(export_dir: Path) -> tuple[str, str]:
    manifest = _load_manifest(export_dir)
    source_path = _member_of_kind(export_dir, manifest, "source")
    doc = json.loads((export_dir / source_path).read_text(encoding="utf-8"))
    object_id = doc["object_id"]
    # Remove every member belonging to this object_id entirely.
    kept = [m for m in manifest["members"] if f"revisions/{object_id}/" not in m["path"]]
    removed = [m for m in manifest["members"] if f"revisions/{object_id}/" in m["path"]]
    for m in removed:
        (export_dir / m["path"]).unlink()
    manifest["members"] = kept
    _save_manifest(export_dir, manifest)
    return "dangling", object_id


# --- case J: invalid generic-reader schema expectation ---
def case_unrecognized_manifest_schema(export_dir: Path) -> tuple[str, str]:
    manifest = _load_manifest(export_dir)
    manifest["schema"] = "flake-export-manifest-v2-unknown"
    _save_manifest(export_dir, manifest)
    return "raises", "unexpected manifest schema"


# --- case K: source capture bytes do not match their own declared digest ---
def case_source_bytes_digest_mismatch(export_dir: Path) -> tuple[str, str]:
    manifest = _load_manifest(export_dir)
    path = _member_of_kind(export_dir, manifest, "source")
    full = export_dir / path
    doc = json.loads(full.read_text(encoding="utf-8"))
    payload = json.loads(doc["payload_raw"])
    capture = payload.get("capture")
    if capture is None or not capture.get("bytes_hex"):
        raise AssertionError("fixture source has no capture.bytes_hex to corrupt")
    hex_chars = list(capture["bytes_hex"])
    idx = 0
    original = hex_chars[idx]
    hex_chars[idx] = "0" if original != "0" else "1"
    capture["bytes_hex"] = "".join(hex_chars)
    payload["capture"] = capture
    new_payload_raw = json.dumps(payload)
    doc["payload_raw"] = new_payload_raw
    doc["payload_sha256"] = sha256_hex(new_payload_raw.encode("utf-8"))
    full.write_text(json.dumps(doc), encoding="utf-8")
    _update_member_hash(export_dir, manifest, path)
    _save_manifest(export_dir, manifest)
    return "raises", "capture.sha256"


CASES = {
    "missing_member": case_missing_member,
    "member_length_mismatch": case_member_length_mismatch,
    "member_digest_mismatch": case_member_digest_mismatch,
    "tampered_integrity_root": case_tampered_integrity_root,
    "duplicate_object_revision_pair": case_duplicate_object_revision_pair,
    "broken_relation_endpoint": case_broken_relation_endpoint,
    "action_dependency_absent": case_action_dependency_absent,
    "truncated_artifact": case_truncated_artifact,
    "missing_referenced_object": case_missing_referenced_object,
    "unrecognized_manifest_schema": case_unrecognized_manifest_schema,
    "source_bytes_digest_mismatch": case_source_bytes_digest_mismatch,
}


def run_all_cases(valid_export_dir: Path, scratch_dir: Path) -> list[dict]:
    """Returns one result dict per case: {name, expected, detail, passed, message}."""
    results = []
    for name, fn in CASES.items():
        case_dir = _copy_export(valid_export_dir, scratch_dir / name)
        expected, detail = fn(case_dir)
        result = {"name": name, "expected": expected, "detail": detail}
        try:
            report = export_reader.read_export(case_dir)
            if expected == "raises":
                result["passed"] = False
                result["message"] = "expected export_reader to raise, but it succeeded"
            else:  # expected == "dangling"
                joined = " | ".join(report["dangling_references"])
                if report["dangling_references"] and detail in joined:
                    result["passed"] = True
                    result["message"] = joined
                else:
                    result["passed"] = False
                    result["message"] = f"expected a dangling reference containing {detail!r}, got: {joined!r}"
        except export_reader.ExportFormatError as e:
            if expected == "raises" and detail in str(e):
                result["passed"] = True
                result["message"] = str(e)
            else:
                result["passed"] = False
                result["message"] = f"raised, but unexpected: {e}"
        results.append(result)
    return results
