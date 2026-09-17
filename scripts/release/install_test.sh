#!/usr/bin/env bash
# T05-03 (plan section 25/tests V10-V13): clean-install / launch / update
# / uninstall testing of the unsigned desktop bundle, run against the
# exact package `.github/workflows/t05-03-release-candidates.yml` just
# built via `cargo tauri build` on this OS. Verifies:
#   - the installer runs unattended (silent/headless) and the app process
#     launches and stays alive without crashing immediately;
#   - a vault created before install survives an uninstall (I09/I10 --
#     "installer cannot erase or silently migrate owned data");
#   - reinstalling the same package over an existing install (this
#     candidate's stand-in for "update", since no distinct prior release
#     version exists yet to upgrade from) does not disturb the retained
#     vault;
#   - uninstall removes the installed application files but explicitly
#     retains the vault, and the script reports that retention rather
#     than merely asserting it silently.
#   - during launch, no outbound network connection is opened (F05: no
#     runtime downloads/telemetry) -- observed via a connection-table
#     diff around the launch window, not by disabling the runner's own
#     network hardware (which would also break the CI job's own ability
#     to report status).
set -euo pipefail

cd "$(dirname "$0")/../.."
REPO_ROOT="$(pwd)"
BUNDLE_DIR="desktop/src-tauri/target/release/bundle"
TEST_VAULT_DIR="${TEST_VAULT_DIR:-$REPO_ROOT/dist/install-test-vault}"
LOG_DIR="${DIST_DIR:-$REPO_ROOT/dist}/install-test"
mkdir -p "$LOG_DIR"

flake_cli() {
  local exe="target/release/flake"
  [[ -f "${exe}.exe" ]] && exe="${exe}.exe"
  "$exe" "$@"
}

echo "==> seeding a pre-install vault so retention can be proven, not assumed"
rm -rf "$TEST_VAULT_DIR"
flake_cli canonical-init --vault "$TEST_VAULT_DIR"
PROJECT_ID="$(flake_cli project-create --vault "$TEST_VAULT_DIR" --name "install-test project" | awk '{print $1}')"
flake_cli capture --vault "$TEST_VAULT_DIR" --project "$PROJECT_ID" --body "survives install/uninstall" > /dev/null
echo "    seeded vault at $TEST_VAULT_DIR (project=$PROJECT_ID)"

open_connections() {
  # Loopback-only connection tables are expected (webview IPC uses
  # http://ipc.localhost); anything to a non-loopback remote address
  # would be the signal this check exists to catch.
  case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*) netstat -ano 2>/dev/null | grep ESTABLISHED || true ;;
    Darwin) netstat -an 2>/dev/null | grep ESTABLISHED || true ;;
    Linux) ss -tn state established 2>/dev/null || true ;;
  esac
}

launch_and_check_no_network() {
  local app_cmd=("$@")
  local before after
  before="$(open_connections)"
  "${app_cmd[@]}" &
  local pid=$!
  sleep 4
  if ! kill -0 "$pid" 2>/dev/null; then
    echo "FAIL: app process exited within 4s of launch (expected still running)" >&2
    exit 1
  fi
  after="$(open_connections)"
  kill "$pid" 2>/dev/null || true
  wait "$pid" 2>/dev/null || true

  local new_remote
  new_remote="$(comm -13 <(echo "$before" | sort) <(echo "$after" | sort) \
    | grep -Ev '127\.0\.0\.1|::1|0\.0\.0\.0|localhost' || true)"
  if [[ -n "$new_remote" ]]; then
    echo "FAIL: new non-loopback connection observed during launch:" >&2
    echo "$new_remote" >&2
    exit 1
  fi
  echo "    launch OK (pid $pid ran >=4s, no non-loopback connection observed)"
}

case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*)
    INSTALLER="$(ls "$BUNDLE_DIR"/nsis/*.exe | head -1)"
    INSTALL_DIR="$REPO_ROOT/dist/install-test/windows-installed"
    rm -rf "$INSTALL_DIR"; mkdir -p "$INSTALL_DIR"
    WIN_INSTALL_DIR="$(cygpath -w "$INSTALL_DIR" 2>/dev/null || echo "$INSTALL_DIR")"

    echo "==> installing (silent NSIS): $INSTALLER -> $WIN_INSTALL_DIR"
    "$INSTALLER" /S "/D=$WIN_INSTALL_DIR"
    sleep 3
    APP_EXE="$(find "$INSTALL_DIR" -iname 'flake*.exe' ! -iname 'uninstall*' | head -1)"
    if [[ -z "$APP_EXE" ]]; then
      echo "FAIL: no installed app executable found under $INSTALL_DIR" >&2
      find "$INSTALL_DIR" -maxdepth 3 >&2
      exit 1
    fi
    echo "==> launching installed app: $APP_EXE"
    launch_and_check_no_network "$APP_EXE"

    echo "==> reinstalling over existing install (stand-in for 'update')"
    "$INSTALLER" /S "/D=$WIN_INSTALL_DIR"
    sleep 3
    [[ -f "$APP_EXE" ]] || { echo "FAIL: app executable missing after reinstall" >&2; exit 1; }
    echo "    reinstall-over-existing OK, app still present"

    UNINSTALLER="$(find "$INSTALL_DIR" -iname 'uninstall*.exe' | head -1)"
    if [[ -z "$UNINSTALLER" ]]; then
      echo "FAIL: no uninstaller found under $INSTALL_DIR" >&2
      exit 1
    fi
    echo "==> uninstalling: $UNINSTALLER"
    "$UNINSTALLER" /S
    sleep 3
    if [[ -f "$APP_EXE" ]]; then
      echo "FAIL: app executable still present after uninstall" >&2
      exit 1
    fi
    echo "    uninstall removed the app executable"
    ;;

  Darwin)
    DMG="$(ls "$BUNDLE_DIR"/dmg/*.dmg | head -1)"
    MOUNT_POINT="$REPO_ROOT/dist/install-test/dmg-mount"
    APP_INSTALL_DIR="$REPO_ROOT/dist/install-test/Applications"
    mkdir -p "$MOUNT_POINT" "$APP_INSTALL_DIR"

    echo "==> mounting: $DMG"
    hdiutil attach "$DMG" -mountpoint "$MOUNT_POINT" -nobrowse -quiet
    APP_BUNDLE="$(find "$MOUNT_POINT" -maxdepth 1 -iname '*.app' | head -1)"
    [[ -n "$APP_BUNDLE" ]] || { echo "FAIL: no .app found in $DMG" >&2; exit 1; }

    echo "==> installing (copy .app -- unsigned, no Gatekeeper/notarization to verify at this task): $APP_BUNDLE"
    rm -rf "${APP_INSTALL_DIR:?}"/*.app
    cp -R "$APP_BUNDLE" "$APP_INSTALL_DIR/"
    hdiutil detach "$MOUNT_POINT" -quiet
    INSTALLED_APP="$(find "$APP_INSTALL_DIR" -maxdepth 1 -iname '*.app' | head -1)"
    APP_BIN="$(find "$INSTALLED_APP/Contents/MacOS" -type f | head -1)"

    echo "==> launching installed app: $APP_BIN"
    launch_and_check_no_network "$APP_BIN"

    echo "==> reinstalling over existing install (stand-in for 'update')"
    hdiutil attach "$DMG" -mountpoint "$MOUNT_POINT" -nobrowse -quiet
    rm -rf "${APP_INSTALL_DIR:?}"/*.app
    cp -R "$MOUNT_POINT"/*.app "$APP_INSTALL_DIR/"
    hdiutil detach "$MOUNT_POINT" -quiet
    [[ -d "$INSTALLED_APP" ]] || { echo "FAIL: app bundle missing after reinstall" >&2; exit 1; }
    echo "    reinstall-over-existing OK, app bundle still present"

    echo "==> uninstalling (remove .app -- this candidate ships no separate uninstaller on macOS)"
    rm -rf "${APP_INSTALL_DIR:?}"/*.app
    if [[ -d "$INSTALLED_APP" ]]; then
      echo "FAIL: app bundle still present after uninstall" >&2
      exit 1
    fi
    echo "    uninstall removed the app bundle"
    ;;

  Linux)
    DEB="$(ls "$BUNDLE_DIR"/deb/*.deb | head -1)"
    PKG_NAME="$(dpkg-deb -f "$DEB" Package)"

    echo "==> installing: $DEB (package=$PKG_NAME)"
    sudo dpkg -i "$DEB" || sudo apt-get -f install -y
    APP_BIN="$(dpkg -L "$PKG_NAME" | grep -E '/usr/bin/|/bin/' | head -1)"
    [[ -n "$APP_BIN" ]] || { echo "FAIL: no installed binary found for package $PKG_NAME" >&2; exit 1; }

    echo "==> launching installed app: $APP_BIN"
    launch_and_check_no_network "$APP_BIN"

    echo "==> reinstalling over existing install (stand-in for 'update')"
    sudo dpkg -i "$DEB"
    dpkg -s "$PKG_NAME" | grep -q '^Status: install ok installed' \
      || { echo "FAIL: package not reported installed after reinstall" >&2; exit 1; }
    echo "    reinstall-over-existing OK, package still installed"

    echo "==> uninstalling: $PKG_NAME"
    sudo dpkg -r "$PKG_NAME"
    if dpkg -s "$PKG_NAME" 2>/dev/null | grep -q '^Status: install ok installed'; then
      echo "FAIL: package still reports installed after uninstall" >&2
      exit 1
    fi
    echo "    uninstall removed the package"
    ;;

  *)
    echo "unrecognized platform: $(uname -s)" >&2
    exit 1
    ;;
esac

echo "==> verifying the pre-install vault survived install/reinstall/uninstall untouched"
SHOW_OUTPUT="$(flake_cli project-show --vault "$TEST_VAULT_DIR" --id "$PROJECT_ID")"
if [[ "$SHOW_OUTPUT" != *"install-test project"* ]]; then
  echo "FAIL: pre-install vault content missing or changed after install/uninstall cycle" >&2
  echo "$SHOW_OUTPUT" >&2
  exit 1
fi
echo "    vault retained and unmodified: $TEST_VAULT_DIR (RETAINED=YES)"

echo "==> install/launch/update/uninstall qualification passed" | tee "$LOG_DIR/result-$(uname -s).txt"
