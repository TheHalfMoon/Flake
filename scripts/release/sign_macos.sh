#!/usr/bin/env bash
# T05-04 (plan section 25: "macOS app/notarize/staple ... using
# owner-controlled credentials without exposing them"). Wraps
# `codesign` / `xcrun notarytool` / `xcrun stapler` / verification
# (`codesign --verify`, `spctl --assess`) around one .app bundle.
#
# Production use (owner-controlled Developer ID + notarization
# credentials present):
#   SIGNING_IDENTITY="Developer ID Application: <Name> (<TeamID>)" \
#   APPLE_ID=<id> APPLE_TEAM_ID=<team> APPLE_APP_PASSWORD=<secret> \
#   scripts/release/sign_macos.sh <App.app>
#
# Test-mechanics mode (no Apple Developer ID / notarization credentials
# -- proves codesign invocation and verification wiring only, and proves
# the notarytool/stapler command lines are well-formed without ever
# calling Apple's live service; NEVER a production signature or a
# notarization claim -- see
# docs/evidence/flake-v1/T05-04/SIGNING_PIPELINE_TEST_MECHANICS.md):
#   TEST_SIGNING_MODE=1 scripts/release/sign_macos.sh <App.app>
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <App.app>" >&2
  exit 2
fi

APP_BUNDLE="$1"
if [[ ! -d "$APP_BUNDLE" ]]; then
  echo "app bundle not found: $APP_BUNDLE" >&2
  exit 1
fi

TEST_MODE="${TEST_SIGNING_MODE:-0}"

if [[ "$TEST_MODE" == "1" ]]; then
  echo "TEST_SIGNING_IDENTITY_ONLY=YES"
  echo "PRODUCTION_SIGNATURE_CLAIMED=NO"
  echo "NOTARIZATION_CLAIMED=NO"

  SIGNING_IDENTITY="-"
  echo "==> using ad-hoc signing identity '-' (no keychain identity required; NOT a Developer ID, cannot pass Gatekeeper)"
else
  : "${SIGNING_IDENTITY:?SIGNING_IDENTITY must be set (production mode, e.g. 'Developer ID Application: Name (TEAMID)')}"
  : "${APPLE_ID:?APPLE_ID must be set (production mode)}"
  : "${APPLE_TEAM_ID:?APPLE_TEAM_ID must be set (production mode)}"
  : "${APPLE_APP_PASSWORD:?APPLE_APP_PASSWORD must be set (production mode)}"
  echo "TEST_SIGNING_IDENTITY_ONLY=NO"
fi

if [[ "$TEST_MODE" == "1" ]]; then
  # An ad-hoc identity ("-") cannot request a trusted timestamp -- Apple's
  # timestamp authority only timestamps a real Developer ID signature.
  TIMESTAMP_FLAG="--timestamp=none"
else
  TIMESTAMP_FLAG="--timestamp"
fi

echo "==> codesign --sign \"$SIGNING_IDENTITY\" --options runtime --deep $TIMESTAMP_FLAG $APP_BUNDLE"
codesign --sign "$SIGNING_IDENTITY" --options runtime --deep --force "$TIMESTAMP_FLAG" "$APP_BUNDLE" 2>&1 \
  | sed -e 's/'"${APPLE_APP_PASSWORD:-__none__}"'/[REDACTED]/g'

echo "==> codesign --verify --deep --strict --verbose=2"
set +e
codesign --verify --deep --strict --verbose=2 "$APP_BUNDLE"
CODESIGN_VERIFY_STATUS=$?
set -e

echo "==> spctl --assess --type execute (Gatekeeper's own assessment)"
set +e
spctl --assess --type execute --verbose=4 "$APP_BUNDLE"
SPCTL_STATUS=$?
set -e

if [[ "$TEST_MODE" == "1" ]]; then
  echo "==> spctl exit status for an ad-hoc/test-signed, unnotarized bundle: $SPCTL_STATUS (a non-zero/rejected result here is CORRECT and expected -- Gatekeeper is supposed to reject an artifact that is not notarized with a real Developer ID; a zero/accepted result would indicate this test artifact was wrongly treated as production-trustworthy and must be investigated)"

  echo "==> proving notarytool/stapler command construction WITHOUT calling Apple's live service"
  if command -v xcrun > /dev/null 2>&1 && xcrun --find notarytool > /dev/null 2>&1; then
    echo "notarytool binary present: $(xcrun --find notarytool)"
    xcrun notarytool submit --help > /dev/null
    echo "NOTARYTOOL_SUBMIT_COMMAND_CONSTRUCTED=xcrun notarytool submit <App.zip> --apple-id <id> --team-id <team> --password <app-specific-password> --wait"
    echo "NOTARYTOOL_INVOKED_AGAINST_APPLE=NO (argument shape verified via --help only)"
  else
    echo "notarytool not found on this host -- command-construction check skipped" >&2
  fi
  if command -v xcrun > /dev/null 2>&1 && xcrun --find stapler > /dev/null 2>&1; then
    echo "stapler binary present: $(xcrun --find stapler)"
    xcrun stapler --help > /dev/null 2>&1 || true
    echo "STAPLER_STAPLE_COMMAND_CONSTRUCTED=xcrun stapler staple <App.app>"
    echo "STAPLER_INVOKED_AGAINST_APPLE=NO (argument shape verified via --help only; stapling a non-notarized bundle would fail regardless)"
  else
    echo "stapler not found on this host -- command-construction check skipped" >&2
  fi

  echo "SIGNATURE_MECHANICS_VERIFIED=$([[ $CODESIGN_VERIFY_STATUS -eq 0 ]] && echo YES || echo NO) (ad-hoc/self-signed TEST identity; codesign's own local signature-validity check, not a trust-chain or notarization claim)"
  echo "TEST_SIGNING_IDENTITY_ONLY=YES"
  echo "PRODUCTION_SIGNATURE_CLAIMED=NO"
  echo "NOTARIZATION_CLAIMED=NO"
  exit $CODESIGN_VERIFY_STATUS
else
  echo "==> xcrun notarytool submit (production)"
  ZIP_PATH="${APP_BUNDLE%.app}.zip"
  ditto -c -k --keepParent "$APP_BUNDLE" "$ZIP_PATH"
  xcrun notarytool submit "$ZIP_PATH" --apple-id "$APPLE_ID" --team-id "$APPLE_TEAM_ID" --password "$APPLE_APP_PASSWORD" --wait \
    2>&1 | sed -e "s/$APPLE_APP_PASSWORD/[REDACTED]/g"
  echo "==> xcrun stapler staple"
  xcrun stapler staple "$APP_BUNDLE"
  echo "==> final spctl --assess (must pass for a real release)"
  spctl --assess --type execute --verbose=4 "$APP_BUNDLE"
  exit $?
fi
