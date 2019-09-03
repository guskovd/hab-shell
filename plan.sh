pkg_name=hab-shell-repo
pkg_origin=guskovd
pkg_version='1.0.52'
pkg_description="Habitat Nix shell implementation"
pkg_maintainer='Guskovd'
pkg_upstream_url="https://github.com/guskovd/hab-shell"

pkg_hab_shell_interpreter="bash"

RUBY_VERSION=2.5.1

pkg_deps=(
    core/bash/4.4.19/20190115012619
    core/clang/7.0.1/20190226032334
    core/docker/18.03.0/20190117151003
    core/gawk/4.2.1/20190115012752
    core/gcc-libs/8.2.0/20190115011926
    core/gcc/8.2.0/20190115004042
    core/git/2.20.1/20190223005329
    core/grep/3.1/20190115012541
    core/hab/0.75.0/20190219230858
    core/libarchive/3.3.3/20190209002803
    core/libsodium/1.0.16/20190116014025
    core/make/4.2.1/20190115013626
    core/openssl/1.0.2q/20190115014220
    core/pkg-config/0.29.2/20190115011955
    core/rsync/3.1.2/20190115215406
    core/ruby/2.5.1/20190130035618
    core/sshpass/1.06/20190115233635
    core/sudo/1.8.18p1/20190117185055
    guskovd/rust-nightly/1.34.0-2019-02-26/20190227151059
    guskovd/rust-racer/2.1.19/20190228121750
)

do_shell() {
    local_rust
    
    export PKG_CONFIG_PATH="$(hab pkg path core/libsodium)/lib/pkgconfig:$(hab pkg path core/libarchive)/lib/pkgconfig:$(hab pkg path core/openssl)/lib/pkgconfig"
    export BUNDLE_PATH=$HOME/.hab-shell/ruby/bundle/$RUBY_VERSION

    export PATH="$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )/.hab-shell/bin:$PATH"
    export PATH="$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )/bin:$PATH"
    
    . ~/.bashrc
}

ruby_setup () {
    ruby_bundle_path=$HOME/.hab-shell/ruby/bundle/$RUBY_VERSION
    mkdir -p $ruby_bundle_path

    pushd "$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" > /dev/null
    bundle install --binstubs
    popd > /dev/null
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
