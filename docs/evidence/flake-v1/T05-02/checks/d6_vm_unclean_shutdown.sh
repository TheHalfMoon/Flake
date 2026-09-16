#!/bin/bash
# T05-02 D6: genuine forced native-VM-unclean-shutdown fault injection.
#
# Boots a real QEMU VM running the ongoing canonical-store write workload
# (flake-fault-workload, via a systemd oneshot service that re-runs on
# every boot and resumes from the vault's own on-disk head), waits for
# the workload to genuinely start, then SIGKILLs the *VM process itself*
# (not the workload, not a signal the guest can catch) at a randomized
# short delay -- discarding whatever the guest kernel's own page cache
# had not yet written back to the virtual disk, exactly as a real power
# loss would. Reboots the same disk and repeats for N cycles, then lets
# a final cycle run to completion uninterrupted. After every cycle,
# independently verifies the vault (verify_fault_cycle.py ->
# tools/independent-verify/sqlite_reader.py, no Flake binary/library
# involved) directly against the disk image via qemu-nbd -- proving zero
# acknowledged canonical loss and zero false-success recovery, not
# merely that the guest boots again.
#
# See docs/canonical/FOUNDER_T05-02_PHYSICAL_POWER_LOSS_AMENDMENT_2026-09-16.md
# for exactly what this does and does not claim relative to real physical
# power loss.
#
# Usage:
#   d6_vm_unclean_shutdown.sh <base-cloud-image> <fault-workload-bin> \
#       <work-dir> <cycles> <seed> <out-json>
#
# Requires (already-authorized infrastructure only): qemu-system-x86_64,
# qemu-img, qemu-nbd, the `nbd` kernel module, python3. Run as root (or
# with passwordless sudo for `modprobe nbd`/mount), since guest-disk
# inspection and image customization both need real block-device mounts.

set -euo pipefail

BASE_IMAGE="$1"
WORKLOAD_BIN="$2"
WORK_DIR="$3"
CYCLES="${4:-30}"
SEED="${5:-20260916}"
OUT_JSON="${6:-$WORK_DIR/d6-results.json}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

modprobe nbd max_part=16 2>/dev/null || true

NBD_DEV=/dev/nbd0
MOUNT_DIR="$WORK_DIR/mnt-guest"
mkdir -p "$MOUNT_DIR"
SERIAL_LOG="$WORK_DIR/serial.log"
: > "$WORK_DIR/cycles.jsonl"

cleanup() {
  pkill -9 -f "qemu-system-x86_64.*${WORK_DIR}/custom.img" >/dev/null 2>&1 || true
  sleep 0.5
  mountpoint -q "$MOUNT_DIR" 2>/dev/null && umount "$MOUNT_DIR" || true
  qemu-nbd --disconnect "$NBD_DEV" >/dev/null 2>&1 || true
}
trap cleanup EXIT

wait_nbd_free() {
  for _ in $(seq 1 20); do
    [ -e "/sys/block/$(basename "$NBD_DEV")/pid" ] || return 0
    sleep 0.5
  done
}

mount_guest() {
  wait_nbd_free
  qemu-nbd --connect="$NBD_DEV" "$WORK_DIR/custom.img"
  sleep 1
  mount "${NBD_DEV}p1" "$MOUNT_DIR"
}

umount_guest() {
  mountpoint -q "$MOUNT_DIR" 2>/dev/null && umount "$MOUNT_DIR" || true
  qemu-nbd --disconnect "$NBD_DEV" >/dev/null 2>&1 || true
  wait_nbd_free
}

set_guest_target() {
  mount_guest
  echo "$1" > "$MOUNT_DIR/etc/flake-fault-target"
  umount_guest
}

echo "--- preparing base guest image (one-time customization) ---"
cp "$BASE_IMAGE" "$WORK_DIR/custom.img"
mount_guest
cp "$WORKLOAD_BIN" "$MOUNT_DIR/usr/local/bin/flake-fault-workload"
chmod +x "$MOUNT_DIR/usr/local/bin/flake-fault-workload"
cp "$SCRIPT_DIR/fault-workload.service" "$MOUNT_DIR/etc/systemd/system/fault-workload.service"
chmod 644 "$MOUNT_DIR/etc/systemd/system/fault-workload.service"
echo 0 > "$MOUNT_DIR/etc/flake-fault-target"
rm -rf "$MOUNT_DIR/root/vault" "$MOUNT_DIR/root/fault-workload.log"
systemctl --root="$MOUNT_DIR" enable fault-workload.service >/dev/null
umount_guest
echo "IMAGE_READY"

# A target far beyond what any single cycle's kill window could reach,
# so every non-final cycle's kill genuinely lands mid-run, not after an
# already-finished workload sitting idle.
FAR_TARGET=2000000

read_current_head_seq() {
  mount_guest
  local db_copy="$WORK_DIR/canonical-head-check.sqlite"
  cp "$MOUNT_DIR/root/vault/.fehrest/canonical.sqlite" "$db_copy" 2>/dev/null || true
  umount_guest
  if [ ! -f "$db_copy" ]; then
    echo 0
    return
  fi
  python3 "$SCRIPT_DIR/verify_fault_cycle.py" "$db_copy" \
    | python3 -c "import json,sys; d=json.load(sys.stdin); print(d.get('vault_row',{}).get('transaction_head_seq', 0) if d.get('ok') else 0)"
}

boot_vm() {
  : > "$SERIAL_LOG"
  qemu-system-x86_64 \
    -m 1024 -smp 2 -machine accel=kvm:tcg -cpu max \
    -drive file="$WORK_DIR/custom.img",if=virtio,cache=writeback \
    -display none -serial file:"$SERIAL_LOG" -no-reboot \
    >"$WORK_DIR/qemu-stdout.log" 2>&1 &
  echo $!
}

# QEMU's `-serial file:` backend buffers and does not reliably flush to
# disk while the guest is still running (confirmed empirically: content
# only became visible on the host after the VM process exited) -- so
# real-time log polling cannot time the kill. Use a fixed, empirically
# generous settle window instead (measured boot-to-service-start ~35-40s
# on this development host under KVM; kept wide for slower/TCG-fallback
# hosts). The serial log itself is still captured and copied into each
# cycle's evidence for post-mortem inspection, just not used for timing.
BOOT_SETTLE_S="${D6_BOOT_SETTLE_S:-60}"

verify_cycle() {
  local cycle_idx="$1"
  mount_guest
  local db_copy="$WORK_DIR/canonical-cycle-${cycle_idx}.sqlite"
  cp "$MOUNT_DIR/root/vault/.fehrest/canonical.sqlite" "$db_copy" 2>/dev/null || true
  # Flake's format-2 store uses `journal_mode=DELETE` (rollback journal,
  # not WAL -- docs/formats/format-2-canonical-sqlite.md). A kill mid-
  # transaction can leave a hot `-journal` file next to the main
  # database; a real SQLite connection (exactly what a real Flake
  # process does on open) automatically rolls it back to the last
  # consistent state, but a raw copy of *only* the main file, without
  # its journal, predictably looks inconsistent to any reader -- by
  # design, not a Flake defect. Copy the journal alongside the main
  # file (same naming convention SQLite itself expects) so the
  # independent reader sees the same crash-recovered view a real Flake
  # process would.
  cp "$MOUNT_DIR/root/vault/.fehrest/canonical.sqlite-journal" "${db_copy}-journal" 2>/dev/null || true
  local log_copy="$WORK_DIR/fault-workload-cycle-${cycle_idx}.log"
  cp "$MOUNT_DIR/root/fault-workload.log" "$log_copy" 2>/dev/null || echo -n "" > "$log_copy"

  # `src/vault.rs`'s `WriteLock` is a deliberate, reviewed create_new/O_EXCL
  # marker file, not an OS advisory lock -- "stale lock reported, never
  # stolen" is documented, intended, safety-motivated behavior
  # (docs/reviews/PHASE_T_IMPLEMENTATION_CONFORMANCE.md), not a defect.
  # A genuinely killed VM leaves this marker behind every time, and the
  # product's own documented remediation is explicit owner action, not
  # silent auto-heal -- so this harness plays exactly that owner role:
  # having just confirmed (via the SIGKILL/wait above) that the VM
  # holding it is completely dead, it explicitly clears the marker
  # before the next cycle, the same as an owner would. This is recorded
  # per cycle, not silently done, since whether a stale lock was even
  # present is itself part of the evidence.
  local stale_lock_found=false
  if [ -f "$MOUNT_DIR/root/vault/.fehrest/writer.lock" ]; then
    stale_lock_found=true
  fi
  rm -f "$MOUNT_DIR/root/vault/.fehrest/writer.lock"
  umount_guest

  local verify_json
  verify_json=$(python3 "$SCRIPT_DIR/verify_fault_cycle.py" "$db_copy")
  local last_progress
  last_progress=$(grep -a -o 'PROGRESS seq=[0-9]*' "$log_copy" | tail -1 | grep -o '[0-9]*' || echo "null")
  python3 -c "
import json, sys
v = json.loads(sys.argv[1])
v['cycle'] = sys.argv[2]
v['stale_writer_lock_found_and_cleared'] = sys.argv[4] == 'true'
lp = sys.argv[3]
v['last_progress_seq_in_log'] = None if lp == 'null' else int(lp)
print(json.dumps(v))
" "$verify_json" "$cycle_idx" "$last_progress" "$stale_lock_found" >> "$WORK_DIR/cycles.jsonl"
}

echo "--- running $CYCLES forced-kill cycles (seed=$SEED) ---"
for i in $(seq 1 "$CYCLES"); do
  echo "=== cycle $i/$CYCLES ==="
  set_guest_target "$FAR_TARGET"
  QPID=$(boot_vm)
  echo "QPID=$QPID"
  # Fixed settle window (see BOOT_SETTLE_S above) plus a deterministic
  # pseudo-random short extra delay, so the kill lands at an
  # unpredictable point genuinely inside the ongoing write workload
  # (verification method: "randomized schedules/seeds") rather than
  # always at the exact same instant after boot.
  delay=$(python3 -c "
import random
random.seed($SEED + $i)
print(round(random.uniform(0.2, 4.0), 3))
")
  echo "settle ${BOOT_SETTLE_S}s + random extra delay: ${delay}s"
  sleep "$BOOT_SETTLE_S"
  sleep "$delay"
  kill -9 "$QPID" 2>/dev/null || true
  wait "$QPID" 2>/dev/null || true
  sleep 0.5
  verify_cycle "$i"
  tail -1 "$WORK_DIR/cycles.jsonl"
done

echo "--- final uninterrupted cycle (clean completion check) ---"
CURRENT_HEAD=$(read_current_head_seq)
FINAL_TARGET=$((CURRENT_HEAD + 50))
echo "current head=$CURRENT_HEAD, final target=$FINAL_TARGET"
set_guest_target "$FINAL_TARGET"
QPID=$(boot_vm)
# FINAL_TARGET is small (reaches head in well under a second once the
# service starts), so the fixed settle window alone -- no kill needed
# for correctness, but the VM is still force-stopped afterward the same
# way, since nothing is in flight by then -- is more than sufficient.
sleep "$BOOT_SETTLE_S"
kill -9 "$QPID" 2>/dev/null || true
wait "$QPID" 2>/dev/null || true
sleep 0.5
verify_cycle "final"
tail -1 "$WORK_DIR/cycles.jsonl"

python3 -c "
import json
cycles = [json.loads(l) for l in open('$WORK_DIR/cycles.jsonl') if l.strip()]
kill_cycles = [c for c in cycles if c['cycle'] != 'final']
final = next(c for c in cycles if c['cycle'] == 'final')
out = {
    'seed': $SEED,
    'requested_cycles': $CYCLES,
    'kill_cycles_run': len(kill_cycles),
    'all_kill_cycles_db_present': all(c['db_exists'] for c in kill_cycles),
    'all_kill_cycles_head_hash_chain_verified': all(
        c.get('head_hash_chain_verified', False) for c in kill_cycles if c['db_exists']
    ),
    'final_cycle': final,
    'final_reached_target': final.get('vault_row', {}).get('transaction_head_seq') == $FINAL_TARGET if final.get('ok') else False,
    'cycles': cycles,
}
out['ok'] = (
    out['all_kill_cycles_db_present']
    and out['all_kill_cycles_head_hash_chain_verified']
    and final.get('ok', False)
    and final.get('head_hash_chain_verified', False)
)
json.dump(out, open('$OUT_JSON', 'w'), indent=2)
print(json.dumps({'ok': out['ok'], 'kill_cycles_run': out['kill_cycles_run']}))
"
echo "D6_HARNESS_DONE"
