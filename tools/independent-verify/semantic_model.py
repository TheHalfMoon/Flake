"""Shared, format-agnostic semantic modeling for the T02-07 independent
verifier. See README.md for why sharing this file does not compromise the
independence of the two raw readers: it contains no raw parsing or hashing
logic for either artifact, only the pure step of turning a list of already
-verified "revision envelopes" into semantic facts.

A revision envelope is a plain dict with exactly these keys:
    object_id, revision_id, parent_revision_id (or None), recorded_seq (int),
    recorded_at, actor, origin, payload (a dict, parsed generically from the
    stored/exported payload JSON text -- never a typed Rust struct), kind
    (the payload's own "kind" tag).

Each raw reader is solely responsible for producing this list correctly and
for independently verifying every hash/digest along the way *before* handing
envelopes here.
"""
from collections import defaultdict


class ChainError(Exception):
    pass


def build_report(envelopes: list[dict]) -> dict:
    """Turn a flat list of revision envelopes into a semantic report.

    Raises ChainError on any internal inconsistency (duplicate
    (object_id, revision_id) pair, broken parent_revision_id chain,
    non-monotonic recorded_seq) -- this function never silently drops a
    bad revision.
    """
    by_object: dict[str, list[dict]] = defaultdict(list)
    seen_pairs = set()
    for env in envelopes:
        pair = (env["object_id"], env["revision_id"])
        if pair in seen_pairs:
            raise ChainError(f"duplicate (object_id, revision_id) pair: {pair}")
        seen_pairs.add(pair)
        by_object[env["object_id"]].append(env)

    objects = {}
    revision_count = 0
    record_count_by_kind: dict[str, int] = defaultdict(int)
    relations = []
    action_dependencies = {}
    source_digests = {}
    decision_states = {}
    project_membership: dict[str, set] = defaultdict(set)
    projects = {}

    for object_id, revs in by_object.items():
        revs.sort(key=lambda e: e["recorded_seq"])
        revision_count += len(revs)
        prev_id = None
        prev_seq = None
        for rev in revs:
            if prev_seq is not None and rev["recorded_seq"] <= prev_seq:
                raise ChainError(
                    f"non-monotonic recorded_seq for object {object_id}: "
                    f"{prev_seq} then {rev['recorded_seq']}"
                )
            if rev["parent_revision_id"] != prev_id:
                raise ChainError(
                    f"broken revision chain for object {object_id} at seq "
                    f"{rev['recorded_seq']}: parent_revision_id="
                    f"{rev['parent_revision_id']!r} but previous revision_id="
                    f"{prev_id!r}"
                )
            prev_id = rev["revision_id"]
            prev_seq = rev["recorded_seq"]

        current = revs[-1]
        kind = current["kind"]
        payload = current["payload"]
        record_count_by_kind[kind] += 1
        objects[object_id] = {
            "kind": kind,
            "current_revision_id": current["revision_id"],
            "current_seq": current["recorded_seq"],
            "revision_count": len(revs),
            "current_payload": payload,
        }

        project_id = payload.get("project_id")
        if kind == "project":
            projects[object_id] = payload
        elif project_id is not None:
            project_membership[project_id].add(object_id)

        if kind == "relation":
            relations.append({
                "relation_object_id": object_id,
                "project_id": project_id,
                "relation_type": payload.get("relation_type"),
                "from_object_id": payload.get("from_object_id"),
                "from_revision_id": payload.get("from_revision_id"),
                "to_object_id": payload.get("to_object_id"),
                "to_revision_id": payload.get("to_revision_id"),
            })
        elif kind == "action":
            action_dependencies[object_id] = list(payload.get("dependency_ids") or [])
        elif kind == "source":
            capture = payload.get("capture") or {}
            source_digests[object_id] = {
                "sha256": capture.get("sha256"),
                "byte_length": capture.get("byte_length"),
                "display_filename": capture.get("display_filename"),
            }
        elif kind == "decision":
            decision_states[object_id] = {
                "lifecycle": payload.get("lifecycle"),
                "decision_key": payload.get("decision_key"),
                "statement": payload.get("statement"),
            }

    return {
        "record_count_by_kind": dict(record_count_by_kind),
        "object_count": len(objects),
        "revision_count": revision_count,
        "objects": objects,
        "projects": projects,
        "project_membership": {k: sorted(v) for k, v in project_membership.items()},
        "relations": sorted(relations, key=lambda r: r["relation_object_id"]),
        "action_dependencies": action_dependencies,
        "source_digests": source_digests,
        "decision_states": decision_states,
    }


def project_scoped_subset(full_report: dict, project_id: str) -> dict:
    """The subset of a full-store semantic report that a correct
    project-scoped export of `project_id` must contain: the Project object
    itself, plus every object whose current payload's project_id equals it.
    Independently re-derived here from the full report's own already
    -reconstructed objects -- not from any fixture-supplied "expected" list.
    """
    member_ids = set(full_report["project_membership"].get(project_id, set()))
    member_ids.add(project_id)
    objects = {oid: obj for oid, obj in full_report["objects"].items() if oid in member_ids}
    return build_report_from_objects(objects)


def build_report_from_objects(objects: dict) -> dict:
    """Rebuild the aggregate-fact portion of a report from an already
    -filtered `objects` mapping (used to re-derive the expected shape of a
    project-scoped subset from a full report, without re-parsing anything)."""
    record_count_by_kind: dict[str, int] = defaultdict(int)
    relations = []
    action_dependencies = {}
    source_digests = {}
    decision_states = {}
    projects = {}
    revision_count = 0
    for object_id, obj in objects.items():
        kind = obj["kind"]
        payload = obj["current_payload"]
        record_count_by_kind[kind] += 1
        revision_count += obj["revision_count"]
        if kind == "project":
            projects[object_id] = payload
        elif kind == "relation":
            relations.append({
                "relation_object_id": object_id,
                "project_id": payload.get("project_id"),
                "relation_type": payload.get("relation_type"),
                "from_object_id": payload.get("from_object_id"),
                "from_revision_id": payload.get("from_revision_id"),
                "to_object_id": payload.get("to_object_id"),
                "to_revision_id": payload.get("to_revision_id"),
            })
        elif kind == "action":
            action_dependencies[object_id] = list(payload.get("dependency_ids") or [])
        elif kind == "source":
            capture = payload.get("capture") or {}
            source_digests[object_id] = {
                "sha256": capture.get("sha256"),
                "byte_length": capture.get("byte_length"),
                "display_filename": capture.get("display_filename"),
            }
        elif kind == "decision":
            decision_states[object_id] = {
                "lifecycle": payload.get("lifecycle"),
                "decision_key": payload.get("decision_key"),
                "statement": payload.get("statement"),
            }
    return {
        "record_count_by_kind": dict(record_count_by_kind),
        "object_count": len(objects),
        "revision_count": revision_count,
        "objects": objects,
        "projects": projects,
        "relations": sorted(relations, key=lambda r: r["relation_object_id"]),
        "action_dependencies": action_dependencies,
        "source_digests": source_digests,
        "decision_states": decision_states,
    }
