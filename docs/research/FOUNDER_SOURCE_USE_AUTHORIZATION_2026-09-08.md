# Founder Source-Use Authorization — 2026-09-08

**Status:** FOUNDER RIGHTS / SOURCE-USE RECORD — NON-AUTHORIZING FOR CURRENT R1 EXECUTION  
**Recorded:** 2026-09-08  
**Applies to:** Fehrest donor/source corpus already recorded in the repository plus the newly supplied Tencent sources  
**Canonical execution frontier:** live `specs/CURRENT.md`

> This record captures the Founder's explicit statement that Fehrest has permission to copy, modify, port, adapt and reuse source code from the listed donor/source repositories. It removes a project-level source-use uncertainty; it does not activate a future spec, admit a runtime dependency, weaken source provenance, bypass third-party obligations, mutate R1, or authorize product implementation while the current frontier forbids it.

## 1. Founder authorization

The Founder supplied the following direction in the project conversation:

```text
FOUNDER_SOURCE_CODE_USE_AUTHORIZATION=YES
SCOPE=THE_NEW_TENCENT_SOURCES_AND_ALL_SOURCE_REPOSITORIES_ALREADY_RECORDED_FOR_FEHREST
PERMITTED_REUSE_MODES=COPY|MODIFY|PORT|ADAPT|REUSE|TEST_VECTOR|REFERENCE
```

For the three sources in the current convergence pass:

```text
Tencent/RoMem     @ 39ac1417b4db41ea729e5c3be71ac20de54da993
Tencent/WeKnora   @ 647848f3954dae34473b8a8d0e0eef5e0fb3a58e
Tencent/SkillHone @ 7d565839fb4dc74f9c77f09ace660e1c0484e048
```

The same Founder source-use authorization applies to donor/source repositories already recorded by Fehrest, subject to the exact provenance and third-party obligations below.

## 2. What this changes

Previous planning language that blocked reuse solely because a public root license was not observed is superseded by this Founder authorization record for project-level source use.

In particular:

```text
ROMEM_PROJECT_SOURCE_USE_RIGHTS_BASIS=FOUNDER_DIRECT_PERMISSION
ROMEM_CODE_COPY_CANDIDATE=YES
ROMEM_CODE_PORT_CANDIDATE=YES
ROMEM_CODE_ADAPT_CANDIDATE=YES
```

This does **not** claim that RoMem publishes a public license. The observed upstream fact remains:

```text
ROMEM_ROOT_PUBLIC_LICENSE_OBSERVED=NO
```

For a source that also has a public license, record both bases where applicable:

```text
RIGHTS_BASIS=PUBLIC_LICENSE+FOUNDER_DIRECT_PERMISSION
```

## 3. What this does not change

```text
PERMISSION_TO_COPY != REQUIREMENT_TO_COPY
PERMISSION_TO_COPY != DEPENDENCY_ADMISSION
PERMISSION_TO_COPY != IMPLEMENTATION_AUTHORITY
PERMISSION_TO_COPY != CANONICAL_AUTHORITY
PERMISSION_TO_COPY != SECURITY_ACCEPTANCE
PERMISSION_TO_COPY != BENCHMARK_SUCCESS
```

Every actual reuse remains owned by an active requirement/spec and must pass the existing Fehrest engineering method.

## 4. Mandatory provenance for copied or ported work

Every copied, modified, translated or substantively adapted source unit must have a reuse dossier containing at least:

```text
SOURCE_ID
SOURCE_REPOSITORY
SOURCE_COMMIT
SOURCE_PATH
DESTINATION_PATH
REUSE_MODE=COPY|PORT|ADAPT|TEST_VECTOR|REFERENCE
RIGHTS_BASIS=PUBLIC_LICENSE|FOUNDER_DIRECT_PERMISSION|BOTH
UPSTREAM_LICENSE_OBSERVED
THIRD_PARTY_COMPONENTS
COPYRIGHT_NOTICE
ATTRIBUTION_LOCATION
TRANSFORMATION_SUMMARY
OWNER_SPEC
SECURITY_REVIEW
BENCHMARK_DECISION
TEST_EVIDENCE
ADVISORY_SCAN
RUNTIME_DEPENDENCY_CHANGE
CANONICAL_AUTHORITY_CHANGE
```

A future machine-readable form is preferred when source reuse begins at scale.

## 5. Public-distribution and third-party rule

Founder permission is recorded as the project's direct source-use basis. It does not erase separately applicable third-party terms bundled inside a donor repository.

Therefore:

- preserve exact upstream copyright and attribution notices when applicable;
- preserve license/NOTICE obligations for public-license components;
- identify nested third-party code/assets/models/data separately;
- before public distribution of copied material whose upstream has no observed public license, retain durable written evidence of the direct permission in the project evidence set;
- never infer rights over a nested third-party component merely from rights over the parent repository.

## 6. Rust semantic ownership remains unchanged

The Founder source-use authorization does not weaken the Architecture Freeze:

```text
RUST_OWNS_FEHREST_PRODUCT_SEMANTICS=YES
```

For Python/Go/TypeScript donor logic that would otherwise own Fehrest canonical semantics, prefer one of:

```text
1. bounded Rust port with exact provenance;
2. test-vector/reference reuse with Fehrest-owned Rust implementation;
3. typed replaceable provider bridge only where the active spec explicitly authorizes a non-Rust provider boundary.
```

Do not import a donor runtime merely because reuse is permitted.

## 7. Current execution effect

```text
FOUNDER_SOURCE_CODE_USE_AUTHORIZATION=RECORDED
R1_CHANGED=NO
R1_REVIEW_CANDIDATE_CHANGED=NO
SPEC_002_ACTIVATED=NO
PRODUCT_IMPLEMENTATION_AUTHORIZED=NO
DEPENDENCY_ADMISSION=NO
PR_2_MERGE_AUTHORIZED=NO
```

The current R1 scientific/statistical gates remain exactly as they were before this record.