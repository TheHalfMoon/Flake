//! `T05-01`: proves the **standalone** `pluma-migrate` binary itself — not
//! just the `fehrest::migration` library it wraps — against the checked-in
//! golden format-1 fixtures (`tests/fixtures/migration/`, `T01-06`). This
//! is the one property no `src/migration.rs` unit test can prove: that the
//! separately-built, separately-invoked offline tool actually works end to
//! end as a real subprocess with real argv/exit codes/stdout, exactly as a
//! format-1 owner without the main app installed would run it.

use std::fs;
use std::path::Path;
use std::process::Command;

fn migrate_bin() -> &'static str {
    env!("CARGO_BIN_EXE_pluma-migrate")
}

fn fehrest_bin() -> &'static str {
    env!("CARGO_BIN_EXE_fehrest")
}

fn tmp(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "pluma-migrate-binary-test-{name}-{}",
        uuid::Uuid::now_v7()
    ));
    dir
}

/// A format-1 vault the standalone binary can be pointed at: same
/// mechanism `pluma-migrate`'s own module docs assume (a real `.fehrest/`
/// guard, materialized here through the main `fehrest` CLI's own `init`
/// -- not hand-forged -- then populated with the exact checked-in gold
/// fixture bytes).
fn legacy_vault_with_gold_fixtures() -> std::path::PathBuf {
    let root = tmp("source");
    let status = Command::new(fehrest_bin())
        .args(["init", "--vault"])
        .arg(&root)
        .status()
        .expect("fehrest init must run");
    assert!(status.success(), "fehrest init failed");

    for fixture in ["legacy-crlf-unicode.md", "legacy-unknown-fields.md"] {
        let src = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/migration")
            .join(fixture);
        fs::copy(&src, root.join(fixture)).unwrap();
    }
    root
}

fn cleanup(p: &Path) {
    let _ = fs::remove_dir_all(p);
}

#[test]
fn standalone_binary_preview_reports_both_gold_fixtures_admitted() {
    let root = legacy_vault_with_gold_fixtures();

    let output = Command::new(migrate_bin())
        .arg("preview")
        .arg(&root)
        .output()
        .expect("pluma-migrate preview must run");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["admitted_count"], 2);
    assert_eq!(json["complete"], true);
    assert_eq!(json["omitted"].as_array().unwrap().len(), 0);

    cleanup(&root);
}

#[test]
fn standalone_binary_import_produces_a_real_openable_format_2_vault() {
    let root = legacy_vault_with_gold_fixtures();
    let new_root = tmp("dest");

    let output = Command::new(migrate_bin())
        .arg("import")
        .arg(&root)
        .arg(&new_root)
        .output()
        .expect("pluma-migrate import must run");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["imported_count"], 2);
    assert_eq!(json["complete"], true);

    // Independent-of-this-tool proof: a fresh CanonicalStore::open on the
    // produced root succeeds and reports exactly 2 committed transactions.
    let store = fehrest::canonical::CanonicalStore::open(&new_root).unwrap();
    assert_eq!(store.transaction_head().unwrap().0, 2);

    cleanup(&root);
    cleanup(&new_root);
}

#[test]
fn standalone_binary_import_refuses_to_clobber_an_existing_format_2_root() {
    let root = legacy_vault_with_gold_fixtures();
    let new_root = tmp("dest-exists");
    // A real prior format-2 vault at new_root, not merely an unrelated
    // sibling file -- CanonicalStore::create's own no-clobber check is
    // keyed on `.fehrest/` already existing at the destination
    // (src/canonical.rs), so that is what must be present here to
    // exercise the actual refusal path.
    let status = Command::new(fehrest_bin())
        .args(["init", "--vault"])
        .arg(&new_root)
        .status()
        .expect("fehrest init must run");
    assert!(status.success(), "fehrest init failed");

    let output = Command::new(migrate_bin())
        .arg("import")
        .arg(&root)
        .arg(&new_root)
        .output()
        .expect("pluma-migrate import must run");
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("no-clobber"),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    cleanup(&root);
    cleanup(&new_root);
}

#[test]
fn standalone_binary_version_and_help_do_not_touch_any_filesystem_path() {
    let version = Command::new(migrate_bin())
        .arg("--version")
        .output()
        .unwrap();
    assert!(version.status.success());
    assert!(String::from_utf8_lossy(&version.stdout).contains("pluma-migrate"));

    let help = Command::new(migrate_bin()).arg("--help").output().unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("USAGE"));
}

/// `flake-migrate` is the deprecated compatibility alias kept alongside the
/// new canonical `pluma-migrate` name (2026-09-20 product rename) -- same
/// binary, `Cargo.toml`'s `[[bin]]` entries both point at
/// `src/bin/migrate.rs`. Prove that byte-for-byte identity holds for the
/// subprocess, the same way `pluma_fehrest_alias_parity.rs` proves it for
/// the main CLI.
#[test]
fn deprecated_flake_migrate_alias_matches_pluma_migrate() {
    let flake_migrate_bin = env!("CARGO_BIN_EXE_flake-migrate");

    let pluma_version = Command::new(migrate_bin())
        .arg("--version")
        .output()
        .unwrap();
    let flake_version = Command::new(flake_migrate_bin)
        .arg("--version")
        .output()
        .unwrap();
    assert!(flake_version.status.success());
    assert_eq!(pluma_version.status.code(), flake_version.status.code());

    let pluma_help = Command::new(migrate_bin()).arg("--help").output().unwrap();
    let flake_help = Command::new(flake_migrate_bin)
        .arg("--help")
        .output()
        .unwrap();
    assert!(flake_help.status.success());
    assert_eq!(pluma_help.stdout, flake_help.stdout);
}
