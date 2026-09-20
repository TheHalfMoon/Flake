//! `T05-02`: proves the **standalone** `pluma-fault-workload` binary
//! itself — the workload a real subprocess kill (VM/host-level, not an
//! in-process signal handler) is aimed at during D6 native-unclean-
//! shutdown qualification. Proves exactly the two properties the D6
//! harness depends on: resumability (a partial run picks up from the
//! real on-disk head, never redoing or skipping a commit) and
//! idempotence at target (a run that finds the target already reached
//! commits nothing new) — both load-bearing for treating "kill this
//! process at an unpredictable point, then re-run the identical
//! invocation" as a safe, repeatable fault-injection primitive.

use std::path::PathBuf;
use std::process::Command;

fn workload_bin() -> &'static str {
    env!("CARGO_BIN_EXE_pluma-fault-workload")
}

fn tmp(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "pluma-fault-workload-binary-test-{name}-{}",
        uuid::Uuid::now_v7()
    ))
}

fn run(root: &PathBuf, target: i64) -> std::process::Output {
    Command::new(workload_bin())
        .arg(root)
        .arg(target.to_string())
        .output()
        .expect("pluma-fault-workload must run")
}

#[test]
fn standalone_binary_creates_a_fresh_vault_and_commits_to_target_seq() {
    let root = tmp("fresh");
    let out = run(&root, 5);
    assert!(out.status.success(), "{:?}", out);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("RESUME seq=0 target=5"));
    assert!(stdout.contains("DONE seq=5"));

    let store = fehrest::canonical::CanonicalStore::open(&root).expect("vault must open");
    let (seq, _hash) = store.transaction_head().unwrap();
    assert_eq!(seq, 5);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn standalone_binary_resumes_from_existing_head_seq_without_redoing_or_skipping_work() {
    let root = tmp("resume");
    let first = run(&root, 5);
    assert!(first.status.success());

    let second = run(&root, 12);
    assert!(second.status.success(), "{:?}", second);
    let stdout = String::from_utf8_lossy(&second.stdout);
    assert!(
        stdout.contains("RESUME seq=5 target=12"),
        "second run must resume from the real on-disk head, not restart from 0: {stdout}"
    );
    assert!(stdout.contains("DONE seq=12"));

    let store = fehrest::canonical::CanonicalStore::open(&root).expect("vault must open");
    let (seq, _hash) = store.transaction_head().unwrap();
    assert_eq!(
        seq, 12,
        "final head must be exactly the target, no more and no less"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn standalone_binary_commits_nothing_new_when_target_is_already_reached() {
    let root = tmp("idempotent");
    let first = run(&root, 5);
    assert!(first.status.success());

    let second = run(&root, 5);
    assert!(second.status.success());
    let stdout = String::from_utf8_lossy(&second.stdout);
    assert!(
        stdout.contains("RESUME seq=5 target=5") && stdout.contains("DONE seq=5"),
        "{stdout}"
    );
    assert!(
        !stdout.contains("PROGRESS"),
        "a run that already meets its target must commit nothing new: {stdout}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn standalone_binary_rejects_missing_arguments() {
    let out = Command::new(workload_bin())
        .output()
        .expect("pluma-fault-workload must run");
    assert!(!out.status.success());
}

#[test]
fn standalone_binary_rejects_a_non_numeric_target() {
    let root = tmp("bad-target");
    let out = Command::new(workload_bin())
        .arg(&root)
        .arg("not-a-number")
        .output()
        .expect("pluma-fault-workload must run");
    assert!(!out.status.success());
    assert!(
        !root.exists(),
        "must not touch the filesystem on a rejected argument"
    );
}
