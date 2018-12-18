pkg_name=hab-shell
pkg_origin=guskovd
pkg_version='1.0.51'
pkg_description="Habitat Shell"
pkg_maintainer='Guskovd'
pkg_upstream_url=""

pkg_hab_shell_interpreter="bash"

pkg_deps=(
    core/sudo
    core/hab
    core/bash
    core/gawk
    core/grep
    guskovd/rust-nightly
    core/git
    core/gcc
    core/gcc-libs
    core/make
    core/clang
    core/openssl
    core/pkg-config
    core/libsodium
    core/libarchive
)

do_shell() {
    echo hello
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
    set_runtime_env HAB_SHELL_INTERPRETER "bash"
}
