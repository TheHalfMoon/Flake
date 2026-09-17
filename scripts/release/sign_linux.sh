#!/usr/bin/env bash
# T05-04 (plan section 25: "Linux/checksum manifests ... using
# owner-controlled credentials without exposing them"). Wraps
# `gpg --detach-sign` / `gpg --verify` around one artifact (a .deb
# package or a .sha256 checksum manifest).
#
# Production use (owner-controlled release-signing GPG key present,
# already imported into the signing environment's keyring):
#   GPG_KEY_ID=<fingerprint or key id> scripts/release/sign_linux.sh <artifact>
#
# Test-mechanics mode (no production release-signing key -- proves gpg
# invocation, detached-signature production and verification wiring
# only; NEVER a production signature -- see
# docs/evidence/flake-v1/T05-04/SIGNING_PIPELINE_TEST_MECHANICS.md):
#   TEST_SIGNING_MODE=1 scripts/release/sign_linux.sh <artifact>
# In test mode, this script generates a disposable ephemeral GPG key in
# a throwaway GNUPGHOME (never the real user/CI keyring), signs with it,
# verifies against that same throwaway keyring, then deletes the whole
# throwaway directory. This never touches, requires or substitutes for
# a real release-signing key.
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <artifact-to-sign>" >&2
  exit 2
fi

ARTIFACT="$1"
if [[ ! -f "$ARTIFACT" ]]; then
  echo "artifact not found: $ARTIFACT" >&2
  exit 1
fi

TEST_MODE="${TEST_SIGNING_MODE:-0}"
DISPOSABLE_GNUPGHOME=""

cleanup() {
  if [[ -n "$DISPOSABLE_GNUPGHOME" && -d "$DISPOSABLE_GNUPGHOME" ]]; then
    echo "==> destroying disposable GNUPGHOME"
    rm -rf "$DISPOSABLE_GNUPGHOME"
  fi
}
trap cleanup EXIT

if [[ "$TEST_MODE" == "1" ]]; then
  echo "TEST_SIGNING_IDENTITY_ONLY=YES"
  echo "PRODUCTION_SIGNATURE_CLAIMED=NO"

  DISPOSABLE_GNUPGHOME="$(mktemp -d)"
  chmod 700 "$DISPOSABLE_GNUPGHOME"
  export GNUPGHOME="$DISPOSABLE_GNUPGHOME"

  echo "==> generating disposable ephemeral GPG TEST key in a throwaway keyring (not the production release-signing key)"
  gpg --batch --gen-key <<EOF
%no-protection
Key-Type: EDDSA
Key-Curve: ed25519
Subkey-Type: ECDH
Subkey-Curve: cv25519
Name-Real: Flake TEST Signing Key - NOT PRODUCTION - disposable
Name-Email: ci-disposable-test@invalid.example
Expire-Date: 1d
%commit
EOF

  GPG_KEY_ID="$(gpg --batch --list-secret-keys --with-colons | awk -F: '/^sec:/ {print $5; exit}')"
  echo "==> disposable TEST key id: $GPG_KEY_ID (ephemeral, throwaway keyring only, destroyed at the end of this script)"
else
  : "${GPG_KEY_ID:?GPG_KEY_ID must be set (production mode)}"
  echo "TEST_SIGNING_IDENTITY_ONLY=NO"
fi

SIG_PATH="${ARTIFACT}.asc"
rm -f "$SIG_PATH"

echo "==> gpg --local-user $GPG_KEY_ID --detach-sign --armor $ARTIFACT"
gpg --batch --yes --local-user "$GPG_KEY_ID" --detach-sign --armor --output "$SIG_PATH" "$ARTIFACT"

echo "==> gpg --verify $SIG_PATH $ARTIFACT"
set +e
gpg --batch --verify "$SIG_PATH" "$ARTIFACT"
VERIFY_STATUS=$?
set -e

if [[ "$TEST_MODE" == "1" ]]; then
  if [[ $VERIFY_STATUS -eq 0 ]]; then
    echo "SIGNATURE_MECHANICS_VERIFIED=YES (disposable ephemeral TEST key, throwaway keyring only)"
  else
    echo "SIGNATURE_MECHANICS_VERIFIED=NO (gpg verify failed even for the disposable test key -- investigate before reusing this wrapper)" >&2
  fi
  echo "TEST_SIGNING_IDENTITY_ONLY=YES"
  echo "PRODUCTION_SIGNATURE_CLAIMED=NO"
  echo "==> signature file produced at $SIG_PATH is disposable-test-key-signed; deleting it (it must never be shipped as if it were a real release signature)"
  rm -f "$SIG_PATH"
  exit $VERIFY_STATUS
else
  echo "==> signature file (keep, ship alongside the artifact): $SIG_PATH"
  exit $VERIFY_STATUS
fi
