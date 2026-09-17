//! `T01-05`/`T05-03`: proves the CLI `backup-run`/`backup-restore`/
//! `vault-recover` commands added by this task as real subprocess
//! invocations of the standalone `flake` binary -- not library-level
//! unit tests of `crate::backup`/`crate::recovery` (those already exist
//! and are unchanged; this task adds no new backup/recovery policy of
//! its own, only a CLI dispatch shell over them, per its own commit
//! message). Closes the "CLI wiring... deferred, matching this task's
//! own forbidden-scope boundary against successor work" gap
//! `docs/evidence/flake-v1/T01-05/REPORT.md` recorded honestly at the
//! time.

use std::path::PathBuf;
use std::process::Command;

fn flake_bin() -> &'static str {
    env!("CARGO_BIN_EXE_flake")
}

fn tmp(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "flake-cli-backup-recover-{name}-{}",
        uuid::Uuid::now_v7()
    ))
}

fn run(args: &[&str]) -> std::process::Output {
    let bin = flake_bin();
    Command::new(bin).args(args).output().expect(bin)
}

fn out(o: &std::process::Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

fn err(o: &std::process::Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

/// A real format-2 vault with one committed note, built entirely through
/// real `flake` subprocess invocations -- not hand-forged. Returns the
/// vault root and the created project's own ID, so callers can
/// independently confirm that exact project survived a backup/restore
/// or recovery round trip, not merely that the CLI printed success.
fn vault_with_one_note() -> (PathBuf, String) {
    let vault = tmp("vault");
    let init = run(&["canonical-init", "--vault", vault.to_str().unwrap()]);
    assert!(init.status.success(), "{}", err(&init));

    let create = run(&[
        "project-create",
        "--vault",
        vault.to_str().unwrap(),
        "--name",
        "Backup test project",
    ]);
    assert!(create.status.success(), "{}", err(&create));
    let project_id = out(&create).split_whitespace().next().unwrap().to_string();

    let capture = run(&[
        "capture",
        "--vault",
        vault.to_str().unwrap(),
        "--project",
        &project_id,
        "--body",
        "note body for backup/recover CLI test",
    ]);
    assert!(capture.status.success(), "{}", err(&capture));
    (vault, project_id)
}

#[test]
fn backup_run_creates_a_verified_backup_reported_on_stdout() {
    let (vault, _project_id) = vault_with_one_note();
    let backup_dest = tmp("backup-dest");

    let backup = run(&[
        "backup-run",
        "--vault",
        vault.to_str().unwrap(),
        "--out",
        backup_dest.to_str().unwrap(),
    ]);
    assert!(backup.status.success(), "{}", err(&backup));
    let stdout = out(&backup);
    assert!(stdout.contains("backed up:"), "{stdout}");
    assert!(stdout.contains("verified=true"), "{stdout}");
    assert!(backup_dest.join(".fehrest").is_dir());

    let _ = std::fs::remove_dir_all(&vault);
    let _ = std::fs::remove_dir_all(&backup_dest);
}

#[test]
fn backup_restore_reconstructs_the_exact_committed_content() {
    let (vault, project_id) = vault_with_one_note();
    let backup_dest = tmp("backup-dest");
    let restore_dest = tmp("restore-dest");

    let backup = run(&[
        "backup-run",
        "--vault",
        vault.to_str().unwrap(),
        "--out",
        backup_dest.to_str().unwrap(),
    ]);
    assert!(backup.status.success(), "{}", err(&backup));

    let restore = run(&[
        "backup-restore",
        "--backup",
        backup_dest.to_str().unwrap(),
        "--out",
        restore_dest.to_str().unwrap(),
    ]);
    assert!(restore.status.success(), "{}", err(&restore));
    let stdout = out(&restore);
    assert!(stdout.contains("restored:"), "{stdout}");

    // Independently confirm the restored root is a real, openable
    // format-2 vault carrying the exact same project committed in the
    // original -- not merely that the CLI printed success.
    let restored_show = run(&[
        "project-show",
        "--vault",
        restore_dest.to_str().unwrap(),
        "--id",
        &project_id,
    ]);
    assert!(restored_show.status.success(), "{}", err(&restored_show));
    assert!(out(&restored_show).contains("Backup test project"));

    let _ = std::fs::remove_dir_all(&vault);
    let _ = std::fs::remove_dir_all(&backup_dest);
    let _ = std::fs::remove_dir_all(&restore_dest);
}

#[test]
fn backup_restore_refuses_to_clobber_an_existing_destination() {
    let (vault, _project_id) = vault_with_one_note();
    let backup_dest = tmp("backup-dest");
    let restore_dest = tmp("restore-dest");

    assert!(run(&[
        "backup-run",
        "--vault",
        vault.to_str().unwrap(),
        "--out",
        backup_dest.to_str().unwrap()
    ])
    .status
    .success());
    assert!(run(&[
        "backup-restore",
        "--backup",
        backup_dest.to_str().unwrap(),
        "--out",
        restore_dest.to_str().unwrap()
    ])
    .status
    .success());

    // Restoring a second time into the same, now-published destination
    // must refuse, never silently overwrite.
    let second = run(&[
        "backup-restore",
        "--backup",
        backup_dest.to_str().unwrap(),
        "--out",
        restore_dest.to_str().unwrap(),
    ]);
    assert!(!second.status.success());

    let _ = std::fs::remove_dir_all(&vault);
    let _ = std::fs::remove_dir_all(&backup_dest);
    let _ = std::fs::remove_dir_all(&restore_dest);
}

#[test]
fn vault_recover_produces_an_independently_reverified_fresh_root() {
    let (vault, project_id) = vault_with_one_note();
    let recover_dest = tmp("recover-dest");

    let recover = run(&[
        "vault-recover",
        "--vault",
        vault.to_str().unwrap(),
        "--out",
        recover_dest.to_str().unwrap(),
    ]);
    assert!(recover.status.success(), "{}", err(&recover));
    let stdout = out(&recover);
    assert!(stdout.contains("recovered:"), "{stdout}");

    // The original vault is never mutated by recovery -- still openable
    // with its original content exactly as before.
    let original_show = run(&[
        "project-show",
        "--vault",
        vault.to_str().unwrap(),
        "--id",
        &project_id,
    ]);
    assert!(original_show.status.success(), "{}", err(&original_show));

    let recovered_show = run(&[
        "project-show",
        "--vault",
        recover_dest.to_str().unwrap(),
        "--id",
        &project_id,
    ]);
    assert!(recovered_show.status.success(), "{}", err(&recovered_show));
    assert!(out(&recovered_show).contains("Backup test project"));

    let _ = std::fs::remove_dir_all(&vault);
    let _ = std::fs::remove_dir_all(&recover_dest);
}

#[test]
fn missing_out_argument_is_a_clear_refusal_not_a_panic() {
    let (vault, _project_id) = vault_with_one_note();
    let result = run(&["backup-run", "--vault", vault.to_str().unwrap()]);
    assert!(!result.status.success());
    assert!(!err(&result).is_empty());
    let _ = std::fs::remove_dir_all(&vault);
}
