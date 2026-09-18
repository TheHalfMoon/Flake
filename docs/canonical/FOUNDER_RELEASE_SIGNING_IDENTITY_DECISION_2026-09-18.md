# Founder decision — Flake Linux release-signing identity (2026-09-18)

**Context:** `T05-04`'s remaining acceptance clause ("all signatures/notarization/stapling
verify") is blocked on three owner-controlled credential resources
(`docs/evidence/flake-v1/T05-04/REPORT.md`'s `BLOCKER_ID=T05-04-SIGNING-CREDENTIALS-UNAVAILABLE`).
This decision resolves one prerequisite for the Linux third: the exact identity string bound
to the production release-signing GPG key, which must be decided once and then treated as
permanent (rotation is a separate, documented procedure — `docs/release/LINUX_RELEASE_SIGNING.md`
— not a routine choice).

**Options considered**, both live/verifiable and neither invented:

1. `285091250+TheHalfMoon@users.noreply.github.com` — the GitHub-issued noreply alias for the
   `TheHalfMoon` account that owns this repository (confirmed live: `gh api users/TheHalfMoon`
   returns numeric id `285091250`). Account/project-scoped rather than personal.
2. `alshehriofficial@gmail.com` — the Founder's personal email, already the public GitHub
   profile email for `TheHalfMoon` and the git author/committer email used throughout this
   repository's history (`docs/canonical/GITHUB_BOOTSTRAP_PROVENANCE.md`,
   `HISTORICAL_GOVERNANCE_RECONCILIATION.md`).

**Founder ruling:** option 1. The canonical production Linux release-signing GPG identity is:

```text
Flake Release Signing <285091250+TheHalfMoon@users.noreply.github.com>
```

Rationale recorded by the Founder: prefer the GitHub noreply identity over a personal email;
do not expose the personal email unnecessarily. This is the canonical identity unless a future
explicit Founder decision replaces it (a rotation, not a routine edit — see
`docs/release/LINUX_RELEASE_SIGNING.md`'s rotation/revocation procedure).

**Binding requirements carried forward from this decision:**

- The resulting public key fingerprint must be recorded in release documentation
  (`docs/release/LINUX_RELEASE_SIGNING.md`, `docs/release/RELEASE_VERIFICATION.md`).
- The private key and any passphrase must never be committed to the repository or printed in
  a CI log.
- Only the public key and its fingerprint are published; verification instructions must be
  reproducible by a third party from published material alone.
- A revocation certificate and a rotation procedure must exist before the key is put into
  production use.

This decision does not itself create the key. Key generation is a one-time Founder-run local
procedure (`docs/release/LINUX_RELEASE_SIGNING.md` § "One-time key generation") — generating
real production private key material inside an agent transcript would risk exposing it through
session logs, which this decision explicitly forbids.
