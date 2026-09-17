#!/usr/bin/env bash
# T05-04 (plan section 25: "Sign Windows payloads/installer ... using
# owner-controlled credentials without exposing them"). Wraps
# `signtool sign`/`signtool verify` around one artifact.
#
# Production use (owner-controlled credentials present):
#   SIGNING_CERT_PATH=<path to .pfx> SIGNING_CERT_PASSWORD=<secret> \
#   TIMESTAMP_URL=<RFC3161 timestamp service URL> \
#   scripts/release/sign_windows.sh <artifact.exe>
#
# Test-mechanics mode (no production credentials -- proves invocation,
# secret handling and verification wiring only; NEVER a production
# signature -- see docs/evidence/flake-v1/T05-04/SIGNING_PIPELINE_TEST_MECHANICS.md):
#   TEST_SIGNING_MODE=1 scripts/release/sign_windows.sh <artifact.exe>
# In test mode, this script generates a disposable self-signed
# Authenticode test certificate (PowerShell `New-SelfSignedCertificate`),
# imports it into this run's own LocalMachine\Root trust store (so
# `signtool verify /pa` can complete the chain-trust check it is actually
# designed to perform), signs with it, verifies, then removes the
# certificate from the store again. This never touches, requires or
# substitutes for a real Authenticode certificate -- it exercises the
# exact same signtool invocation shape a real certificate would use.
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

find_signtool() {
  if command -v signtool.exe > /dev/null 2>&1; then
    command -v signtool.exe
    return 0
  fi
  local kit_root="/c/Program Files (x86)/Windows Kits/10/bin"
  local found
  found="$(find "$kit_root" -iname "signtool.exe" -path "*x64*" 2>/dev/null | sort -r | head -1)"
  if [[ -n "$found" ]]; then
    echo "$found"
    return 0
  fi
  return 1
}

SIGNTOOL="$(find_signtool || true)"
if [[ -z "$SIGNTOOL" ]]; then
  echo "signtool.exe not found on this host (expected: Windows SDK present on windows-latest CI runners)" >&2
  exit 1
fi
echo "==> using signtool: $SIGNTOOL"

TEST_MODE="${TEST_SIGNING_MODE:-0}"
CLEANUP_THUMBPRINT=""

cleanup() {
  if [[ -n "$CLEANUP_THUMBPRINT" ]]; then
    echo "==> removing disposable test certificate from LocalMachine\\Root and CurrentUser\\My (thumbprint $CLEANUP_THUMBPRINT)"
    powershell.exe -NoProfile -NonInteractive -Command \
      "Get-ChildItem Cert:\\LocalMachine\\Root | Where-Object { \$_.Thumbprint -eq '$CLEANUP_THUMBPRINT' } | Remove-Item -Force -ErrorAction SilentlyContinue; Get-ChildItem Cert:\\CurrentUser\\My | Where-Object { \$_.Thumbprint -eq '$CLEANUP_THUMBPRINT' } | Remove-Item -Force -ErrorAction SilentlyContinue" \
      > /dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

if [[ "$TEST_MODE" == "1" ]]; then
  echo "TEST_SIGNING_IDENTITY_ONLY=YES"
  echo "PRODUCTION_SIGNATURE_CLAIMED=NO"
  echo "==> generating disposable self-signed Authenticode TEST certificate (not a production identity)"

  # Every path handed to powershell.exe is cygpath -w converted first:
  # embedding a raw MSYS/POSIX path (e.g. from `mktemp -d`) inside a
  # PowerShell -Command string is not reliably auto-translated by Git
  # Bash's argv path-mangling (that mangling applies to whole argv
  # elements, not to path-shaped substrings buried inside one larger
  # -Command string), and silently produces a path PowerShell cannot
  # resolve.
  WORKDIR="$(mktemp -d)"
  PFX_PATH="$(cygpath -w "$WORKDIR/test-signing.pfx")"

  # Thumbprint comes back over this call's own stdout rather than a
  # round-tripped file -- one fewer path to get wrong, and the only
  # PowerShell output on this path is the final Write-Output line (every
  # earlier cmdlet result is assigned to a variable or piped to
  # Out-Null).
  THUMBPRINT="$(powershell.exe -NoProfile -NonInteractive -Command "
    \$pw = ConvertTo-SecureString -String 'test-only-disposable-password' -Force -AsPlainText
    \$cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject 'CN=Flake TEST Signing Identity - NOT PRODUCTION - disposable' -KeyUsage DigitalSignature -FriendlyName 'flake-ci-disposable-test-cert' -CertStoreLocation Cert:\\CurrentUser\\My -NotAfter (Get-Date).AddDays(1)
    Export-PfxCertificate -Cert \$cert -FilePath '$PFX_PATH' -Password \$pw | Out-Null
    Write-Output \$cert.Thumbprint
  " | tr -d '\r\n')"

  if [[ -z "$THUMBPRINT" ]]; then
    echo "failed to generate disposable test certificate (empty thumbprint)" >&2
    exit 1
  fi

  # Re-import from the exported PFX into LocalMachine\Root, not
  # CurrentUser\Root: confirmed live that signtool's own /pa chain-trust
  # check ("A certificate chain processed, but terminated in a root
  # which is not trusted") does not treat a CurrentUser\Root addition as
  # sufficient on this runner -- Authenticode policy chain-building
  # consults the machine-wide root store as its trust anchor, not the
  # per-user one. GitHub-hosted Windows runners execute job steps with
  # local administrator rights, so this write does not require an
  # explicit elevation prompt.
  powershell.exe -NoProfile -NonInteractive -Command "
    \$pw = ConvertTo-SecureString -String 'test-only-disposable-password' -Force -AsPlainText
    Import-PfxCertificate -FilePath '$PFX_PATH' -CertStoreLocation Cert:\\LocalMachine\\Root -Password \$pw -ErrorAction SilentlyContinue | Out-Null
  " > /dev/null 2>&1 || true

  CLEANUP_THUMBPRINT="$THUMBPRINT"
  SIGNING_CERT_PATH="$PFX_PATH"
  SIGNING_CERT_PASSWORD="test-only-disposable-password"
  TIMESTAMP_URL="${TIMESTAMP_URL:-http://timestamp.digicert.com}"

  echo "==> disposable TEST certificate thumbprint: $THUMBPRINT (self-signed, added only to this ephemeral runner's own LocalMachine\\Root store -- not a publicly trusted CA, not a production identity)"
else
  : "${SIGNING_CERT_PATH:?SIGNING_CERT_PATH must be set (production mode)}"
  : "${SIGNING_CERT_PASSWORD:?SIGNING_CERT_PASSWORD must be set (production mode)}"
  : "${TIMESTAMP_URL:?TIMESTAMP_URL must be set (production mode)}"
  echo "TEST_SIGNING_IDENTITY_ONLY=NO"
fi

WIN_ARTIFACT="$(cygpath -w "$ARTIFACT" 2>/dev/null || echo "$ARTIFACT")"
WIN_CERT="$(cygpath -w "$SIGNING_CERT_PATH" 2>/dev/null || echo "$SIGNING_CERT_PATH")"

echo "==> signing $ARTIFACT (password redacted from this log)"
# MSYS_NO_PATHCONV=1: Git Bash auto-translates any bare `/word`-shaped
# argument that looks like a POSIX absolute path into a Windows path
# before a native (non-MSYS) executable ever sees it -- the exact same
# class of bug scripts/release/install_test.sh already hit and
# documented for NSIS's own bare `/S` flag (silently rewritten to
# `S:/`). signtool's own `/fd`/`/f`/`/p`/`/tr`/`/td` flags are just as
# vulnerable; confirmed live here by signtool itself reporting `/fd` as
# entirely missing despite it being present, correctly ordered, on the
# command line. Disabling Git Bash's path conversion for this one
# invocation is the standard fix, not a per-flag escape.
MSYS_NO_PATHCONV=1 "$SIGNTOOL" sign /f "$WIN_CERT" /p "$SIGNING_CERT_PASSWORD" /fd sha256 /tr "$TIMESTAMP_URL" /td sha256 "$WIN_ARTIFACT" \
  > >(sed "s/$SIGNING_CERT_PASSWORD/[REDACTED]/g") 2> >(sed "s/$SIGNING_CERT_PASSWORD/[REDACTED]/g" >&2)

echo "==> verifying signature (chain-trust check via signtool verify /pa)"
set +e
MSYS_NO_PATHCONV=1 "$SIGNTOOL" verify /pa /v "$WIN_ARTIFACT"
VERIFY_STATUS=$?
set -e

if [[ "$TEST_MODE" == "1" ]]; then
  if [[ $VERIFY_STATUS -eq 0 ]]; then
    echo "SIGNATURE_MECHANICS_VERIFIED=YES (disposable self-signed TEST certificate, trusted only inside this ephemeral CI runner's own store)"
  else
    echo "SIGNATURE_MECHANICS_VERIFIED=NO (signtool verify failed even for the disposable test certificate -- investigate before reusing this wrapper)" >&2
  fi
  echo "TEST_SIGNING_IDENTITY_ONLY=YES"
  echo "PRODUCTION_SIGNATURE_CLAIMED=NO"
  exit $VERIFY_STATUS
else
  exit $VERIFY_STATUS
fi
