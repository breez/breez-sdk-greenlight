use std::process::Command;

uniffi::build_foreign_language_testcases!(
    "tests/bindings/test_breez_sdk.swift",
    "tests/bindings/test_breez_sdk.kts",
    "tests/bindings/test_breez_sdk.py"
);

#[test]
fn test_csharp() {
    let output = Command::new("dotnet")
        .arg("run")
        .arg("--project")
        .arg("tests/bindings/csharp/sdk-cs-demo.csproj")
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success());
}

#[test]
fn test_golang() {
    let output = Command::new("go")
        .env(
            "CGO_LDFLAGS",
            "-lbreez_sdk_bindings -L../../../ffi/golang -Wl,-rpath,../../../ffi/golang",
        )
        .env("CGO_ENABLED", "1")
        .current_dir("tests/bindings/golang/")
        .arg("run")
        .arg("./")
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success());
}
