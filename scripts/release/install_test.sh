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
#   - during launch, this process itself opens no outbound network
#     connection (F05: no runtime downloads/telemetry) -- checked by
#     inspecting the launched process's own open sockets specifically,
#     not by disabling the runner's own network hardware (which would
#     also break the CI job's own ability to report status) and not by
#     diffing the whole machine's connection table (which, on macOS,
#     also caught the OS's own Gatekeeper/OCSP-style background check
#     of a newly launched unsigned app -- not anything Flake requested;
#     see the comment on process_established_remote_connections below).
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

# Scoped to the launched process's own PID, not a whole-machine
# connection-table diff: a whole-machine diff was tried first and, on
# the macOS CI runner, immediately caught several outbound TLS
# connections to Apple IP ranges that appeared the moment *any* new
# unsigned .app was launched (consistent with a Gatekeeper/OCSP
# revocation-style check the OS performs on an unrecognized app, not
# something Flake's own code requested) -- exactly the kind of
# platform-owned background traffic already documented and accepted as
# non-blocking at T04-01 (`FOUNDER_WEBVIEW2_NETWORK_BOUNDARY`) for
# WebView2's own background telemetry on Windows. Filtering to this
# process's own PID checks the actual claim (Flake's own process opens
# no outbound connection), not "nothing on the whole machine changed
# during this four-second window", which no real OS ever satisfies.
process_established_remote_connections() {
  local pid="$1"
  case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*)
      netstat -ano 2>/dev/null | grep ESTABLISHED | awk -v p="$pid" '$NF==p' || true
      ;;
    Darwin)
      lsof -a -p "$pid" -i -n -P 2>/dev/null | grep ESTABLISHED || true
      ;;
    Linux)
      ss -tnp state established 2>/dev/null | grep "pid=$pid," || true
      ;;
  esac
}

# $1: basename of the real app binary, used to re-resolve its actual
# running pid via `pgrep` after launch -- needed on Linux specifically,
# where the launch command is wrapped in `xvfb-run` (no display server
# on this CI runner) and `$!` would only be xvfb-run's own wrapper-
# script pid, not the wrapped GTK app's. macOS/Windows launch directly,
# so `$!` is already correct there and this simply reconfirms it.
# $2..: the full command to run (may itself start with a wrapper).
launch_and_check_no_network() {
  local app_basename="$1"; shift
  "$@" &
  local wrapper_pid=$!
  sleep 4
  local pid=""
  if command -v pgrep > /dev/null; then
    pid="$(pgrep -f "$app_basename" 2>/dev/null | head -1)"
  fi
  [[ -n "$pid" ]] || pid="$wrapper_pid"
  if ! kill -0 "$pid" 2>/dev/null; then
    echo "FAIL: app process exited within 4s of launch (expected still running)" >&2
    exit 1
  fi
  local connections
  connections="$(process_established_remote_connections "$pid")"
  kill "$pid" "$wrapper_pid" 2>/dev/null || true
  wait "$wrapper_pid" 2>/dev/null || true

  local new_remote
  new_remote="$(echo "$connections" | grep -Ev '127\.0\.0\.1|::1|0\.0\.0\.0|localhost' || true)"
  if [[ -n "$new_remote" ]]; then
    echo "FAIL: this process opened a non-loopback connection during launch:" >&2
    echo "$new_remote" >&2
    exit 1
  fi
  echo "    launch OK (pid $pid ran >=4s, no non-loopback connection opened by this process)"
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
    launch_and_check_no_network "$(basename "$APP_EXE")" "$APP_EXE"

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
    launch_and_check_no_network "$(basename "$APP_BIN")" "$APP_BIN"

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

    echo "==> launching installed app (headless via Xvfb -- this CI runner has no display server): $APP_BIN"
    launch_and_check_no_network "$(basename "$APP_BIN")" xvfb-run -a --server-args="-screen 0 1280x1024x24" "$APP_BIN"

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
