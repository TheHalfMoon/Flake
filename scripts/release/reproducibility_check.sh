#!/usr/bin/env bash
# T05-03 (plan section 25: "Reproducibility requires two clean builds
# with pinned compiler/dependencies/assets: unsigned payloads match
# bit-for-bit or each irreducible toolchain difference is isolated,
# documented and independently shown not to affect code/content.").
#
# Builds the five shipped CLI binaries twice from a genuinely clean
# `target/` (not an incremental rebuild), then compares checksums.
# `pluma`, `flake` and `fehrest` are expected to differ from each other
# (separate `[[bin]]` targets embed their own target-name/build-path debug
# metadata -- see tests/cli_alias_parity.rs for the functional parity
# proof instead), but each binary must match *itself* byte-for-byte
# across the two clean builds, or this script reports exactly which
# bytes differ and fails rather than asserting reproducibility it did not
# verify.
set -euo pipefail

cd "$(dirname "$0")/../.."
REPO_ROOT="$(pwd)"
OUT_DIR="${DIST_DIR:-$REPO_ROOT/dist}/reproducibility"
mkdir -p "$OUT_DIR"

BINS=(pluma flake fehrest pluma-migrate flake-migrate)
EXE=""
case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*) EXE=".exe" ;;
esac

build_once() {
  local label="$1"
  echo "==> clean build #$label"
  rm -rf target
  cargo build --release --locked --bin pluma --bin flake --bin fehrest --bin pluma-migrate --bin flake-migrate
  mkdir -p "$OUT_DIR/build-$label"
  for bin in "${BINS[@]}"; do
    cp "target/release/${bin}${EXE}" "$OUT_DIR/build-$label/${bin}${EXE}"
  done
}

build_once 1
build_once 2

FAILED=0
for bin in "${BINS[@]}"; do
  a="$OUT_DIR/build-1/${bin}${EXE}"
  b="$OUT_DIR/build-2/${bin}${EXE}"
  if command -v sha256sum > /dev/null; then
    hash_a="$(sha256sum "$a" | cut -d' ' -f1)"
    hash_b="$(sha256sum "$b" | cut -d' ' -f1)"
  else
    hash_a="$(shasum -a 256 "$a" | cut -d' ' -f1)"
    hash_b="$(shasum -a 256 "$b" | cut -d' ' -f1)"
  fi
  if [[ "$hash_a" == "$hash_b" ]]; then
    echo "REPRODUCIBLE  $bin  $hash_a"
  else
    echo "DIVERGENT     $bin  build1=$hash_a build2=$hash_b"
    FAILED=1
  fi
done

REPORT="$OUT_DIR/reproducibility-result.txt"
{
  echo "platform: $(uname -s)"
  echo "date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  for bin in "${BINS[@]}"; do
    a="$OUT_DIR/build-1/${bin}${EXE}"
    b="$OUT_DIR/build-2/${bin}${EXE}"
    if cmp -s "$a" "$b"; then
      echo "$bin: BIT_IDENTICAL"
    else
      echo "$bin: DIVERGENT (see below for byte-offset diff)"
      cmp "$a" "$b" || true
    fi
  done
} > "$REPORT"
echo "==> result written to $REPORT"

exit $FAILED
