# T03-08 — Automated continuity qualification

Executes the founder-authorized replacement for T03-07's human confirmatory
trial: every one of T03-07's 96 sealed confirmatory cases run through the
real Flake CLI (`flake_arm.py`) and an independent maintained-Markdown
baseline (`baseline_arm.py`), producing 192 raw attempts, graded against the
unmodified T03-07 gold keys by two independently-written implementations
(`analysis.py` / `verify_independent.py`). See `PROTOCOL_ADDENDUM.md` for
the full design and `docs/canonical/FOUNDER_NO_HUMAN_QUALIFICATION_GATES_2026-09-16.md`
for the authorizing decision.

## Files

- `PROTOCOL_ADDENDUM.md` -- the automated-execution design and pass criteria.
- `flake_arm.py` -- real Flake CLI/Core execution arm.
- `baseline_arm.py` -- independent maintained-Markdown execution arm.
- `harness.py` -- resumable, append-only orchestrator (`runs/<name>/records.jsonl`).
- `analysis.py` -- producer: raw manifest -> Pass/Fail/Inconclusive route.
- `verify_independent.py` -- oracle: independently recomputes the same route.
- `seal.py` -- SHA-256 seal of the harness/analysis code and the reused T03-07 case bank.
- `test_*.py` / `run_tests.py` -- test suites (aggregate with `python3 run_tests.py`).
- `runs/dry-run/` -- synthetic run against `cases_dev.json` only (`case_source: "dev_synthetic"`).
- `runs/confirmatory/` -- the real 192-attempt run against the sealed case bank.

## Running

```bash
# Dry run against non-confirmatory dev cases only.
python3 harness.py --cases ../T03-07/cases_dev.json --run-name dry-run --case-source dev_synthetic

# Seal the code before any confirmatory attempt.
python3 seal.py

# Confirmatory run against the sealed T03-07 case bank.
python3 harness.py --cases ../T03-07/cases_confirmatory.json --run-name confirmatory --case-source confirmatory

# Producer and oracle routing.
python3 analysis.py --manifest runs/confirmatory/records.jsonl --cases ../T03-07/cases_confirmatory.json --forbid-case-source dev_synthetic
python3 verify_independent.py --manifest runs/confirmatory/records.jsonl --cases ../T03-07/cases_confirmatory.json --forbid-case-source dev_synthetic
```

Full evidence, raw command output, and the acceptance-criteria disposition
are in `docs/evidence/flake-v1/T03-08/REPORT.md`.
