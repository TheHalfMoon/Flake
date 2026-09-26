#!/usr/bin/env python3
"""T05-05: repeat the legacy migration path from the published instructions.

Builds a small deterministic format-1 legacy vault (the way USER_GUIDE.md
section 6 describes: `pluma init` plus plain Markdown records), runs
`pluma-migrate preview` (read-only, must not write anything) and
`pluma-migrate import` into a fresh destination, then independently verifies
the format-2 result with tools/independent-verify/sqlite_reader.py -- no
Pluma Rust code is executed by the verification step.

The fixture deliberately includes the same edge shapes the repository's own
gold fixtures cover: CRLF line endings, non-ASCII content, trailing
whitespace, and one record carrying unknown frontmatter fields (which the
published preview documents as an omission, never a silent drop).

Usage:
  python3 legacy_migration_repeat.py --pluma-bin <path> --migrate-bin <path> \
      --workdir <disposable-dir> --out <report-json-path>

Exits non-zero on any failure. The workdir is always deleted before exit.
"""
import argparse
import json
import shutil
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[5]
sys.path.insert(0, str(REPO_ROOT / "tools" / "independent-verify"))
import sqlite_reader  # noqa: E402


def run(cmd, **kwargs):
    result = subprocess.run(cmd, capture_output=True, text=True, **kwargs)
    if result.returncode != 0:
        raise RuntimeError(
            f"command failed ({result.returncode}): {cmd}\n"
            f"stdout: {result.stdout}\nstderr: {result.stderr}"
        )
    return result


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pluma-bin", required=True)
    parser.add_argument("--migrate-bin", required=True)
    parser.add_argument("--workdir", required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    pluma = str(args.pluma_bin)
    migrate = str(args.migrate_bin)
    workdir = Path(args.workdir)
    report_path = Path(args.out)
    report: dict = {"checks": {}}

    try:
        if workdir.exists():
            shutil.rmtree(workdir)
        workdir.mkdir(parents=True)
        src = workdir / "legacy-src"
        src.mkdir()

        run([pluma, "init", "--vault", str(src)])
        report["checks"]["legacy_init"] = "ok"

        records = {
            "record-000000.md": "---\nid: 018f4d2a-1111-7890-abcd-ef0123456781\ntitle: First legacy note\n---\nPlain body, first record.\n",
            "record-000001.md": "---\nid: 018f4d2a-2222-7890-abcd-ef0123456782\ntitle: Legacy CRLF and Unicode\n---\nUnicode body: caf\u00e9 \U0001f600\r\nSecond line with trailing spaces   \r\n",
            "record-000002.md": "---\nid: 018f4d2a-3333-7890-abcd-ef0123456783\ntitle: Legacy With Unknown Fields\nfuture_field: not understood by the current frontmatter parser\n---\nBody content for the unknown-fields record.\n",
            "record-000003.md": "---\nid: 018f4d2a-4444-7890-abcd-ef0123456784\ntitle: Fourth legacy note\n---\nAnother plain body.\n",
            "record-000004.md": "---\nid: 018f4d2a-5555-7890-abcd-ef0123456785\ntitle: Fifth legacy note\n---\nFinal plain body.\n",
        }
        for name, content in records.items():
            (src / name).write_text(content, encoding="utf-8", newline="")
        before = sorted(p.name for p in src.iterdir())
        report["checks"]["fixture_records_written"] = len(records)

        preview = run([migrate, "preview", str(src)])
        preview_doc = json.loads(preview.stdout)
        report["checks"]["preview_schema"] = preview_doc.get("schema")
        report["checks"]["preview_admitted"] = preview_doc.get("admitted_count")
        report["checks"]["preview_complete"] = preview_doc.get("complete")
        after_preview = sorted(p.name for p in src.iterdir())
        if after_preview != before:
            raise RuntimeError(f"preview modified the source vault: {before} -> {after_preview}")
        report["checks"]["preview_wrote_nothing"] = True

        dest = workdir / "migrated"
        import_result = run([migrate, "import", str(src), str(dest)])
        import_doc = json.loads(import_result.stdout)
        report["checks"]["import_imported_count"] = import_doc.get("imported_count")
        report["checks"]["import_complete"] = import_doc.get("complete")
        if import_doc.get("imported_count") != preview_doc.get("admitted_count"):
            raise RuntimeError(
                f"imported {import_doc.get('imported_count')} != preview-admitted {preview_doc.get('admitted_count')}"
            )
        report["checks"]["import_matches_preview_admission"] = True
        if sorted(p.name for p in src.iterdir()) != before:
            raise RuntimeError("import modified the source vault")
        report["checks"]["source_untouched_by_import"] = True

        db_path = dest / ".fehrest" / "canonical.sqlite"
        vault_result = sqlite_reader.read_vault(db_path)
        report["checks"]["head_hash_chain_verified"] = vault_result["head_hash_chain_verified"]
        report["checks"]["command_count"] = vault_result["command_count"]
        if not vault_result["head_hash_chain_verified"]:
            raise RuntimeError("migrated vault head-hash chain did not verify")
        if vault_result["command_count"] < 1:
            raise RuntimeError("migrated vault has no commands")

        report["passed"] = True
    except Exception as exc:
        report["passed"] = False
        report["error"] = str(exc)[:2000]
        raise SystemExit(f"MIGRATION_REPEAT_FAILED: {exc}")
    finally:
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(json.dumps(report, indent=2), encoding="utf-8")
        shutil.rmtree(workdir, ignore_errors=True)

    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
