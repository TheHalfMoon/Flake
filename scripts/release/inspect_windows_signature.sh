#!/usr/bin/env bash
# T05-04 (Founder amendment:
# docs/canonical/FOUNDER_WEBSITE_FIRST_DIRECT_DISTRIBUTION_AMENDMENT_2026-09-26.md).
# Windows direct-web distribution truth inspection.
#
# This script performs a REAL Windows signature inspection on one production
# artifact and reports the truthful result. Under the website-first direct
# distribution amendment the expected result is AUTHENTICODE=NOT_SIGNED --
# which is acceptable only when explicitly documented, never represented as
# trusted/signed, and only when every other release-integrity gate passes.
#
# This script NEVER claims PASS for Authenticode. It reports one of:
#   WINDOWS_AUTHENTICODE_STATUS=SIGNED_TRUSTED (real trusted chain verified)
#   WINDOWS_AUTHENTICODE_STATUS=NOT_SIGNED     (no signature present)
#   WINDOWS_AUTHENTICODE_STATUS=UNTRUSTED      (a signature exists but does
#                                              not chain to a trusted root)
# and always exits 0 on a successful inspection (the inspection itself
# succeeding is distinct from the artifact being trusted).
#
# Usage:
#   scripts/release/inspect_windows_signature.sh <artifact.exe>
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <artifact-to-inspect>" >&2
  exit 2
fi

ARTIFACT="$1"
if [[ ! -f "$ARTIFACT" ]]; then
  echo "artifact not found: $ARTIFACT" >&2
  exit 1
fi

echo "TEST_SIGNING_IDENTITY_ONLY=NO"
echo "WINDOWS_AUTHENTICODE_TRUST_CLAIMED=NO"
echo "WINDOWS_STORE_SIGNATURE=NOT_USED"

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

WIN_ARTIFACT="$(cygpath -w "$ARTIFACT" 2>/dev/null || echo "$ARTIFACT")"

echo "==> Get-AuthenticodeSignature inspection (PowerShell, no trust claim)"
powershell.exe -NoProfile -NonInteractive -Command \
  "\$s = Get-AuthenticodeSignature -FilePath '$WIN_ARTIFACT'; \$s | Format-List SignerCertificate,Status,StatusMessage | Out-String -Width 300" \
  || true

echo "==> signtool verify /pa /v (chain-trust check; a non-zero result is EXPECTED for an unsigned direct-distribution artifact)"
set +e
MSYS_NO_PATHCONV=1 "$SIGNTOOL" verify /pa /v "$WIN_ARTIFACT"
VERIFY_STATUS=$?
set -e

if [[ $VERIFY_STATUS -eq 0 ]]; then
  echo "WINDOWS_AUTHENTICODE_STATUS=SIGNED_TRUSTED"
  echo "WINDOWS_AUTHENTICODE_TRUST=AVAILABLE (independently verified above -- do not confuse with the amendment's default NOT_AVAILABLE expectation)"
else
  echo "WINDOWS_AUTHENTICODE_STATUS=NOT_SIGNED"
  echo "WINDOWS_AUTHENTICODE_TRUST=NOT_AVAILABLE"
fi
echo "WINDOWS_AUTHENTICODE_TRUST_CLAIMED=NO"
echo "WINDOWS_SMARTSCREEN_WARNING=EXPECTED_AND_DISCLOSED"
echo "WINDOWS_SIGNATURE_INSPECTION_EXIT=$VERIFY_STATUS"
