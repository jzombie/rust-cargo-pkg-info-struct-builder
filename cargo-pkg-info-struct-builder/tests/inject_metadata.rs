use cargo_pkg_info_struct_builder::inject_build_metadata;
use std::fs;

#[test]
fn test_inject_build_metadata() {
    let temp_dir = tempfile::tempdir().unwrap(); // Creates a temp directory
    let dest_path = temp_dir.path().join("cargo_pkg_info.rs");

    // Run metadata injection
    inject_build_metadata(dest_path.to_path_buf());

    // Ensure the file is created
    assert!(dest_path.exists(), "cargo_pkg_info.rs should be created");

    // Read and check contents
    let contents = fs::read_to_string(dest_path).unwrap();
    assert!(
        contents.contains("pub struct CargoPkgInfo"),
        "Struct definition missing"
    );
    assert!(
        contents.contains("CARGO_PKG_NAME"),
        "Package name metadata missing"
    );
}
