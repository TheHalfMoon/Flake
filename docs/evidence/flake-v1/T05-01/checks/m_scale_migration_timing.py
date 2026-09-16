#!/usr/bin/env python3
"""T05-01: plan section 27 M-scale (and, disk permitting, L-scale)
migration performance timing for the standalone `flake-migrate` binary,
plus independent verification of its output.

Generates a synthetic format-1 legacy vault with the requested record
count and average per-record body size, times `flake-migrate preview`
and `flake-migrate import` as real subprocesses (release build), and
independently verifies the resulting format-2 output using
`tools/independent-verify/sqlite_reader.py` -- no Flake Rust code is
executed by the verification step, only re-derived rules from
`docs/formats/format-2-canonical-sqlite.md`.

The generated dataset is always deleted before this script exits (even on
failure), regardless of size -- this repeats every run, not just once,
because this development host has very little free disk space (see
REPORT.md "Environment").

Usage:
  python3 m_scale_migration_timing.py --n 10000 --avg-bytes 100000 \
      --fehrest-bin <path> --migrate-bin <path> --label M \
      --out <raw-output-json-path>
"""
import argparse
import json
import random
import shutil
import subprocess
import sys
import time
import uuid
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[5]
sys.path.insert(0, str(REPO_ROOT / "tools" / "independent-verify"))
import sqlite_reader  # noqa: E402

WORDS = [
    "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta",
    "kappa", "lambda", "mu", "nu", "xi", "omicron", "pi", "rho", "sigma",
    "tau", "upsilon", "phi", "chi", "psi", "omega",
]


def gen_body(rng: random.Random, target_bytes: int) -> str:
    parts = []
    size = 0
    while size < target_bytes:
        line = " ".join(rng.choice(WORDS) for _ in range(14)) + "\n"
        parts.append(line)
        size += len(line)
    return "".join(parts)


def generate_legacy_vault(root: Path, fehrest_bin: str, n: int, avg_bytes: int, seed: int) -> int:
    root.mkdir(parents=True, exist_ok=True)
    subprocess.run([fehrest_bin, "init", "--vault", str(root)], check=True, capture_output=True, text=True)
    rng = random.Random(seed)
    total_bytes = 0
    heartbeat_start = time.perf_counter()
    for i in range(n):
        obj_id = str(uuid.uuid4())
        title = f"M-scale synthetic record {i}"
        body = gen_body(rng, avg_bytes)
        content = f"---\nid: {obj_id}\ntitle: {title}\n---\n{body}"
        path = root / f"record-{i:06d}.md"
        path.write_text(content, encoding="utf-8", newline="\n")
        total_bytes += len(content.encode("utf-8"))
        # A CI runner's own stdout-silence watchdog can cancel a step that
        # produces no output for several minutes -- generating a large L
        # dataset file-by-file takes long enough to trigger that on its
        # own, independent of anything actually being wrong, so a periodic
        # heartbeat here is required, not cosmetic.
        if (i + 1) % 5000 == 0 or (time.perf_counter() - heartbeat_start) > 30:
            print(f"  generated {i + 1}/{n} legacy files ({total_bytes} bytes so far)", flush=True)
            heartbeat_start = time.perf_counter()
    return total_bytes


def time_subprocess(args) -> tuple[float, subprocess.CompletedProcess]:
    """Times `args` end to end exactly like `subprocess.run`, but polls
    with a periodic heartbeat print while waiting -- see the comment in
    `generate_legacy_vault` above; a `flake-migrate import` of a large
    dataset can itself run long enough in total silence to trigger the
    same CI watchdog, independent of the subprocess actually working
    correctly. The heartbeat is emitted by this script, never by
    `flake-migrate` itself -- its own stdout/stderr streams are captured
    unchanged and only inspected after it exits, so the measured
    `elapsed` and the tool's own JSON contract are both untouched."""
    start = time.perf_counter()
    proc = subprocess.Popen(args, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    last_heartbeat = start
    while True:
        try:
            stdout, stderr = proc.communicate(timeout=30)
            break
        except subprocess.TimeoutExpired:
            now = time.perf_counter()
            print(f"  ... still waiting on {' '.join(args)} ({now - start:.0f}s elapsed)", flush=True)
            last_heartbeat = now
    elapsed = time.perf_counter() - start
    result = subprocess.CompletedProcess(args, proc.returncode, stdout, stderr)
    return elapsed, result


def free_bytes(path: Path) -> int:
    return shutil.disk_usage(path).free


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--n", type=int, required=True)
    ap.add_argument("--avg-bytes", type=int, required=True)
    ap.add_argument("--fehrest-bin", required=True)
    ap.add_argument("--migrate-bin", required=True)
    ap.add_argument("--label", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--work-dir", default=None)
    ap.add_argument("--seed", type=int, default=20260916)
    args = ap.parse_args()

    work_root = Path(args.work_dir) if args.work_dir else Path.home() / f"flake-t05-01-{args.label.lower()}-scale-tmp"
    source_root = work_root / "source"
    dest_root = work_root / "dest"
    if work_root.exists():
        shutil.rmtree(work_root)
    work_root.mkdir(parents=True)

    result = {
        "label": args.label,
        "n": args.n,
        "avg_bytes_requested": args.avg_bytes,
        "seed": args.seed,
        "free_bytes_before_generate": free_bytes(work_root),
    }

    try:
        gen_start = time.perf_counter()
        total_bytes = generate_legacy_vault(source_root, args.fehrest_bin, args.n, args.avg_bytes, args.seed)
        result["generate_seconds"] = time.perf_counter() - gen_start
        result["actual_source_payload_bytes"] = total_bytes
        result["free_bytes_after_generate"] = free_bytes(work_root)

        preview_elapsed, preview_proc = time_subprocess(
            [args.migrate_bin, "preview", str(source_root)]
        )
        result["preview_seconds"] = preview_elapsed
        result["preview_exit_code"] = preview_proc.returncode
        if preview_proc.returncode != 0:
            result["preview_stderr"] = preview_proc.stderr
            raise RuntimeError(f"preview failed: {preview_proc.stderr}")
        preview_json = json.loads(preview_proc.stdout)
        result["preview_admitted_count"] = preview_json["admitted_count"]
        result["preview_complete"] = preview_json["complete"]

        import_elapsed, import_proc = time_subprocess(
            [args.migrate_bin, "import", str(source_root), str(dest_root)]
        )
        result["import_seconds"] = import_elapsed
        result["import_exit_code"] = import_proc.returncode
        if import_proc.returncode != 0:
            result["import_stderr"] = import_proc.stderr
            raise RuntimeError(f"import failed: {import_proc.stderr}")
        import_json = json.loads(import_proc.stdout)
        result["import_imported_count"] = import_json["imported_count"]
        result["import_complete"] = import_json["complete"]

        # Independent verification: no Flake Rust code executed here.
        verify_start = time.perf_counter()
        db_path = dest_root / ".fehrest" / "canonical.sqlite"
        independent = sqlite_reader.read_vault(db_path)
        result["independent_verify_seconds"] = time.perf_counter() - verify_start
        result["independent_command_count"] = independent["command_count"]
        result["independent_current_object_count"] = len(independent["current_object"])
        result["independent_head_hash_chain_verified"] = independent["head_hash_chain_verified"]

        result["counts_agree"] = (
            preview_json["admitted_count"] == args.n
            and import_json["imported_count"] == args.n
            and independent["command_count"] == args.n
            and len(independent["current_object"]) == args.n
        )
        # Plan section 27: "Full export/import M" target 60s/max 180s;
        # "L rebuild/export/import/full verification" target 600s/max 1800s
        # (L is explicitly permitted slower reported batch times, but must
        # stay bounded).
        gate_target, gate_max = (600, 1800) if args.label == "L" else (60, 180)
        result["gate_target_seconds"] = gate_target
        result["gate_maximum_seconds"] = gate_max
        result["import_within_target"] = import_elapsed <= gate_target
        result["import_within_maximum"] = import_elapsed <= gate_max
        result["ok"] = bool(
            result["counts_agree"]
            and independent["head_hash_chain_verified"]
            and result["import_within_maximum"]
        )
    except Exception as e:  # noqa: BLE001 -- this harness's own top-level report, not product code
        result["ok"] = False
        result["error"] = str(e)
    finally:
        result["free_bytes_before_cleanup"] = free_bytes(work_root)
        shutil.rmtree(work_root, ignore_errors=True)
        result["dataset_deleted"] = not work_root.exists()

    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    Path(args.out).write_text(
        json.dumps(result, indent=2, sort_keys=True), encoding="utf-8", newline="\n"
    )
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0 if result.get("ok") else 1


if __name__ == "__main__":
    sys.exit(main())
