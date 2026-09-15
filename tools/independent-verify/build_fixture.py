#!/usr/bin/env python3
"""T02-07 disposable fixture builder.

Drives the REAL `fehrest` CLI binary (the actual product, used the way an
ordinary user would) through the complete create/capture/find/complete/
export loop, across two projects, so the independent verifier has:

  - a full-store export (`export-full`) containing both projects,
  - a project-scoped export (`export-project`) for project A only,
  - a live `canonical.sqlite` to inspect directly,
  - a rebuilt derived FTS index that is later deleted to prove the CLI
    loop does not depend on it,
  - a cross-project leak check (project B's content/IDs must never appear
    in project A's scoped export).

This script is part of the FIXTURE side, not the independent verifier. It
is allowed, and expected, to use the real Flake product to create data —
T02-07 forbids the *verifier* from linking Flake internals, not fixture
creation via the CLI a real user would run. Every fixture lives under a
disposable temp directory; nothing here is committed to the repository.

Emits one JSON document to stdout describing exactly what was built,
purely for evidence/documentation and for driving the cross-project leak
check below (never fed to the verifier as a trusted "expected" oracle for
counts/identities — the verifier re-derives all of that independently from
the SQLite/export bytes themselves).
"""
import json
import shutil
import subprocess
import sys
from pathlib import Path


def run(binary, vault, args):
    cmd = [str(binary), *args, "--vault", str(vault)]
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        raise RuntimeError(
            f"command failed ({result.returncode}): {cmd}\n"
            f"stdout: {result.stdout}\nstderr: {result.stderr}"
        )
    return result.stdout.strip()


def first_two_tokens(line):
    parts = line.split()
    return parts[0], parts[1]


def build(binary: Path, workdir: Path) -> dict:
    vault = workdir / "vault"
    run(binary, vault, ["canonical-init"])

    # --- Project A: the fully-populated project ---
    out = run(binary, vault, ["project-create", "--name", "Project A", "--description", "Independent reconstruction fixture A"])
    project_a, _ = first_two_tokens(out)

    out = run(binary, vault, ["note-create", "--project", project_a, "--title", "Fixture note", "--body", "Unicode/CRLF body: café éé\r\nsecond line \U0001F600"])
    note_id, note_rev = first_two_tokens(out)

    out = run(binary, vault, ["action-create", "--project", project_a, "--title", "Fixture action root"])
    action_root_id, action_root_rev = first_two_tokens(out)

    out = run(binary, vault, ["action-create", "--project", project_a, "--title", "Fixture action dependent", "--depends-on", action_root_id])
    action_dep_id, action_dep_rev = first_two_tokens(out)

    out = run(binary, vault, ["decision-create", "--project", project_a, "--key", "fixture-decision", "--statement", "Use SQLite format-2 for T02-07", "--basis", "evidence"])
    decision_id, decision_rev = first_two_tokens(out)

    # Source: import an actual small local file as fixture evidence bytes.
    src_file = workdir / "evidence.txt"
    src_file.write_text("independent reconstruction fixture evidence bytes\n", encoding="utf-8")
    out = run(binary, vault, ["source-import", "--project", project_a, "--label", "fixture-evidence", "--path", str(src_file)])
    source_id, source_rev = first_two_tokens(out)

    out = run(binary, vault, ["relation-create", "--project", project_a, "--type", "supports", "--from", decision_id, "--to", source_id, "--note", "decision supported by fixture evidence"])
    relation_id, relation_rev = first_two_tokens(out)

    # Complete/update loop: start+complete the root action, accept the decision.
    out = run(binary, vault, ["action-start", "--id", action_root_id, "--expect", action_root_rev])
    action_root_rev = out.split()[1]
    out = run(binary, vault, ["action-complete", "--id", action_root_id, "--expect", action_root_rev, "--summary", "done for fixture"])
    action_root_rev = out.split()[1]

    out = run(binary, vault, ["decision-accept", "--id", decision_id, "--expect", decision_rev])
    decision_rev = out.split()[1]

    # A second update revision on the note, to prove full-history export.
    out = run(binary, vault, ["note-update", "--id", note_id, "--expect", note_rev, "--body", "Updated body after fixture edit"])
    note_rev = out.split()[1]

    # --- Project B: isolation control, must never leak into A's scoped export ---
    out = run(binary, vault, ["project-create", "--name", "Project B secret", "--description", "must never appear in A's export"])
    project_b, _ = first_two_tokens(out)
    out = run(binary, vault, ["note-create", "--project", project_b, "--title", "B only", "--body", "SECRET-MARKER-PROJECT-B-CONTENT-9f3a"])
    note_b_id, _ = first_two_tokens(out)

    # --- Find: FTS index build + search over project A ---
    run(binary, vault, ["fts-rebuild"])
    # Search for text present in the *current* (post-update) note body, not
    # the original body -- the index is rebuilt after `note-update` above.
    search_out = run(binary, vault, ["fts-search", "--project", project_a, "--query", "Updated"])

    # --- Export: full store, and project A scoped ---
    export_full = workdir / "export-full"
    export_project = workdir / "export-project-a"
    run(binary, vault, ["export-run", "--out", str(export_full)])
    run(binary, vault, ["export-run", "--project", project_a, "--out", str(export_project)])

    manifest = {
        "vault": str(vault),
        "export_full_dir": str(export_full / ".fehrest-export"),
        "export_project_dir": str(export_project / ".fehrest-export"),
        "project_a": project_a,
        "project_b": project_b,
        "project_a_object_ids": sorted({
            project_a, note_id, action_root_id, action_dep_id, decision_id, source_id, relation_id,
        }),
        "project_b_object_ids": sorted({project_b, note_b_id}),
        "project_b_secret_marker": "SECRET-MARKER-PROJECT-B-CONTENT-9f3a",
        "fts_search_hit_count_before_index_removed": search_out.splitlines()[1] if len(search_out.splitlines()) > 1 else None,
    }
    return manifest


def prove_derived_index_removal(binary: Path, vault: Path, project_a: str, workdir: Path) -> dict:
    """Delete the derived FTS index, then prove the canonical CLI loop
    (record-show / project-records / export-run) still works correctly
    without it -- derived state is disposable, canonical state is not."""
    index_path = vault / ".fehrest" / "derived-fts.sqlite"
    existed_before = index_path.exists()
    if index_path.exists():
        index_path.unlink()

    records_out = run(binary, vault, ["project-records", "--project", project_a])
    reexport_dir = workdir / "export-after-index-removed"
    export_out = run(binary, vault, ["export-run", "--project", project_a, "--out", str(reexport_dir)])

    return {
        "index_existed_before_removal": existed_before,
        "index_exists_after_removal": index_path.exists(),
        "project_records_after_removal": records_out,
        "export_after_index_removed_dir": str(reexport_dir / ".fehrest-export"),
        "export_after_index_removed_output": export_out,
    }


def main():
    if len(sys.argv) != 3:
        print("usage: build_fixture.py <fehrest-binary> <workdir>", file=sys.stderr)
        return 2
    binary = Path(sys.argv[1]).resolve()
    workdir = Path(sys.argv[2]).resolve()
    if workdir.exists():
        shutil.rmtree(workdir)
    workdir.mkdir(parents=True)

    manifest = build(binary, workdir)
    removal = prove_derived_index_removal(binary, Path(manifest["vault"]), manifest["project_a"], workdir)
    manifest["derived_index_removal_proof"] = removal

    print(json.dumps(manifest, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
