#!/usr/bin/env bash
# T05-03 (plan section 25: "Release set: ... SHA-256 manifest, signatures,
# SBOM and notices"). Generates a CycloneDX SBOM for the shipped release
# binaries (pluma/flake/fehrest/pluma-migrate/flake-migrate) plus the
# desktop shell, then
# copies just the SBOMs for artifacts this release actually ships --
# `cargo cyclonedx --describe binaries` emits one file per `[[bin]]`
# target in the workspace (including internal-only benchmark/fault-test
# harness binaries that are never packaged), named `<bin>_bin.cdx.json`
# at the crate root; this script keeps only the ones that matter and
# discards the rest rather than shipping SBOMs for things not in the
# release archive.
set -euo pipefail

cd "$(dirname "$0")/../.."
REPO_ROOT="$(pwd)"
DIST_DIR="${DIST_DIR:-$REPO_ROOT/dist}"
mkdir -p "$DIST_DIR/sbom"

if ! command -v cargo-cyclonedx > /dev/null; then
  echo "==> installing cargo-cyclonedx"
  cargo install cargo-cyclonedx --locked
fi

collect_sbom() {
  local crate_dir="$1"
  local wanted_bin="$2"
  local dest_name="$3"
  (
    cd "$crate_dir"
    rm -f -- *_bin.cdx.json
    cargo cyclonedx --format json --describe binaries --spec-version 1.5
    local src="${wanted_bin}_bin.cdx.json"
    if [[ ! -f "$src" ]]; then
      echo "expected SBOM file missing: $crate_dir/$src" >&2
      exit 1
    fi
    mv "$src" "$DIST_DIR/sbom/$dest_name"
    rm -f -- *_bin.cdx.json
  )
}

echo "==> generating SBOM: pluma (root workspace CLI, canonical name)"
collect_sbom "$REPO_ROOT" pluma pluma-cli-sbom.cdx.json

echo "==> generating SBOM: flake (root workspace CLI, deprecated compatibility alias)"
collect_sbom "$REPO_ROOT" flake flake-cli-sbom.cdx.json

echo "==> generating SBOM: fehrest (root workspace CLI, deprecated compatibility alias)"
collect_sbom "$REPO_ROOT" fehrest fehrest-cli-sbom.cdx.json

echo "==> generating SBOM: pluma-migrate (standalone migration tool, canonical name)"
collect_sbom "$REPO_ROOT" pluma-migrate pluma-migrate-sbom.cdx.json

echo "==> generating SBOM: flake-migrate (standalone migration tool, deprecated compatibility alias)"
collect_sbom "$REPO_ROOT" flake-migrate flake-migrate-sbom.cdx.json

if [[ -d desktop/src-tauri ]]; then
  echo "==> generating SBOM: pluma-desktop (desktop shell)"
  collect_sbom "$REPO_ROOT/desktop/src-tauri" pluma-desktop pluma-desktop-sbom.cdx.json
fi

echo "==> SBOM(s) written to $DIST_DIR/sbom/"
ls -la "$DIST_DIR/sbom/"

# Sanity check: every SBOM must actually be parseable JSON with at least
# one component listed, not an empty/broken stub silently reported as
# success.
python3 - "$DIST_DIR"/sbom/*.cdx.json <<'PY'
import json, sys
for path in sys.argv[1:]:
    with open(path, encoding="utf-8") as f:
        doc = json.load(f)
    components = doc.get("components", [])
    print(f"{path}: {len(components)} components, bomFormat={doc.get('bomFormat')}, specVersion={doc.get('specVersion')}")
    assert doc.get("bomFormat") == "CycloneDX", f"{path}: not a CycloneDX document"
    assert len(components) > 0, f"{path}: zero components -- SBOM generation likely broken"
PY
echo "==> SBOM sanity check passed"
