//! `T05-03`: proves "`pluma` is the new product command... A
//! compatibility alias, if shipped, must call the same new
//! implementation... no hidden behavior fork" (plan section 25) as an
//! actual test, not only a source comment. `pluma`, `flake` and `fehrest`
//! are all built from the exact same `src/main.rs` (see `Cargo.toml`'s own
//! comment on the three `[[bin]]` entries) -- this proves that identity
//! functionally, at the level a real invoker would observe (exit code
//! and stdout/stderr), which is the property that actually matters,
//! rather than asserting raw byte-identical executables (the binaries'
//! compiled bytes legitimately differ in embedded build-path debug
//! metadata -- e.g. the linked PDB path -- which is expected and harmless
//! for distinct cargo bin targets from identical source, not a behavior
//! fork).
//!
//! `pluma` is the canonical name after the 2026-09-20 Flake→Pluma product
//! rename. `flake` and `fehrest` are both kept as deprecated compatibility
//! aliases (`flake` from the rename, `fehrest` from the original Phase T
//! thesis-proof name) and neither is removed here without a separate,
//! explicitly governed breaking release.

use std::process::{Command, Output};

/// Every name this exact compiled program is shipped under. `pluma` first
/// because it is the canonical name every other alias is compared against.
const ALL_NAMES: &[&str] = &["pluma", "flake", "fehrest"];

fn bin(name: &str) -> &'static str {
    match name {
        "pluma" => env!("CARGO_BIN_EXE_pluma"),
        "flake" => env!("CARGO_BIN_EXE_flake"),
        "fehrest" => env!("CARGO_BIN_EXE_fehrest"),
        other => panic!("unknown alias {other}"),
    }
}

fn run(name: &str, args: &[&str]) -> Output {
    let b = bin(name);
    Command::new(b).args(args).output().expect(b)
}

fn assert_identical_output(a: Output, b: Output) {
    assert_eq!(a.status.code(), b.status.code());
    assert_eq!(a.stdout, b.stdout);
    assert_eq!(a.stderr, b.stderr);
}

/// Every alias in `ALL_NAMES` other than `pluma` itself, so each test only
/// has to state the property once and it is checked against every
/// compatibility alias, not just one.
fn other_aliases() -> impl Iterator<Item = &'static str> {
    ALL_NAMES.iter().copied().filter(|n| *n != "pluma")
}

#[test]
fn help_output_is_identical() {
    for alias in other_aliases() {
        assert_identical_output(run("pluma", &["--help"]), run(alias, &["--help"]));
    }
}

#[test]
fn no_args_output_is_identical() {
    for alias in other_aliases() {
        assert_identical_output(run("pluma", &[]), run(alias, &[]));
    }
}

#[test]
fn unknown_command_refusal_is_identical() {
    for alias in other_aliases() {
        assert_identical_output(
            run("pluma", &["not-a-real-command"]),
            run(alias, &["not-a-real-command"]),
        );
    }
}

#[test]
fn vault_init_and_scan_behave_identically_on_independent_vaults() {
    for alias in other_aliases() {
        let pluma_root =
            std::env::temp_dir().join(format!("cli-alias-parity-pluma-{}", uuid::Uuid::now_v7()));
        let alias_root =
            std::env::temp_dir().join(format!("cli-alias-parity-{alias}-{}", uuid::Uuid::now_v7()));

        let pluma_init = run("pluma", &["init", "--vault", pluma_root.to_str().unwrap()]);
        let alias_init = run(alias, &["init", "--vault", alias_root.to_str().unwrap()]);
        assert_eq!(pluma_init.status.code(), alias_init.status.code());
        // Not byte-identical stdout (each prints its own root path), but both
        // must report success the same way and both must have actually
        // published a real vault -- proven by `scan` succeeding identically
        // below, not merely asserted.
        assert!(pluma_init.status.success());
        assert!(alias_init.status.success());

        let pluma_scan = run("pluma", &["scan", "--vault", pluma_root.to_str().unwrap()]);
        let alias_scan = run(alias, &["scan", "--vault", alias_root.to_str().unwrap()]);
        assert_identical_output(pluma_scan, alias_scan);

        let _ = std::fs::remove_dir_all(&pluma_root);
        let _ = std::fs::remove_dir_all(&alias_root);
    }
}
