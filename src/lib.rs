use chrono::{DateTime, Local, Utc};
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
    build_time_local: Option<Arc<DateTime<Local>>>,
}

impl CargoPkgInfo {
    /// Creates a new `CargoPkgInfo` instance with values set from environment variables.
    pub fn new() -> Self {
        let parsed_build_utc = option_env!("BUILD_TIME")
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .map(|dt| dt.with_timezone(&Local))
            .map(Arc::new);

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
            build_time_local: parsed_build_utc.into(),
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
    pub fn build_target(&self) -> &str {
        &self.build_target
    }

    /// Returns the local build time in a specified format or `"N/A"` if unavailable.
    ///
    /// # Arguments
    /// * `format` - A format string following `chrono::format` syntax.
    ///
    /// # Example
    /// ```
    /// use cargo_pkg_info_struct::CargoPkgInfo;
    /// let info = CargoPkgInfo::new();
    /// assert!(info.build_time_local("%Y-%m-%d %H:%M:%S").len() > 0);
    /// ```
    pub fn build_time_local(&self, format: &str) -> String {
        match &self.build_time_local {
            Some(time) => time.format(format).to_string(),
            None => NO_ENV_FALLBACK.to_string(), // Return "N/A" if build time isn't set
        }
    }
}
