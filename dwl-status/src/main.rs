use std::process::Command;

const STDERR: &str = "/tmp/dwl-stderr";
const STDOUT: &str = "/tmp/dwl-stdout";

fn main() {
    assert!(Command::new("watch")
        .arg("-n")
        .arg("1")
        .arg("--precise")
        .arg("--exec")
        .arg("tail")
        .arg("-n")
        .arg("5")
        .arg(STDOUT)
        .arg(STDERR)
        .spawn()
        .expect("msg")
        .wait()
        .is_ok());
}
