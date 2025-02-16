use std::sync::Arc;

const NO_ENV_FALLBACK: &str = "N/A";

/// Reference: <https://doc.rust-lang.org/cargo/reference/environment-variables.html>
/// This struct provides access to package metadata set by Cargo at compile time,
/// including versioning, authors, license information, and build details.
pub struct CargoPkgInfo {
    app_name: Arc<str>,
    crate_name: Arc<str>,
    app_version: Arc<str>,
    version_major: Arc<str>,
    version_minor: Arc<str>,
    version_patch: Arc<str>,
    version_pre: Arc<str>,
    authors: Arc<str>,
    description: Arc<str>,
    homepage: Arc<str>,
    repository: Arc<str>,
    license: Arc<str>,
    license_content: Arc<str>,
    rust_version: Arc<str>,
    readme_path: Arc<str>,
    build_target: Arc<str>,
}

impl CargoPkgInfo {
    /// Creates a new `CargoPkgInfo` instance with values set from environment variables.
    pub fn new() -> Self {
        Self {
            app_name: option_env!("CARGO_PKG_NAME")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            crate_name: option_env!("CARGO_CRATE_NAME")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            app_version: option_env!("CARGO_PKG_VERSION")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            version_major: option_env!("CARGO_PKG_VERSION_MAJOR")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            version_minor: option_env!("CARGO_PKG_VERSION_MINOR")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            version_patch: option_env!("CARGO_PKG_VERSION_PATCH")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            version_pre: option_env!("CARGO_PKG_VERSION_PRE")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            authors: option_env!("CARGO_PKG_AUTHORS")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            description: option_env!("CARGO_PKG_DESCRIPTION")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            homepage: option_env!("CARGO_PKG_HOMEPAGE")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            repository: option_env!("CARGO_PKG_REPOSITORY")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            license: option_env!("CARGO_PKG_LICENSE")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            license_content: option_env!("CARGO_PKG_LICENSE_CONTENT")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            rust_version: option_env!("CARGO_PKG_RUST_VERSION")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            readme_path: option_env!("CARGO_PKG_README")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
            build_target: option_env!("BUILD_TARGET")
                .unwrap_or(NO_ENV_FALLBACK)
                .into(),
        }
    }

    /// Returns the application name.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.app_name().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    /// Returns the crate name.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.crate_name().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }

    /// Returns the full application version.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.app_version().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn app_version(&self) -> &str {
        &self.app_version
    }

    /// Returns the major version of the application.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.version_major().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn version_major(&self) -> &str {
        &self.version_major
    }

    /// Returns the minor version of the application.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.version_minor().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn version_minor(&self) -> &str {
        &self.version_minor
    }

    /// Returns the patch version of the application.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.version_patch().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn version_patch(&self) -> &str {
        &self.version_patch
    }

    /// Returns the pre-release version of the application.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(info.version_pre().len() >= 0);
    /// ```
    #[allow(dead_code)]
    pub fn version_pre(&self) -> &str {
        &self.version_pre
    }

    /// Returns the authors of the package.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.authors().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn authors(&self) -> &str {
        &self.authors
    }

    /// Returns the description of the package.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.description().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the homepage URL of the package.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.homepage().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn homepage(&self) -> &str {
        &self.homepage
    }

    /// Returns the repository URL of the package.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.repository().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn repository(&self) -> &str {
        &self.repository
    }

    /// Returns the license type of the package.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.license().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn license(&self) -> &str {
        &self.license
    }

    /// Returns the contents of the license file (embedded at build time).
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(info.license_content().len() > 0);
    /// ```
    #[allow(dead_code)]
    pub fn license_content(&self) -> &str {
        &self.license_content
    }

    /// Returns the Rust version required by the package.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.rust_version().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn rust_version(&self) -> &str {
        &self.rust_version
    }

    /// Returns the path to the README file.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.readme_path().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn readme_path(&self) -> &str {
        &self.readme_path
    }

    /// Returns the build target (architecture/platform).
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(!info.build_target().is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn build_target(&self) -> &str {
        &self.build_target
    }
}
