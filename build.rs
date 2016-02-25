// Copyright (c) 2016 Adam Perry <adam.n.perry@gmail.com>
//
// This software may be modified and distributed under the terms of the MIT license.  See the
// LICENSE file for details.

use std::fs::copy;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env!("OUT_DIR");

    // configure the build
    Command::new("./configure")
        .current_dir(Path::new("parasail_c"))
        .output()
        .expect("Failed to configure parasail.");

    // build the library
    Command::new("make")
        .current_dir(Path::new("parasail_c"))
        .output()
        .expect("Failed to build parasail.");

    // put the static library in the right directory so we can clean up
    let target_file = format!("{}/libparasail.a", out_dir);
    copy("parasail_c/.libs/libparasail.a", target_file)
        .expect("Problem copying library to target directoy.");

    // clean up the temporary build files
    Command::new("make")
        .current_dir(Path::new("parasail_c"))
        .arg("clean")
        .output()
        .expect("Failed to clean up build files.");

    // clean up the configuration files
    Command::new("make")
        .current_dir(Path::new("parasail_c"))
        .arg("distclean")
        .output()
        .expect("Failed to clean up configuration files.");

    // let cargo know that it can find the file in the out directory
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=parasail");
}
