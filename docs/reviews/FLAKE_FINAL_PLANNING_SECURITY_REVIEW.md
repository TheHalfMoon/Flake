# Flake final planning security review — migration-derived index

`MIGRATION_DERIVED=YES`
`ORIGINAL_ASTRO_BYTES=NOT_RECOVERED`

The original separately reported Astro security-review bytes were not recovered. This file exists to make the canonical plan's reference explicit and non-misleading.

## Authoritative security contract

Section 20 of `docs/canonical/FLAKE_CANONICAL_BUILD_PLAN.md`, together with the task-level trust, disclosure, durability, provenance, recovery, dependency-admission, and negative-test gates, is the authoritative v1 security contract.

Historical security and trust reviews remain evidence. They cannot silently override the final plan, mint authority from content, weaken local ownership, or convert derived state into canonical authority.

Implementation must preserve default-deny disclosure, bounded agent authority, exact provenance, fail-closed incompatible transitions, honest recovery status, dependency/rights admission, secret exclusion, and evidence-backed security claims. A Class D change requires the dedicated adversarial/security review required by the plan.
