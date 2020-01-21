pkg_name=hab-shell-repo
pkg_origin=guskovd
pkg_version='1.0.52'
pkg_description="Habitat Nix shell implementation"
pkg_maintainer='Guskovd'
pkg_upstream_url="https://github.com/guskovd/hab-shell"

pkg_hab_shell_interpreter="bash"

RUBY_VERSION=2.5.1

pkg_deps=(
    core/bash
    core/gawk
    core/grep
    core/git
    core/musl
    core/zlib-musl
    core/xz-musl
    core/bzip2-musl
    core/libarchive-musl
    core/openssl-musl
    core/libsodium-musl
    core/coreutils
    core/gcc
    core/pkg-config
    core/rsync
    core/ruby
    core/sshpass
    core/sudo
    core/hab/0.75.0/20190219230858
    guskovd/rustup/1.19.0/20190917154527
    guskovd/rust-racer/2.1.27/20190917152749
)

do_shell() {
    plan_path="$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
    
    export PKG_CONFIG_PATH="$(hab pkg path core/libsodium)/lib/pkgconfig:$(hab pkg path core/libarchive)/lib/pkgconfig:$(hab pkg path core/openssl)/lib/pkgconfig"
    export BUNDLE_PATH=$HOME/.hab-shell/ruby/bundle/$RUBY_VERSION

    export PATH="$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )/.hab-shell/bin:$PATH"
    export PATH="$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )/bin:$PATH"
    export PATH="${plan_path}/.hab-shell/bin:${plan_path}/.cargo/bin:$PATH"

    la_ldflags="-L$(hab pkg path core/zlib-musl)/lib -lz"
    la_ldflags="$la_ldflags -L$(hab pkg path core/xz-musl)/lib -llzma"
    la_ldflags="$la_ldflags -L$(hab pkg path core/bzip2-musl)/lib -lbz2"
    la_ldflags="$la_ldflags -L$(hab pkg path core/openssl-musl)/lib -lssl -lcrypto"
    
    export LIBARCHIVE_LIB_DIR=$(hab pkg path core/libarchive-musl)/lib
    export LIBARCHIVE_INCLUDE_DIR=$(hab pkg path core/libarchive-musl)/include
    export LIBARCHIVE_LDFLAGS="$la_ldflags"
    export LIBARCHIVE_STATIC=true
    export OPENSSL_LIB_DIR=$(hab pkg path core/openssl-musl)/lib
    export OPENSSL_INCLUDE_DIR=$(hab pkg path core/openssl-musl)/include
    export OPENSSL_STATIC=true
    export SODIUM_LIB_DIR=$(hab pkg path core/libsodium-musl)/lib
    export SODIUM_STATIC=true

    export LD_LIBRARY_PATH=$(hab pkg path core/gcc)/lib

    export CARGO_HOME=${plan_path}/.cargo
    export RUSTUP_HOME=${plan_path}/.rustup
    
    . ~/.bashrc

    export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
}

ruby_setup () {
    ruby_bundle_path=$HOME/.hab-shell/ruby/bundle/$RUBY_VERSION
    mkdir -p $ruby_bundle_path

    pushd "$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" > /dev/null
    bundle install --binstubs
    popd > /dev/null
}

rustup_setup() {
    rustup-init -y
    
    rustup update
    rustup component add rls rust-analysis rust-src
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
