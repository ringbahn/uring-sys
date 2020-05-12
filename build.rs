use std::env;
use std::path::PathBuf;
use std::process::Command;

use cc::Build;

fn main() {
    let project = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).canonicalize().unwrap();
    let liburing = project.join("liburing");

    // configure script
    Command::new("./configure").current_dir(&liburing).output().expect("configure script failed");

    let src = liburing.join("src");

    // liburing
    Build::new().file(src.join("setup.c"))
                .file(src.join("queue.c"))
                .file(src.join("syscall.c"))
                .file(src.join("register.c"))
                .include(src.join("include"))
                .extra_warnings(false)
                .compile("uring");

    // (our additional, linkable C bindings)
    Build::new().file(project.join("rusturing.c")).include(src.join("include"))
                .compile("rusturing");
}
