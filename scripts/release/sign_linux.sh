#!/usr/bin/env bash
# T05-04 (plan section 25: "Linux/checksum manifests ... using
# owner-controlled credentials without exposing them"). Wraps
# `gpg --detach-sign` / `gpg --verify` around one artifact (a .deb
# package or a .sha256 checksum manifest).
#
# Production use has two supported input shapes (see
# docs/release/LINUX_RELEASE_SIGNING.md for the full architecture):
#   (a) GPG_KEY_ID=<id> -- a key already present in this environment's own
#       keyring.
#   (b) GPG_PRIVATE_KEY=<armored key> [GPG_KEY_PASSPHRASE=<secret>]
#       [GPG_KEY_FINGERPRINT=<expected fingerprint>] -- the key is
#       imported into a fresh, ephemeral GNUPGHOME created for this one
#       invocation (never this environment's persistent keyring) and
#       destroyed on exit. If GPG_KEY_FINGERPRINT is set, the script
#       refuses to sign unless the imported key's own fingerprint matches
#       exactly, so a corrupted or substituted secret can never silently
#       sign under the wrong identity. Key material and passphrase are
#       never echoed, never passed as a bare argv value, and never left
#       on disk outside that ephemeral, deleted directory.
#   scripts/release/sign_linux.sh <artifact>
#
# Test-mechanics mode (no production release-signing key -- proves gpg
# invocation, detached-signature production and verification wiring
# only; NEVER a production signature -- see
# docs/evidence/flake-v1/T05-04/SIGNING_PIPELINE_TEST_MECHANICS.md):
#   TEST_SIGNING_MODE=1 scripts/release/sign_linux.sh <artifact>
# In test mode, this script either generates a disposable ephemeral GPG
# key in a throwaway GNUPGHOME (default), or -- if GPG_PRIVATE_KEY is
# also set -- imports that (still disposable/throwaway) key through the
# exact same import_key_from_secret path production mode uses, to prove
# that specific mechanics without ever touching a real production
# secret. Either way, test mode signs and verifies against that same
# throwaway keyring, then deletes the whole throwaway directory, and
# never claims a production signature.
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
    echo "==> destroying ephemeral GNUPGHOME"
    rm -rf "$DISPOSABLE_GNUPGHOME"
  fi
}
trap cleanup EXIT

# Imports an armored key (passed as $1, never as a filename -- callers
# pass the secret's own value, e.g. from $GPG_PRIVATE_KEY) into a fresh
# ephemeral GNUPGHOME and prints only the derived key id on stdout.
# Every diagnostic line goes to stderr so command substitution
# (`GPG_KEY_ID="$(import_key_from_secret "$GPG_PRIVATE_KEY")"`) captures
# exactly the id and nothing else. Optionally pins the imported key's
# fingerprint against $GPG_KEY_FINGERPRINT if that variable is set.
import_key_from_secret() {
  local armored_key="$1"
  DISPOSABLE_GNUPGHOME="$(mktemp -d)"
  chmod 700 "$DISPOSABLE_GNUPGHOME"
  export GNUPGHOME="$DISPOSABLE_GNUPGHOME"

  if ! printf '%s\n' "$armored_key" | gpg --batch --import >/tmp/gpg-import.$$.log 2>&1; then
    echo "gpg import failed (key material never logged):" >&2
    cat /tmp/gpg-import.$$.log >&2
    rm -f /tmp/gpg-import.$$.log
    exit 1
  fi
  rm -f /tmp/gpg-import.$$.log

  local imported_id imported_fpr
  imported_id="$(gpg --batch --list-secret-keys --with-colons | awk -F: '/^sec:/ {print $5; exit}')"
  if [[ -z "$imported_id" ]]; then
    echo "no secret key found after import" >&2
    exit 1
  fi

  if [[ -n "${GPG_KEY_FINGERPRINT:-}" ]]; then
    imported_fpr="$(gpg --batch --list-secret-keys --with-colons | awk -F: '/^fpr:/ {print $10; exit}')"
    if [[ "$imported_fpr" != "$GPG_KEY_FINGERPRINT" ]]; then
      echo "imported key fingerprint ($imported_fpr) does not match expected GPG_KEY_FINGERPRINT ($GPG_KEY_FINGERPRINT) -- refusing to sign with an unexpected key" >&2
      exit 1
    fi
    echo "==> fingerprint pinned and confirmed: $imported_fpr" >&2
  fi

  echo "$imported_id"
}

if [[ "$TEST_MODE" == "1" ]]; then
  echo "TEST_SIGNING_IDENTITY_ONLY=YES"
  echo "PRODUCTION_SIGNATURE_CLAIMED=NO"

  if [[ -n "${GPG_PRIVATE_KEY:-}" ]]; then
    echo "==> importing a disposable TEST key via the same secret-injection mechanics production mode uses (GPG_PRIVATE_KEY), proving that code path -- not a production key"
    GPG_KEY_ID="$(import_key_from_secret "$GPG_PRIVATE_KEY")"
    echo "==> imported disposable TEST key id: $GPG_KEY_ID (ephemeral GNUPGHOME, destroyed at the end of this script)"
  else
    echo "==> generating disposable ephemeral GPG TEST key in a throwaway keyring (not the production release-signing key)"
    DISPOSABLE_GNUPGHOME="$(mktemp -d)"
    chmod 700 "$DISPOSABLE_GNUPGHOME"
    export GNUPGHOME="$DISPOSABLE_GNUPGHOME"

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
  fi
else
  if [[ -n "${GPG_PRIVATE_KEY:-}" ]]; then
    echo "==> importing the production release-signing key from GPG_PRIVATE_KEY into an ephemeral GNUPGHOME (never this environment's persistent keyring; key material never logged)"
    GPG_KEY_ID="$(import_key_from_secret "$GPG_PRIVATE_KEY")"
  else
    : "${GPG_KEY_ID:?GPG_KEY_ID or GPG_PRIVATE_KEY must be set (production mode)}"
  fi
  echo "TEST_SIGNING_IDENTITY_ONLY=NO"
fi

SIG_PATH="${ARTIFACT}.asc"
rm -f "$SIG_PATH"

echo "==> gpg --local-user $GPG_KEY_ID --detach-sign --armor $ARTIFACT"
set +e
if [[ -n "${GPG_KEY_PASSPHRASE:-}" ]]; then
  printf '%s' "$GPG_KEY_PASSPHRASE" | gpg --batch --yes --pinentry-mode loopback --passphrase-fd 0 \
    --local-user "$GPG_KEY_ID" --detach-sign --armor --output "$SIG_PATH" "$ARTIFACT"
else
  gpg --batch --yes --local-user "$GPG_KEY_ID" --detach-sign --armor --output "$SIG_PATH" "$ARTIFACT"
fi
SIGN_STATUS=$?
set -e
if [[ $SIGN_STATUS -ne 0 ]]; then
  echo "gpg detached-sign failed" >&2
  exit $SIGN_STATUS
fi

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
