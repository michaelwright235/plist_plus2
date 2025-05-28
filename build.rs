use std::{env, fs::canonicalize, path::PathBuf};

// The version (git tag) of libplist that is going to be used
// Changing it to a new major version may result in an incompatible pregenerated bindings
const LIBPLIST_VERSION: &str = "2.7.0";
const LIBPLIST_REPO: &str = "https://github.com/libimobiledevice/libplist.git";

fn main() {
    // Tell cargo to invalidate the built crate whenever build files change
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

    ////////////////////////////
    //   BINDGEN GENERATION   //
    ////////////////////////////

    if cfg!(feature = "pls-generate") {
        let cur_path = env::current_dir().unwrap();
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        // Clone the repo to access the headers
        env::set_current_dir(&out_path).unwrap();
        repo_clone(LIBPLIST_REPO, None);
        env::set_current_dir(cur_path).unwrap();

        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header("wrapper.h")
            // Include an out dir, that contains a cloned repo
            .clang_arg(format!("-I{}", out_path.as_os_str().to_str().unwrap()))
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }

    if cfg!(feature = "vendored") {
        // Change current directory to OUT_DIR
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        env::set_current_dir(out_path).unwrap();
        // Clone the vendored libraries
        if !cfg!(feature = "pls-generate") {
            // Clone it only if it hasn't been earlier
            repo_clone(LIBPLIST_REPO, Some(LIBPLIST_VERSION));
        }
        repo_setup(LIBPLIST_REPO);
        // Build it
        let dst = autotools::Config::new("libplist")
            .without("cython", None)
            .build();

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );

        println!("cargo:rustc-link-lib=static=plist-2.0");
    } else {
        // Check if folder ./override exists
        let override_path = PathBuf::from("./override").join(env::var("TARGET").unwrap());
        if override_path.exists() {
            println!(
                "cargo:rustc-link-search={}",
                canonicalize(&override_path).unwrap().display()
            );
        }

        println!("cargo:rustc-link-search=/usr/local/lib");
        println!("cargo:rustc-link-search=/usr/lib");
        println!("cargo:rustc-link-search=/opt/homebrew/lib");
        println!("cargo:rustc-link-search=/usr/local/opt/libimobiledevice/lib");
        println!("cargo:rustc-link-search=/usr/local/opt/libusbmuxd/lib");
        println!("cargo:rustc-link-search=/usr/local/opt/libimobiledevice-glue/lib");
    }
}

/// Clones the repository with a given url.
/// A branch (usually a git tag) can also be specified.
fn repo_clone(url: &str, branch: Option<&str>) {
    let mut cmd = std::process::Command::new("git");
    cmd.arg("clone");
    cmd.args(["--depth", "1"]);
    if let Some(b) = branch {
        cmd.args(["--branch", b]);
    }
    cmd.arg(url);
    cmd.output().unwrap();
}

/// Runs config commands for a repo
fn repo_setup(url: &str) {
    env::set_current_dir(url.split('/').next_back().unwrap().replace(".git", "")).unwrap();
    unsafe {
        env::set_var("NOCONFIGURE", "1");
    }
    let mut cmd = std::process::Command::new("./autogen.sh");
    let _ = cmd.output();
    unsafe {
        env::remove_var("NOCONFIGURE");
    }
    env::set_current_dir("..").unwrap();
}
