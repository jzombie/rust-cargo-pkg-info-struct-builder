use std::{env, fs};

fn main() {
    let target = env::var("TARGET").unwrap();
    let build_time = chrono::Utc::now().to_rfc3339();

    // Read the license file if available
    let license_content = env::var("CARGO_PKG_LICENSE_FILE")
        .ok()
        .and_then(|path| fs::read_to_string(&path).ok())
        .unwrap_or_else(|| "License file not found".to_string());

    // Set environment variables at compile time
    println!("cargo:rustc-env=BUILD_TARGET={}", target);
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!(
        "cargo:rustc-env=CARGO_PKG_LICENSE_CONTENT={}",
        license_content
    );
}
