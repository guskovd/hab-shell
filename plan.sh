pkg_name=hab-shell-repo
pkg_origin=guskovd
pkg_version='1.0.52'
pkg_description="Habitat Nix shell implementation"
pkg_maintainer='Guskovd'
pkg_upstream_url="https://github.com/guskovd/hab-shell"

pkg_hab_shell_interpreter="bash"

RUBY_VERSION=2.5.1

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
    core/ruby/$RUBY_VERSION
    core/docker
    core/rsync
    core/sshpass
    guskovd/rust-nightly
    guskovd/rust-racer
    guskovd/rustup
)

do_shell() {
    export PKG_CONFIG_PATH="$(hab pkg path core/libsodium)/lib/pkgconfig:$(hab pkg path core/libarchive)/lib/pkgconfig:$(hab pkg path core/openssl)/lib/pkgconfig"

    ruby_bundle_path=$HOME/.hab-shell/ruby/bundle/$RUBY_VERSION

    mkdir -p $ruby_bundle_path
    export BUNDLE_PATH=$ruby_bundle_path

    pushd "$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" > /dev/null
    bundle install --binstubs > /dev/null
    popd > /dev/null

    export PATH="$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )/.hab-shell/bin:$PATH"
    export PATH="$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )/bin:$PATH"
    
    . ~/.bashrc
}

local_rust() {
    export CARGO_HOME=$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )/.cargo
    
    plan_path="$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
    mkdir -p $plan_path/.rust
    commit=$(rustc -v --version | grep commit-hash | awk '{print $2}')
    if [[ ! -d $plan_path/.rust/rust-$commit ]]; then
	wget https://github.com/rust-lang/rust/archive/${commit}.zip -O /tmp/${commit}.zip
	unzip -qq /tmp/${commit}.zip -d $plan_path/.rust
    fi
    export RUST_SRC_PATH=$plan_path/.rust/rust-$commit/src/
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
