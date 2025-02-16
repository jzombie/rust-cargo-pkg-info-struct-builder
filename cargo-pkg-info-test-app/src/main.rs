mod cargo_pkg_info;
use cargo_pkg_info::CargoPkgInfo;

fn main() {
    println!(
        "{:?} {:?} {:?} {:?} {:?}",
        CargoPkgInfo::pkg_name(),
        CargoPkgInfo::crate_name(),
        CargoPkgInfo::pkg_version(),
        CargoPkgInfo::build_target(),
        CargoPkgInfo::build_time_utc(),
    );
}
