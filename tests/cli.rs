use std::process::Command;

fn run(home: &str, path: &str) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_shorten-path"))
        .env("HOME", home)
        .arg(path)
        .output()
        .expect("failed to run shorten-path");
    assert!(output.status.success());
    String::from_utf8(output.stdout).expect("stdout is not UTF-8")
}

#[test]
fn shortens_path_under_home() {
    assert_eq!(
        run("/home/alice", "/home/alice/repositories/src/github.com"),
        "~/r/s/github.com"
    );
}

#[test]
fn replaces_home_itself() {
    assert_eq!(run("/home/alice", "/home/alice"), "~");
}

#[test]
fn keeps_path_outside_home() {
    assert_eq!(run("/home/alice", "/usr/local/bin"), "/usr/local/bin");
}

#[test]
fn prints_without_trailing_newline() {
    assert!(!run("/home/alice", "/home/alice/foo").ends_with('\n'));
}
