use std::hash;

use openssl::sha;
//use std::env;

fn main() {
    // if let Ok(v) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
    //     let version = u64::from_str_radix(&v, 16).unwrap();

    //     if version >= 0x1_01_01_00_0 {
    //         println!("cargo:rustc-cfg=openssl111");
    //     }
    // }

    let mut hasher = sha::Sha256::new();

    hasher.update(b"Hello, ");
    hasher.update(b"World!");

    let hash = hasher.finish();
    println!("Hashed \"Hello, World!\" to {:?}", hash);
}
