# Flake canonical plan migration provenance (project renamed to Pluma, 2026-09-20)

Date: 2026-09-14

## Purpose

This record explains how the final Astro plan moved from a local-only planning handoff into the canonical GitHub repository without pretending that historical local-only Git objects existed remotely.

## Recovered source

```text
DOCUMENT=FLAKE_CANONICAL_BUILD_PLAN.md
SOURCE_SIZE_BYTES=217463
SOURCE_SHA256=b555f83ff12882ae6f55f90bbeaa411de52b84661a7f3953350cbfc6bd789fb2
REMOTE_BASELINE=b03b7a8a1507cccef9f7230e17b1c2ff9f08ae84
```

The recovered source was the original conversation upload containing the Astro canonical build plan. Its byte length and SHA-256 were verified before repository integration. A gzip/base64 transport was independently checked during recovery, then discarded; transport fragments are not part of the clean canonical integration.

## Historical local-only identifiers

The plan's snapshot references `4246f6d...` as a preserved local review commit and `852e44b...` as the reported final local planning commit. GitHub did not contain those commits at migration time. This migration does not manufacture remote commits with those identifiers and does not rewrite repository history to imitate them.

Those SHAs and the historical Windows/OneDrive path are provenance references only. No implementation task may require them.

## Missing original collateral

The exact original bytes of the separately reported Astro handoff, architecture-decision annex, corpus-disposition annex, and final security-review file were not recovered from the available conversation/library sources. They are therefore not represented as byte-identical originals.

Repository files created at those referenced paths during this migration are explicitly marked as migration-derived indexes or handoffs. The authoritative substance remains the recovered canonical build plan and the preserved historical repository evidence.

## Canonicality after merge

After this migration lands on `main`, GitHub `main` is the required source for the canonical plan. Local-only planning state is no longer an execution dependency.
