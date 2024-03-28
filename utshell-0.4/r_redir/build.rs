//# SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.

//# SPDX-License-Identifier: GPL-3.0-or-later

use std::env;

fn main() {
    let libsh_dir = "./lib/sh/";

    println!(
        "cargo:rustc-link-search=native={}",
        env::join_paths(&[libsh_dir]).unwrap().to_str().unwrap()
    );

    println!("cargo:rustc-link-args=-Wl,--copy-dt-needed-entries -fpic");

    println!("cargo:rustc-flags=-l static=sh");
    println!("cargo:rustc-link-lib=static=sh");
    println!("cargo:rerun-if-changed=lib/sh/netopen.c");
    //println!("cargo:rustc-flags=-l dylib=rt");
}
