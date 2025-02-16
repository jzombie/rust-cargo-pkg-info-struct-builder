
# cargo-pkg-info-struct-builder

A build-time crate that generates a struct (`CargoPkgInfo`) to provide easy, structured access to your Rust project’s compile-time Cargo environment variables.

## Available Metadata Methods

Once generated, the `CargoPkgInfo` struct provides access to the following metadata:

| **Method**                           | **Description**                                      |
|--------------------------------------|------------------------------------------------------|
| `CargoPkgInfo::pkg_name()`           | Package name (`CARGO_PKG_NAME`)                      |
| `CargoPkgInfo::crate_name()`         | Crate name (`CARGO_CRATE_NAME`)                      |
| `CargoPkgInfo::pkg_version()`        | Full version (`CARGO_PKG_VERSION`)                   | 
| `CargoPkgInfo::version_major()`      | Major version (`CARGO_PKG_VERSION_MAJOR`)            |
| `CargoPkgInfo::version_minor()`      | Minor version (`CARGO_PKG_VERSION_MINOR`)            |
| `CargoPkgInfo::version_patch()`      | Patch version (`CARGO_PKG_VERSION_PATCH`)            |
| `CargoPkgInfo::version_pre()`        | Pre-release version (`CARGO_PKG_VERSION_PRE`)        |
| `CargoPkgInfo::authors()`            | Authors (`CARGO_PKG_AUTHORS`)                        |
| `CargoPkgInfo::description()`        | Description (`CARGO_PKG_DESCRIPTION`)                |
| `CargoPkgInfo::homepage()`           | Homepage URL (`CARGO_PKG_HOMEPAGE`)                  |
| `CargoPkgInfo::repository()`         | Repository URL (`CARGO_PKG_REPOSITORY`)              |
| `CargoPkgInfo::license()`            | License name (`CARGO_PKG_LICENSE`)                   |
| `CargoPkgInfo::license_content()`    | Full license text (`CARGO_PKG_LICENSE_FILE`)         |
| `CargoPkgInfo::rust_version()`       | Required Rust version (`CARGO_PKG_RUST_VERSION`)     |
| `CargoPkgInfo::readme_path()`        | Path to README file (`CARGO_PKG_README`)             |
| `CargoPkgInfo::build_target()`       | Compilation target (`BUILD_TARGET`)                  |
| `CargoPkgInfo::build_time_utc()`     | Build timestamp UTC (`BUILD_TIME_UTC`)               |

---

## Overview

At compile time, Cargo sets various environment variables describing your project (version, authors, license, etc.). `cargo-pkg-info-struct-builder` gathers those variables from your project (not this library) and creates a file, at the path of your choosing, containing a `CargoPkgInfo` struct.

This struct provides simple, typed methods to retrieve metadata like:

- Version (major, minor, patch, pre-release)
- Authors, description, homepage, and repository
- License information (including file contents)
- Build target and a compile-time build timestamp

Because this happens during the project’s build process, you get project-wide metadata embedded in the final binary—no additional steps required.

## Quick Start

Add it as a **build dependency**:

```sh
cargo add cargo-pkg-info-struct-builder --build
```

Then, in your **`build.rs`**:

```rust
use cargo_pkg_info_struct_builder::inject_build_metadata;
use std::path::Path;

fn main() {
    // Generate metadata into a Rust file.
    // You can change the path of your choosing, and directories are
    // auto-generated if they do not already exist.
    let dest_path = Path::new("src").join("cargo_pkg_info.rs");
    inject_build_metadata(dest_path.to_path_buf());
}
```

Now, in your main application:

```rust
// Assuming the `dest_path` is used from the above example.
mod cargo_pkg_info;
use cargo_pkg_info::CargoPkgInfo;

fn main() {
    println!(
        "{} v{} (Built on {})",
        CargoPkgInfo::pkg_name(),
        CargoPkgInfo::pkg_version(),
        CargoPkgInfo::build_time_utc().unwrap_or(0),
    );
}
```

**This embeds project metadata into your binary at compile time—no runtime environment reads needed!**

## Why Compile-Time Injection?

Unlike crates that retrieve package metadata **at runtime**, this crate:

- **Embeds metadata at compile time** to avoid file I/O in production.
- **Captures the metadata of your project**, not the dependency itself.
- **Works with any Rust project** (including WASM) without requiring Cargo to be installed at runtime.

## How It Works

When `build.rs` runs:

1. It reads package metadata from Cargo environment variables.
2. It generates a Rust source file (`cargo_pkg_info.rs`) containing a struct with methods that access this metadata.
3. The file is compiled into your project, allowing **runtime access to compiled environment info.**

This is ideal for **logging, debugging, and version tracking** in Rust applications.

## Notes
- If an environment variable is missing, it **falls back to `"N/A"`** instead of panicking.
- **Numeric fields** like `build_time_utc()` return `Option<u64>` instead of a string.
- The generated file **can be committed to version control**, but it will remain **unchanged unless the template itself is modified.** Metadata updates do not change the file itself.

## License
Licensed under **MIT**. See [`LICENSE`](https://github.com/jzombie/rust-cargo-pkg-info-struct-builder/blob/main/LICENSE) for details.
