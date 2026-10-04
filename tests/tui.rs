#![cfg(feature = "gitu")]

use std::process::{Command, Output};

#[test]
fn test_gitu_is_an_alias_of_tui() {
    for arg in ["--version", "--help"] {
        assert_eq!(delta(&["--gitu", arg]), delta(&["--tui", arg]));
    }
}

#[test]
fn test_tui_help_uses_tui() {
    let help = String::from_utf8(delta(&["--tui", "--help"]).stdout).unwrap();
    assert!(help.contains("Usage: delta --tui"), "{}", help);
}

fn delta(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_delta"))
        .args(args)
        .output()
        .unwrap()
}
