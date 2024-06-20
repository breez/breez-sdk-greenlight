fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_git_revision_hash();
    Ok(())
}

fn set_git_revision_hash() {
    use std::process::Command;

    let args = &["rev-parse", "--short=10", "HEAD"];
    let Ok(output) = Command::new("git").args(args).output() else {
        return;
    };
    let rev = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if rev.is_empty() {
        return;
    }
    println!("cargo:rustc-env=SDK_GIT_HASH={}", rev);
}
