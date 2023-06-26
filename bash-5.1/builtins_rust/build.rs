    use std::{env};
    
    fn main() {
        let library_dir = "/opt/rsbash/builtins";
        println!("cargo:rustc-link-search=native={}", env::join_paths(&[library_dir]).unwrap().to_str().unwrap());
    }
