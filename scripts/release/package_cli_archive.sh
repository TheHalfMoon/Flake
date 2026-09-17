#!/usr/bin/env bash
# T05-03: builds the section-25 CLI release archive for whichever platform
# this script runs on (Windows/macOS/Linux, driven per-OS by the CI
# matrix in .github/workflows/t05-03-release-candidates.yml). Produces
# one archive containing the `flake`/`fehrest`/`flake-migrate` binaries
# plus docs/release/USER_GUIDE.md, and a SHA-256 manifest alongside it.
#
# Labeled UNSIGNED_DEVELOPER_RC throughout: signing is T05-04, not this
# task (plan section 25's own acceptance criteria for T05-03 is scoped to
# "every unsigned candidate installs/runs/updates/uninstalls").
set -euo pipefail

cd "$(dirname "$0")/../.."
REPO_ROOT="$(pwd)"
DIST_DIR="${DIST_DIR:-$REPO_ROOT/dist}"
mkdir -p "$DIST_DIR"

VERSION="$(cargo metadata --format-version=1 --no-deps 2>/dev/null \
  | python3 -c 'import json,sys; d=json.load(sys.stdin); print([p["version"] for p in d["packages"] if p["name"]=="fehrest"][0])')"

case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*) PLATFORM="windows-x86_64"; EXE=".exe"; ARCHIVE_KIND="zip" ;;
  Darwin) PLATFORM="macos-aarch64"; EXE=""; ARCHIVE_KIND="tar.gz" ;;
  Linux) PLATFORM="linux-x86_64"; EXE=""; ARCHIVE_KIND="tar.gz" ;;
  *) echo "unrecognized platform: $(uname -s)" >&2; exit 1 ;;
esac

echo "==> building release binaries (version=$VERSION platform=$PLATFORM)"
cargo build --release --locked --bin flake --bin fehrest --bin flake-migrate

STAGE_NAME="flake-${VERSION}-${PLATFORM}"
STAGE_DIR="$DIST_DIR/$STAGE_NAME"
rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR"

for bin in flake fehrest flake-migrate; do
  src="target/release/${bin}${EXE}"
  if [[ ! -f "$src" ]]; then
    echo "expected built binary missing: $src" >&2
    exit 1
  fi
  cp "$src" "$STAGE_DIR/"
done

cp docs/release/USER_GUIDE.md "$STAGE_DIR/README.md"

# T05-04: "About/help/distribution include license, source, privacy and
# support/reporting route" -- the distributed archive itself must carry the
# project license and third-party notices, not only the source repository.
for legal in LICENSE NOTICE; do
  if [[ ! -f "$legal" ]]; then
    echo "expected $legal missing at repository root" >&2
    exit 1
  fi
  cp "$legal" "$STAGE_DIR/"
done
mkdir -p "$STAGE_DIR/docs/legal"
cp docs/legal/THIRD-PARTY-LICENSES.md "$STAGE_DIR/docs/legal/"

# Every command below is a plain smoke check that the archive's own copies
# of the binaries actually run on this platform -- not merely that they
# compiled -- before anything is zipped up and reported as a candidate.
echo "==> smoke-checking staged binaries"
"$STAGE_DIR/flake${EXE}" --help > /dev/null
"$STAGE_DIR/fehrest${EXE}" --help > /dev/null
"$STAGE_DIR/flake-migrate${EXE}" --help > /dev/null

ARCHIVE_BASENAME="$STAGE_NAME"
pushd "$DIST_DIR" > /dev/null
if [[ "$ARCHIVE_KIND" == "zip" ]]; then
  ARCHIVE_FILE="${ARCHIVE_BASENAME}.zip"
  rm -f "$ARCHIVE_FILE"
  if command -v 7z > /dev/null; then
    7z a -tzip "$ARCHIVE_FILE" "$STAGE_NAME" > /dev/null
  else
    python3 -c "
import shutil
shutil.make_archive('$ARCHIVE_BASENAME', 'zip', '.', '$STAGE_NAME')
"
  fi
else
  ARCHIVE_FILE="${ARCHIVE_BASENAME}.tar.gz"
  rm -f "$ARCHIVE_FILE"
  tar -czf "$ARCHIVE_FILE" "$STAGE_NAME"
fi
popd > /dev/null

MANIFEST_FILE="$DIST_DIR/${ARCHIVE_BASENAME}.sha256"
pushd "$DIST_DIR" > /dev/null
if command -v sha256sum > /dev/null; then
  sha256sum "$ARCHIVE_FILE" > "$(basename "$MANIFEST_FILE")"
else
  shasum -a 256 "$ARCHIVE_FILE" > "$(basename "$MANIFEST_FILE")"
fi
popd > /dev/null

echo "==> archive:  $DIST_DIR/$ARCHIVE_FILE"
echo "==> manifest: $MANIFEST_FILE"
cat "$MANIFEST_FILE"
