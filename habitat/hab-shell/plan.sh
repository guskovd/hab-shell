. ../../plan.sh

pkg_name=hab-shell
pkg_version='0.1.1'

pkg_build_deps=(
    core/bash
    core/gawk
    core/grep
    core/git
    core/gcc
    core/gcc-libs
    core/make
    core/clang
    core/openssl
    core/pkg-config
    core/libsodium
    core/libarchive
    guskovd/rust-nightly
)

pkg_deps=(
    core/sudo
    core/hab
)

pkg_bin_dirs=(bin)

do_build() {
    cp -rf ../../{src,Cargo.toml,Cargo.lock} .
    cargo build --release
}

do_install() {
    cargo install --root "${pkg_prefix}"
}
