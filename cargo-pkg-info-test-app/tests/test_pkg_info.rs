use cargo_pkg_info_test_app::CargoPkgInfo;

#[test]
fn test_pkg_info() {
    // Define expected values
    let expected_pkg_name = "cargo-pkg-info-test-app";
    let expected_version = "1.2.3-beta";
    let expected_major = "1";
    let expected_minor = "2";
    let expected_patch = "3";
    let expected_pre = "beta";
    let expected_authors = "Test Author 1 <test1@example.com>:Test Author 2 <test2@example.com>";
    let expected_description = "An example app to test cargo-pkg-info-struct-builder";
    let expected_license = "MIT";
    let expected_repository = "https://github.com/jzombie/rust-cargo-pkg-info-struct-builder";
    let expected_homepage = "https://crates.io/crates/cargo-pkg-info-struct-builder";
    let expected_rust_version = "1.84";
    let expected_readme_path = "README.md";

    // Check the package name
    assert_eq!(
        CargoPkgInfo::pkg_name(),
        expected_pkg_name,
        "Expected package name to be '{}', but got '{}'",
        expected_pkg_name,
        CargoPkgInfo::pkg_name()
    );

    // Check versioning
    assert_eq!(
        CargoPkgInfo::pkg_version(),
        expected_version,
        "Expected version to be '{}', but got '{}'",
        expected_version,
        CargoPkgInfo::pkg_version()
    );

    assert_eq!(
        CargoPkgInfo::version_major(),
        expected_major,
        "Expected major version to be '{}', but got '{}'",
        expected_major,
        CargoPkgInfo::version_major()
    );
    assert_eq!(
        CargoPkgInfo::version_minor(),
        expected_minor,
        "Expected minor version to be '{}', but got '{}'",
        expected_minor,
        CargoPkgInfo::version_minor()
    );
    assert_eq!(
        CargoPkgInfo::version_patch(),
        expected_patch,
        "Expected patch version to be '{}', but got '{}'",
        expected_patch,
        CargoPkgInfo::version_patch()
    );
    assert_eq!(
        CargoPkgInfo::version_pre(),
        expected_pre,
        "Expected pre-release version to be '{}', but got '{}'",
        expected_pre,
        CargoPkgInfo::version_pre()
    );

    // Check authors
    assert_eq!(
        CargoPkgInfo::authors(),
        expected_authors,
        "Expected authors to be '{}', but got '{}'",
        expected_authors,
        CargoPkgInfo::authors()
    );

    // Check metadata
    assert_eq!(
        CargoPkgInfo::description(),
        expected_description,
        "Expected description to be '{}', but got '{}'",
        expected_description,
        CargoPkgInfo::description()
    );
    assert_eq!(
        CargoPkgInfo::license(),
        expected_license,
        "Expected license to be '{}', but got '{}'",
        expected_license,
        CargoPkgInfo::license()
    );
    assert_eq!(
        CargoPkgInfo::repository(),
        expected_repository,
        "Expected repository URL to be '{}', but got '{}'",
        expected_repository,
        CargoPkgInfo::repository()
    );
    assert_eq!(
        CargoPkgInfo::homepage(),
        expected_homepage,
        "Expected homepage URL to be '{}', but got '{}'",
        expected_homepage,
        CargoPkgInfo::homepage()
    );
    assert_eq!(
        CargoPkgInfo::rust_version(),
        expected_rust_version,
        "Expected Rust version to be '{}', but got '{}'",
        expected_rust_version,
        CargoPkgInfo::rust_version()
    );
    assert_eq!(
        CargoPkgInfo::readme_path(),
        expected_readme_path,
        "Expected README path to be '{}', but got '{}'",
        expected_readme_path,
        CargoPkgInfo::readme_path()
    );

    // Ensure build-time metadata exists
    assert!(
        CargoPkgInfo::build_target() != "N/A",
        "Expected build target to be set, but got '{}'",
        CargoPkgInfo::build_target()
    );
    assert!(
        CargoPkgInfo::build_time_utc().is_some(),
        "Expected build time to be available, but it was None"
    );
}
