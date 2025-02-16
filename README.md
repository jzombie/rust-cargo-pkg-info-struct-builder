# CargoPkgInfo 
A Rust crate that provides compile-time package metadata, including versioning, authors, license information, and build details, accessible at runtime without additional file I/O.

## Overview
`CargoPkgInfo` provides structured access to package metadata set by Cargo at **compile time**, including:  
✅ Versioning (major, minor, patch, pre-release)  
✅ Authors, description, homepage, and repository  
✅ License information (including file contents)  
✅ Rust version requirements  
✅ Build target and timestamp  

All values are retrieved from **Cargo environment variables**, ensuring no runtime file I/O is required.  

---

## Installation

```bash
cargo add cargo-pkg-info-struct
```

---

## Usage
Retrieve package metadata anywhere in your Rust project:

```rust
use cargo_pkg_info_struct::CargoPkgInfo;

fn main() {
    let info = CargoPkgInfo::new();

    println!("App: {} v{}", info.app_name(), info.app_version());
    println!("Authors: {}", info.authors());
    println!("License: {}", info.license());
    println!("Build Time: {}", info.build_time_local("%Y-%m-%d %H:%M:%S"));
}
```

##  Available Metadata
| **Method**                  | **Description** |
|-----------------------------|---------------|
| `app_name()`                | Package name (`CARGO_PKG_NAME`) |
| `crate_name()`              | Crate name (`CARGO_CRATE_NAME`) |
| `app_version()`             | Full version (`CARGO_PKG_VERSION`) |
| `version_major()`           | Major version (`CARGO_PKG_VERSION_MAJOR`) |
| `version_minor()`           | Minor version (`CARGO_PKG_VERSION_MINOR`) |
| `version_patch()`           | Patch version (`CARGO_PKG_VERSION_PATCH`) |
| `version_pre()`             | Pre-release version (`CARGO_PKG_VERSION_PRE`) |
| `authors()`                 | Authors (`CARGO_PKG_AUTHORS`) |
| `description()`             | Description (`CARGO_PKG_DESCRIPTION`) |
| `homepage()`                | Homepage URL (`CARGO_PKG_HOMEPAGE`) |
| `repository()`              | Repository URL (`CARGO_PKG_REPOSITORY`) |
| `license()`                 | License name (`CARGO_PKG_LICENSE`) |
| `license_content()`         | Full license text (`CARGO_PKG_LICENSE_FILE`) |
| `rust_version()`            | Required Rust version (`CARGO_PKG_RUST_VERSION`) |
| `readme_path()`             | Path to README file (`CARGO_PKG_README`) |
| `build_target()`            | Compilation target (`BUILD_TARGET`) |
| `build_time_local(fmt)`     | Build timestamp (custom format) |

---

## Notes
- **Compile-time efficiency**: No file reads, just environment variable access.  
- **Safe defaults**: If an environment variable is missing, `"N/A"` is returned.  
- **Custom build time format**: Use `build_time_local("%Y-%m-%d %H:%M:%S")`.  

---

## License
Licensed under **MIT**. See [`LICENSE`](./LICENSE) for details.
