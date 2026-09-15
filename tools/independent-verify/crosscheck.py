"""Cross-check step: compares a SQLite-derived semantic report against an
export-derived one. This is the actual "no self-validating oracle" proof --
`canonical.sqlite` and a `.fehrest-export/` package are two artifacts
produced by two different Flake code paths from the same underlying store,
read here by two independently-implemented raw readers
(`sqlite_reader.py`, `export_reader.py`). Agreement between their two
independently-derived semantic reports is the evidence; neither report is
treated as ground truth for the other, and neither is produced by
re-running Flake's own import/export code.
"""
import semantic_model


class CrossCheckError(Exception):
    pass


def _diff_objects(label: str, a: dict, b: dict) -> list[str]:
    problems = []
    keys_a, keys_b = set(a), set(b)
    if keys_a != keys_b:
        problems.append(
            f"{label}: object id sets differ -- only in first: {sorted(keys_a - keys_b)}, "
            f"only in second: {sorted(keys_b - keys_a)}"
        )
    for object_id in keys_a & keys_b:
        oa, ob = a[object_id], b[object_id]
        if oa["kind"] != ob["kind"]:
            problems.append(f"{label}[{object_id}]: kind differs {oa['kind']!r} vs {ob['kind']!r}")
        if oa["current_revision_id"] != ob["current_revision_id"]:
            problems.append(
                f"{label}[{object_id}]: current_revision_id differs "
                f"{oa['current_revision_id']!r} vs {ob['current_revision_id']!r}"
            )
        if oa["revision_count"] != ob["revision_count"]:
            problems.append(
                f"{label}[{object_id}]: revision_count differs "
                f"{oa['revision_count']} vs {ob['revision_count']}"
            )
        if oa["current_payload"] != ob["current_payload"]:
            problems.append(f"{label}[{object_id}]: current_payload differs")
    return problems


def compare_reports(label: str, report_a: dict, report_b: dict) -> list[str]:
    """Full structural comparison of two semantic reports built from the
    same declared scope. Returns a list of human-readable problems (empty
    means exact agreement)."""
    problems = []
    if report_a["record_count_by_kind"] != report_b["record_count_by_kind"]:
        problems.append(
            f"{label}: record_count_by_kind differs: "
            f"{report_a['record_count_by_kind']} vs {report_b['record_count_by_kind']}"
        )
    if report_a["revision_count"] != report_b["revision_count"]:
        problems.append(
            f"{label}: revision_count differs: {report_a['revision_count']} vs {report_b['revision_count']}"
        )
    problems.extend(_diff_objects(label, report_a["objects"], report_b["objects"]))
    if report_a["relations"] != report_b["relations"]:
        problems.append(f"{label}: relations differ")
    if report_a["action_dependencies"] != report_b["action_dependencies"]:
        problems.append(f"{label}: action_dependencies differ")
    if report_a["source_digests"] != report_b["source_digests"]:
        problems.append(f"{label}: source_digests differ")
    if report_a["decision_states"] != report_b["decision_states"]:
        problems.append(f"{label}: decision_states differ")
    return problems


def compare_full_export(sqlite_report: dict, export_full_report: dict) -> list[str]:
    """A full-store export must be exactly equal to the live SQLite store's
    own semantic report -- same objects, same current state, same
    relations/dependencies, byte for byte on every semantic fact."""
    return compare_reports("full-export vs sqlite", sqlite_report, export_full_report)


def compare_project_export(
    sqlite_full_report: dict, export_project_report: dict, project_id: str,
    forbidden_object_ids: set,
) -> list[str]:
    """A project-scoped export must equal exactly the project-scoped
    subset independently re-derived from the full SQLite report (never a
    fixture-supplied "expected" list), and must contain none of another
    project's object ids."""
    expected_subset = semantic_model.project_scoped_subset(sqlite_full_report, project_id)
    problems = compare_reports("project-export vs sqlite subset", expected_subset, export_project_report)

    leaked = forbidden_object_ids & set(export_project_report["objects"])
    if leaked:
        problems.append(f"project export leaks foreign object ids: {sorted(leaked)}")
    return problems
