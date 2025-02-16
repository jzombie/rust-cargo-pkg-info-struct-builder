mod cargo_pkg_info;
use cargo_pkg_info::CargoPkgInfo;

fn main() {
    let cargo_pkg_info = CargoPkgInfo::new();

    println!(
        "{:?} {:?} {:?} {:?}",
        cargo_pkg_info.app_name(),
        cargo_pkg_info.crate_name(),
        cargo_pkg_info.app_version(),
        cargo_pkg_info.build_target()
    );
}
