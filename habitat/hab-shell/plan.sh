. ../../plan.sh

pkg_name=hab-shell
pkg_version=pkg_version=_set_from_cargo_

pkg_build_deps=(
    core/bash
    core/gawk
    core/grep
    core/git
    core/gcc
    core/make
    core/clang
    core/openssl
    core/pkg-config
    guskovd/rust-nightly
)

pkg_deps=(
    core/sudo
    core/hab
    core/libsodium
    core/libarchive
    core/gcc-libs
)

pkg_bin_dirs=(bin)

update_pkg_version() {
    pkg_version=$(cargo pkgid | awk -F '#' '{print $2}')
    pkg_artifact="$HAB_CACHE_ARTIFACT_PATH/${pkg_origin}-${pkg_name}-${pkg_version}-${pkg_release}-${pkg_target}.${_artifact_ext}"
    pkg_dirname="$pkg_name-$pkg_version"
    pkg_prefix=$HAB_PKG_PATH/${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}
}

do_build() {
    cp -rf ../../{src,Cargo.toml,Cargo.lock} .
    cargo build --release
    update_pkg_version
}

do_install() {
    cargo install --root "${pkg_prefix}"
}

do_setup_environment() {
    return 0
}
