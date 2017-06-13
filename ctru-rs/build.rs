use std::env;
use std::path::PathBuf;

fn main() {
    let dkp_path = PathBuf::from(env::var("DEVKITPRO").unwrap());

    println!("cargo:rustc-link-search=native={}", dkp_path.join("libctru/lib").display());
    println!("cargo:rustc-link-lib=static={}", match env::var("PROFILE").unwrap().as_str() {
        "release" => "ctru",
        "debug" => "ctrud",
        _ => unreachable!(),
    });
}
