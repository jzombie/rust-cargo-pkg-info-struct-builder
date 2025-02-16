use chrono::{DateTime, Local, Utc};
use std::sync::Arc;

const NO_ENV_FALLBACK: &str = "N/A";

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
    build_time_local: Arc<str>,
}

impl CargoPkgInfo {
    /// Creates a new `CargoPkgInfo` instance with values set from environment variables.
    pub fn new() -> Self {
        let build_timestamp_str = option_env!("BUILD_TIME").unwrap_or(NO_ENV_FALLBACK);

        let parsed_build_utc = build_timestamp_str
            .parse::<DateTime<Utc>>()
            .ok()
            .map(|dt| dt.with_timezone(&Local))
            .map(|dt| dt.format("%b %d, %Y %H:%M:%S %Z").to_string())
            .unwrap_or_else(|| NO_ENV_FALLBACK.to_string());

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

    /// Read-only getters
    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }

    /// Returns the full application version.
    pub fn app_version(&self) -> &str {
        &self.app_version
    }

    /// Returns the major version of the application.
    pub fn version_major(&self) -> &str {
        &self.version_major
    }

    /// Returns the minor version of the application.
    pub fn version_minor(&self) -> &str {
        &self.version_minor
    }

    /// Returns the patch version of the application.
    pub fn version_patch(&self) -> &str {
        &self.version_patch
    }

    /// Returns the pre-release version of the application.
    pub fn version_pre(&self) -> &str {
        &self.version_pre
    }

    pub fn authors(&self) -> &str {
        &self.authors
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn homepage(&self) -> &str {
        &self.homepage
    }

    pub fn repository(&self) -> &str {
        &self.repository
    }

    pub fn license(&self) -> &str {
        &self.license
    }

    pub fn license_content(&self) -> &str {
        &self.license_content
    }

    pub fn rust_version(&self) -> &str {
        &self.rust_version
    }

    pub fn readme_path(&self) -> &str {
        &self.readme_path
    }

    pub fn build_target(&self) -> &str {
        &self.build_target
    }

    pub fn build_time_local(&self) -> &str {
        &self.build_time_local
    }
}
