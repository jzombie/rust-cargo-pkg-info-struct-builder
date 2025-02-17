use cargo_pkg_info_struct_builder::inject_build_metadata;
use std::fs;
use syn;

// Note: Additional tests are performed directly in the `cargo-pkg-info-test-app` workspace.

#[test]
fn test_inject_build_metadata() {
    let temp_dir = tempfile::tempdir().unwrap();
    let dest_path = temp_dir.path().join("cargo_pkg_info.rs");

    // Run metadata injection
    inject_build_metadata(dest_path.to_path_buf());

    // Ensure the file is created
    assert!(dest_path.exists(), "cargo_pkg_info.rs should be created");

    // Read and check contents
    let contents = fs::read_to_string(&dest_path).unwrap();
    assert!(
        contents.contains("pub struct CargoPkgInfo"),
        "Struct definition missing"
    );
    assert!(
        contents.contains("CARGO_PKG_NAME"),
        "Package name metadata missing"
    );

    // Validate the file's Rust syntax in-memory (NO FILE CREATION)
    syn::parse_file(&contents).expect("Generated Rust file is invalid!");
}

#[test]
fn test_inject_build_metadata_no_mtime_change() {
    // On some filesystems or OSes, modification time resolution can be coarse.
    // We'll do a short sleep to ensure we can detect changes (if any).
    use std::{thread, time::Duration};

    let temp_dir = tempfile::tempdir().unwrap();
    let dest_path = temp_dir.path().join("cargo_pkg_info.rs");

    // First injection
    inject_build_metadata(dest_path.to_path_buf());
    let first_metadata = fs::metadata(&dest_path).unwrap();
    let first_modified = first_metadata.modified().unwrap();

    // Sleep to ensure we can detect a modification time change
    thread::sleep(Duration::from_secs(1));

    // Second injection (no changes expected)
    inject_build_metadata(dest_path.to_path_buf());
    let second_metadata = fs::metadata(&dest_path).unwrap();
    let second_modified = second_metadata.modified().unwrap();

    // Check that the mod time hasn't changed
    assert_eq!(
        first_modified, second_modified,
        "File modification time changed despite no new metadata!"
    );

    // Validate syntax again
    let contents = fs::read_to_string(&dest_path).unwrap();
    syn::parse_file(&contents).expect("Generated Rust file is invalid on second parse!");
}

/// Ensures that invalid environment variable names are rejected.
#[test]
#[should_panic(expected = "Invalid Cargo environment variable name")]
fn test_invalid_env_var_names() {
    cargo_pkg_info_struct_builder::set_cargo_env_var("INVALID VAR", "test value");
}

#[test]
#[should_panic(expected = "Environment variable name 'CARGO_TEST_VAR' is reserved")]
fn test_reject_reserved_cargo_vars() {
    cargo_pkg_info_struct_builder::set_cargo_env_var("CARGO_TEST_VAR", "test value");
}
