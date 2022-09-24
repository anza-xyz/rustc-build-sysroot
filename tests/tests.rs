use std::process::{self, Command};

use rustc_version::VersionMeta;
use tempdir::TempDir;

use rustc_build_sysroot::*;

fn run(cmd: &mut Command) {
    assert!(cmd.status().expect("failed to run {cmd:?}").success());
}

#[test]
fn host() {
    let rustc_version = VersionMeta::for_command(Command::new("rustc")).unwrap();
    let src_dir = rustc_sysroot_src(Command::new("rustc")).unwrap();

    for mode in [BuildMode::Build, BuildMode::Check] {
        let sysroot_dir = TempDir::new("rustc-build-sysroot-test-sysroot").unwrap();
        let sysroot = Sysroot::new(sysroot_dir.path(), &rustc_version.host);
        sysroot
            .build_from_source(&src_dir, mode, &rustc_version, || {
                let mut cmd = Command::new("cargo");
                cmd.stdout(process::Stdio::null());
                cmd.stderr(process::Stdio::null());
                cmd
            })
            .unwrap();

        let crate_name = "rustc-build-sysroot-test-crate";
        let crate_dir = TempDir::new(crate_name).unwrap();
        run(Command::new("cargo")
            .args(&["new", crate_name])
            .current_dir(&crate_dir));
        let crate_dir = crate_dir.path().join(crate_name);
        run(Command::new("cargo")
            .arg(mode.as_str())
            .current_dir(&crate_dir)
            .env(
                "RUSTFLAGS",
                format!("--sysroot {}", sysroot_dir.path().display()),
            ));
    }
}
