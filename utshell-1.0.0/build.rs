extern crate dunce;

use std::{env, path::PathBuf};

fn main() {
    let library_name_glob = "glob";
    let library_name_readline = "readline";
    let library_name_history = "history";
    let library_name_sh = "sh";
    let library_name_var_params = "var_params";

    let root_r = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    let library_dir_glob = dunce::canonicalize(root_r.join("lib/glob")).unwrap();
    let library_dir_readline = dunce::canonicalize(root_r.join("lib/readline")).unwrap();
    let library_dir_history = dunce::canonicalize(root_r.join("lib/readline")).unwrap();
    let library_dir_sh = dunce::canonicalize(root_r.join("lib/sh")).unwrap();
    let library_dir_var_params = dunce::canonicalize(root_r.join("variable")).unwrap();

    // 添加链接库
    println!("cargo:rustc-link-lib=static={}", library_name_glob);
    println!("cargo:rustc-link-lib=static={}", library_name_readline);
    println!("cargo:rustc-link-lib=static={}", library_name_history);
    println!("cargo:rustc-link-lib=static={}", library_name_sh);
    println!("cargo:rustc-link-lib=static={}", library_name_var_params);

    // 库查找路径
    println!(
        "cargo:rustc-link-search=native={}",
        env::join_paths(&[library_dir_glob])
            .unwrap()
            .to_str()
            .unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        env::join_paths(&[library_dir_readline])
            .unwrap()
            .to_str()
            .unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        env::join_paths(&[library_dir_history])
            .unwrap()
            .to_str()
            .unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        env::join_paths(&[library_dir_sh])
            .unwrap()
            .to_str()
            .unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        env::join_paths(&[library_dir_var_params])
            .unwrap()
            .to_str()
            .unwrap()
    );

    println!("cargo:rustc-link-search=native=/usr/lib64/");
}
