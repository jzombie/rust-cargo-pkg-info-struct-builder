//------------------------------------------------------------------------------
// This file is automatically generated by `cargo-pkg-info-struct-builder`.
//
// DO NOT EDIT THIS FILE MANUALLY. ANY CHANGES WILL BE OVERWRITTEN.
//
// It contains a struct `CargoPkgInfo` that provides access to package metadata
// set by Cargo at compile time, including versioning, authors, license
// information, and build details. The values are obtained from environment
// variables defined in the Cargo.toml file and passed during the build process.
//
// For more information, see:
// - Cargo Environment Variables: <https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates>
// - Repository: <https://github.com/jzombie/rust-cargo-pkg-info-struct-builder>
//------------------------------------------------------------------------------

pub struct CargoPkgInfo {}

/// Macro to convert escaped `\\n` sequences back into actual newline characters (`\n`).
///
/// This ensures environment variables or serialized data containing escaped
/// newlines are correctly interpreted as multi-line text.
///
/// Note: For simplicity this is intentionally normalized to Unix-style
/// line endings, though testing is performed cross-platform.
#[macro_export]
macro_rules! unescape_newlines {
    ($s:expr) => {
        $s.replace("\\n", "\n")
    };
}

impl CargoPkgInfo {
    /// Returns the package name.
    #[allow(dead_code)]
    pub fn pkg_name() -> Option<&'static str> {
        option_env!("CARGO_PKG_NAME")
    }

    /// Returns the crate name.
    #[allow(dead_code)]
    pub fn crate_name() -> Option<&'static str> {
        option_env!("CARGO_CRATE_NAME")
    }

    /// Returns the package version.
    #[allow(dead_code)]
    pub fn pkg_version() -> Option<&'static str> {
        option_env!("CARGO_PKG_VERSION")
    }

    /// Returns the major version of the package.
    #[allow(dead_code)]
    pub fn version_major() -> Option<&'static str> {
        option_env!("CARGO_PKG_VERSION_MAJOR")
    }

    /// Returns the major version of the package as a number.
    #[allow(dead_code)]
    pub fn version_major_numeric() -> Option<u32> {
        option_env!("CARGO_PKG_VERSION_MAJOR").and_then(|s| s.parse().ok())
    }

    /// Returns the minor version of the package.
    #[allow(dead_code)]
    pub fn version_minor() -> Option<&'static str> {
        option_env!("CARGO_PKG_VERSION_MINOR")
    }

    /// Returns the minor version of the package as a number.
    #[allow(dead_code)]
    pub fn version_minor_numeric() -> Option<u32> {
        option_env!("CARGO_PKG_VERSION_MINOR").and_then(|s| s.parse().ok())
    }

    /// Returns the patch version of the pacakge.
    #[allow(dead_code)]
    pub fn version_patch() -> Option<&'static str> {
        option_env!("CARGO_PKG_VERSION_PATCH")
    }

    /// Returns the patch version of the pacakge as a number.
    #[allow(dead_code)]
    pub fn version_patch_numeric() -> Option<u32> {
        option_env!("CARGO_PKG_VERSION_PATCH").and_then(|s| s.parse().ok())
    }

    /// Returns the pre-release version of the package.
    #[allow(dead_code)]
    pub fn version_pre() -> Option<&'static str> {
        option_env!("CARGO_PKG_VERSION_PRE")
    }

    /// Returns the authors of the package.
    #[allow(dead_code)]
    pub fn authors() -> Option<&'static str> {
        option_env!("CARGO_PKG_AUTHORS")
    }

    /// Returns the description of the package.
    #[allow(dead_code)]
    pub fn description() -> Option<&'static str> {
        option_env!("CARGO_PKG_DESCRIPTION")
    }

    /// Returns the homepage URL of the package.
    #[allow(dead_code)]
    pub fn homepage() -> Option<&'static str> {
        option_env!("CARGO_PKG_HOMEPAGE")
    }

    /// Returns the repository URL of the package.
    #[allow(dead_code)]
    pub fn repository() -> Option<&'static str> {
        option_env!("CARGO_PKG_REPOSITORY")
    }

    /// Returns the license type of the package.
    #[allow(dead_code)]
    pub fn license() -> Option<&'static str> {
        option_env!("CARGO_PKG_LICENSE")
    }

    /// Returns the contents of the license file (embedded at build time).
    #[allow(dead_code)]
    pub fn license_content() -> Option<&'static str> {
        let license = option_env!("LICENSE_CONTENT");

        // Return immediately if no license
        license?;

        Some(Self::unescape_newlines(license.unwrap()))
    }

    /// Returns the Rust version required by the package.
    #[allow(dead_code)]
    pub fn rust_version() -> Option<&'static str> {
        option_env!("CARGO_PKG_RUST_VERSION")
    }

    /// Returns the path to the README file.
    #[allow(dead_code)]
    pub fn readme_path() -> Option<&'static str> {
        option_env!("CARGO_PKG_README")
    }

    /// Returns the build target (architecture/platform).
    #[allow(dead_code)]
    pub fn build_target() -> Option<&'static str> {
        option_env!("BUILD_TARGET")
    }

    /// Returns the UTC build time as an `Option<u64>`.
    #[allow(dead_code)]
    pub fn build_time_utc() -> Option<u64> {
        option_env!("BUILD_TIME_UTC").and_then(|s| s.parse::<u64>().ok())
    }

    /// Converts escaped `\\n` sequences back into actual newline characters (`\n`).
    ///
    /// This function replaces all occurrences of `\\n` in a string with `\n`,
    /// ensuring that environment variables or serialized data containing escaped
    /// newlines are correctly interpreted as multi-line text.
    ///
    /// ## Behavior
    ///
    /// - The conversion is **intentionally normalized** to Unix-style line endings (`\n`),
    ///   even when executed on Windows. This provides **cross-platform consistency**
    ///   while ensuring predictable behavior across different environments.
    /// - **Memory Safety Consideration**: The resulting string is **boxed and leaked**
    ///   to extend its lifetime (`'static`). This prevents ownership issues when
    ///   returning a borrowed reference.
    ///
    /// ## Platform Testing
    ///
    /// This function has been tested across multiple platforms to ensure consistent behavior.
    /// See the CI configuration for test coverage:
    ///
    /// - [GitHub Actions CI Workflow](https://github.com/jzombie/rust-cargo-pkg-info-struct-builder/blob/main/.github/workflows/rust-tests.yml)
    /// - `test_custom_vars` in the integration test suite:
    ///   [Test Source](https://github.com/jzombie/rust-cargo-pkg-info-struct-builder/blob/main/cargo-pkg-info-test-app/tests/test_pkg_info.rs)
    fn unescape_newlines(s: &str) -> &'static str {
        Box::leak(s.replace("\\n", "\n").into_boxed_str())
    }

    /// Splits a multi-line environment variable into a vector of lines.
    ///
    /// This function takes the output of `option_env!()` (i.e., `Option<&'static str>`)
    /// and performs the following steps:
    ///
    /// 1. If `env_data` is `None`, it returns `None`.
    /// 2. If `env_data` is `Some(...)`, it first applies `unescape_newlines(...)`
    ///    to convert escaped newline sequences back into real newlines.
    /// 3. The unescaped string is then split into lines using `.lines()`.
    /// 4. Finally, it returns `Some(Vec<&'static str>)` where each element represents
    ///    a line from the multi-line environment variable.
    ///
    /// # Arguments
    ///
    /// * `env_data` - An `Option<&'static str>` containing the raw environment variable data.
    ///   - This is typically the result of `option_env!("VAR_NAME")`.
    ///
    /// # Returns
    ///
    /// * `Some(Vec<&'static str>)` if the environment variable exists and contains multiple lines.
    /// * `None` if the environment variable is not set.
    /// ```
    #[allow(dead_code)]
    pub fn split_multi_line_custom_var(env_data: Option<&'static str>) -> Option<Vec<String>> {
        env_data.map(|data| {
            let unescaped = unescape_newlines!(data); // Macro expands into a String
            unescaped.lines().map(String::from).collect() // Convert each line into an owned String
        })
    }
}
