use cargo_pkg_info_test_app::CargoPkgInfo;

#[test]
fn test_pkg_info() {
    // Define expected values
    let expected_pkg_name = Some("cargo-pkg-info-test-app");
    let expected_version = Some("1.2.3-beta");
    let expected_major = Some("1");
    let expected_major_numeric = Some(1);
    let expected_minor = Some("2");
    let expected_minor_numeric = Some(2);
    let expected_patch = Some("3");
    let expected_patch_numeric = Some(3);
    let expected_pre = Some("beta");
    let expected_authors =
        Some("Test Author 1 <test1@example.com>:Test Author 2 <test2@example.com>");
    let expected_description = Some("An example app to test cargo-pkg-info-struct-builder");
    let expected_license = Some("MIT");
    let expected_license_content = Some("=== MOCK LICENSE FOR TESTING ===\n\nPermission is hereby granted to test, debug, and enhance.\n");
    let expected_repository = Some("https://github.com/jzombie/rust-cargo-pkg-info-struct-builder");
    let expected_homepage = Some("https://crates.io/crates/cargo-pkg-info-struct-builder");
    let expected_rust_version = Some("1.84");
    let expected_readme_path = Some("README.md");

    // Check the package name
    assert_eq!(
        CargoPkgInfo::pkg_name(),
        expected_pkg_name,
        "Expected package name to be '{:?}', but got '{:?}'",
        expected_pkg_name,
        CargoPkgInfo::pkg_name()
    );

    // Check versioning
    assert_eq!(
        CargoPkgInfo::pkg_version(),
        expected_version,
        "Expected version to be '{:?}', but got '{:?}'",
        expected_version,
        CargoPkgInfo::pkg_version()
    );

    assert_eq!(
        CargoPkgInfo::version_major(),
        expected_major,
        "Expected major version to be '{:?}', but got '{:?}'",
        expected_major,
        CargoPkgInfo::version_major()
    );
    assert_eq!(
        CargoPkgInfo::version_major_numeric(),
        expected_major_numeric,
        "Expected major version (numeric) to be '{:?}', but got '{:?}'",
        expected_major_numeric,
        CargoPkgInfo::version_major_numeric()
    );
    assert_eq!(
        CargoPkgInfo::version_minor(),
        expected_minor,
        "Expected minor version to be '{:?}', but got '{:?}'",
        expected_minor,
        CargoPkgInfo::version_minor()
    );
    assert_eq!(
        CargoPkgInfo::version_minor_numeric(),
        expected_minor_numeric,
        "Expected minor version (numeric) to be '{:?}', but got '{:?}'",
        expected_minor_numeric,
        CargoPkgInfo::version_major_numeric()
    );
    assert_eq!(
        CargoPkgInfo::version_patch(),
        expected_patch,
        "Expected patch version to be '{:?}', but got '{:?}'",
        expected_patch,
        CargoPkgInfo::version_patch()
    );
    assert_eq!(
        CargoPkgInfo::version_patch_numeric(),
        expected_patch_numeric,
        "Expected patch version (numeric) to be '{:?}', but got '{:?}'",
        expected_patch_numeric,
        CargoPkgInfo::version_patch_numeric()
    );
    assert_eq!(
        CargoPkgInfo::version_pre(),
        expected_pre,
        "Expected pre-release version to be '{:?}', but got '{:?}'",
        expected_pre,
        CargoPkgInfo::version_pre()
    );

    // Check authors
    assert_eq!(
        CargoPkgInfo::authors(),
        expected_authors,
        "Expected authors to be '{:?}', but got '{:?}'",
        expected_authors,
        CargoPkgInfo::authors()
    );

    // Check metadata
    assert_eq!(
        CargoPkgInfo::description(),
        expected_description,
        "Expected description to be '{:?}', but got '{:?}'",
        expected_description,
        CargoPkgInfo::description()
    );
    assert_eq!(
        CargoPkgInfo::license(),
        expected_license,
        "Expected license to be '{:?}', but got '{:?}'",
        expected_license,
        CargoPkgInfo::license()
    );
    assert_eq!(
        CargoPkgInfo::license_content(),
        expected_license_content,
        "Expected license content to be '{:?}', but got '{:?}'",
        expected_license_content,
        CargoPkgInfo::license_content()
    );
    assert_eq!(
        CargoPkgInfo::repository(),
        expected_repository,
        "Expected repository URL to be '{:?}', but got '{:?}'",
        expected_repository,
        CargoPkgInfo::repository()
    );
    assert_eq!(
        CargoPkgInfo::homepage(),
        expected_homepage,
        "Expected homepage URL to be '{:?}', but got '{:?}'",
        expected_homepage,
        CargoPkgInfo::homepage()
    );
    assert_eq!(
        CargoPkgInfo::rust_version(),
        expected_rust_version,
        "Expected Rust version to be '{:?}', but got '{:?}'",
        expected_rust_version,
        CargoPkgInfo::rust_version()
    );
    assert_eq!(
        CargoPkgInfo::readme_path(),
        expected_readme_path,
        "Expected README path to be '{:?}', but got '{:?}'",
        expected_readme_path,
        CargoPkgInfo::readme_path()
    );

    // Ensure build-time metadata exists
    assert!(
        CargoPkgInfo::build_target().is_some(),
        "Expected build target to be set, but it was None",
    );
    assert!(
        CargoPkgInfo::build_time_utc().is_some(),
        "Expected build time to be available, but it was None"
    );
}

#[test]
fn test_custom_vars() {
    // Refer to `build.rs` in `cargo-pkg-info-test-app` for the setting of these

    assert_eq!(option_env!("TEST_CUSTOM_ENV_VAR"), Some("TEST_RESULT"));

    assert_eq!(
        CargoPkgInfo::split_multi_line_custom_var(option_env!("TEST_MULTI_LINE_CUSTOM_ENV_VAR")),
        Some(vec![
            "Some multi-line environment variable".to_string(),
            "".to_string(),
            "Level 1".to_string(),
            "    Level 2".to_string(),
            "        Level 3".to_string(),
        ])
    );
}
