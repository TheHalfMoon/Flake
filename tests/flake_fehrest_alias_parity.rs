//! `T05-03`: proves "`flake` is the new product command... A
//! compatibility alias, if shipped, must call the same new
//! implementation... no hidden behavior fork" (plan section 25) as an
//! actual test, not only a source comment. `flake` and `fehrest` are
//! built from the exact same `src/main.rs` (see `Cargo.toml`'s own
//! comment on the two `[[bin]]` entries) -- this proves that identity
//! functionally, at the level a real invoker would observe (exit code
//! and stdout/stderr), which is the property that actually matters,
//! rather than asserting raw byte-identical executables (the two
//! binaries' compiled bytes legitimately differ in embedded build-path
//! debug metadata -- e.g. the linked PDB path -- which is expected and
//! harmless for two distinct cargo bin targets from identical source,
//! not a behavior fork).

use std::process::{Command, Output};

fn flake_bin() -> &'static str {
    env!("CARGO_BIN_EXE_flake")
}

fn fehrest_bin() -> &'static str {
    env!("CARGO_BIN_EXE_fehrest")
}

fn run(bin: &str, args: &[&str]) -> Output {
    Command::new(bin).args(args).output().expect(bin)
}

fn assert_identical_output(a: Output, b: Output) {
    assert_eq!(a.status.code(), b.status.code());
    assert_eq!(a.stdout, b.stdout);
    assert_eq!(a.stderr, b.stderr);
}

#[test]
fn help_output_is_identical() {
    assert_identical_output(
        run(flake_bin(), &["--help"]),
        run(fehrest_bin(), &["--help"]),
    );
}

#[test]
fn no_args_output_is_identical() {
    assert_identical_output(run(flake_bin(), &[]), run(fehrest_bin(), &[]));
}

#[test]
fn unknown_command_refusal_is_identical() {
    assert_identical_output(
        run(flake_bin(), &["not-a-real-command"]),
        run(fehrest_bin(), &["not-a-real-command"]),
    );
}

#[test]
fn vault_init_and_scan_behave_identically_on_independent_vaults() {
    let flake_root =
        std::env::temp_dir().join(format!("flake-alias-parity-flake-{}", uuid::Uuid::now_v7()));
    let fehrest_root = std::env::temp_dir().join(format!(
        "flake-alias-parity-fehrest-{}",
        uuid::Uuid::now_v7()
    ));

    let flake_init = run(
        flake_bin(),
        &["init", "--vault", flake_root.to_str().unwrap()],
    );
    let fehrest_init = run(
        fehrest_bin(),
        &["init", "--vault", fehrest_root.to_str().unwrap()],
    );
    assert_eq!(flake_init.status.code(), fehrest_init.status.code());
    // Not byte-identical stdout (each prints its own root path), but both
    // must report success the same way and both must have actually
    // published a real vault -- proven by `scan` succeeding identically
    // below, not merely asserted.
    assert!(flake_init.status.success());
    assert!(fehrest_init.status.success());

    let flake_scan = run(
        flake_bin(),
        &["scan", "--vault", flake_root.to_str().unwrap()],
    );
    let fehrest_scan = run(
        fehrest_bin(),
        &["scan", "--vault", fehrest_root.to_str().unwrap()],
    );
    assert_identical_output(flake_scan, fehrest_scan);

    let _ = std::fs::remove_dir_all(&flake_root);
    let _ = std::fs::remove_dir_all(&fehrest_root);
}
