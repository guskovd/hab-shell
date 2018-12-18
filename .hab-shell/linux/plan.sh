pkg_name=hab-shell-repo
pkg_origin=guskovd
pkg_version='1.0.52'
pkg_description="Habitat Nix shell implementation"
pkg_maintainer='Guskovd'
pkg_upstream_url="https://github.com/guskovd/hab-shell"

pkg_hab_shell_interpreter="bash"

pkg_deps=(
    core/sudo
    core/hab
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

do_shell() {
    export PKG_CONFIG_PATH="$(hab pkg path core/libsodium)/lib/pkgconfig:$(hab pkg path core/libarchive)/lib/pkgconfig:$(hab pkg path core/openssl)/lib/pkgconfig"
    . ~/.bashrc
}

do_build() {
    return 0
}

do_install() {
    return 0
}

do_setup_environment() {
    push_runtime_env LD_LIBRARY_PATH "$(pkg_path_for core/gcc-libs)/lib"
    push_runtime_env LD_LIBRARY_PATH "$(pkg_path_for core/libsodium)/lib"
    push_runtime_env LD_LIBRARY_PATH "$(pkg_path_for core/libarchive)/lib"
    push_runtime_env LD_LIBRARY_PATH "$(pkg_path_for core/openssl)/lib"
}
