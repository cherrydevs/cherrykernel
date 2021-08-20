// in boot/src/main.rs
use bootloader_locator::locate_bootloader;
use std::{path::Path, process::Command};

pub fn main() {
    let kernel_binary = Path::new("target/x86_64-cherrykernel/debug/cherrykernel")
        .canonicalize()
        .unwrap();
    let bootloader_manifest = locate_bootloader("bootloader").unwrap();

    // new code below

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let kernel_dir = manifest_dir.parent().unwrap();
    let kernel_manifest = kernel_dir.join("Cargo.toml");
    let target_dir = kernel_dir.join("target");
    let out_dir = kernel_binary.parent().unwrap();

    // create a new build command; use the `CARGO` environment variable to
    // also support non-standard cargo versions
    let mut build_cmd = Command::new(env!("CARGO"));

    // pass the arguments
    build_cmd.arg("builder");
    build_cmd.arg("--kernel-manifest").arg(&kernel_manifest);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary);
    build_cmd.arg("--target-dir").arg(&target_dir);
    build_cmd.arg("--out-dir").arg(&out_dir);

    // set the working directory
    let bootloader_dir = bootloader_manifest.parent().unwrap();
    build_cmd.current_dir(&bootloader_dir);

    // run the command
    let exit_status = build_cmd.status().unwrap();
    if !exit_status.success() {
        panic!("bootloader build failed");
    }
}