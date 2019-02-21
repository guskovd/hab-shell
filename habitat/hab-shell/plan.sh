pkg_origin=guskovd
pkg_name=hab-shell
pkg_version=pkg_version=_set_from_cargo_
pkg_description="Habitat Nix shell implementation"
pkg_maintainer='Guskovd'
pkg_upstream_url="https://github.com/guskovd/hab-shell"

pkg_build_deps=(
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
    core/rust
    core/gcc
)

pkg_deps=(
    core/sudo
    core/hab
    core/docker
)

pkg_bin_dirs=(bin)

do_prepare() {
    build_type="--release"
    export rustc_target="x86_64-unknown-linux-musl"
    build_line "Setting rustc_target=$rustc_target"

    la_ldflags="-L$(pkg_path_for zlib-musl)/lib -lz"
    la_ldflags="$la_ldflags -L$(pkg_path_for xz-musl)/lib -llzma"
    la_ldflags="$la_ldflags -L$(pkg_path_for bzip2-musl)/lib -lbz2"
    la_ldflags="$la_ldflags -L$(pkg_path_for openssl-musl)/lib -lssl -lcrypto"
    
    export LIBARCHIVE_LIB_DIR=$(pkg_path_for libarchive-musl)/lib
    export LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for libarchive-musl)/include
    export LIBARCHIVE_LDFLAGS="$la_ldflags"
    export LIBARCHIVE_STATIC=true
    export OPENSSL_LIB_DIR=$(pkg_path_for openssl-musl)/lib
    export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl-musl)/include
    export OPENSSL_STATIC=true
    export SODIUM_LIB_DIR=$(pkg_path_for libsodium-musl)/lib
    export SODIUM_STATIC=true

    export LD_LIBRARY_PATH=$(pkg_path_for gcc)/lib
    build_line "Setting LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
}

update_pkg_version() {
    pkg_version=$(cargo pkgid | awk -F '#' '{print $2}')
    pkg_artifact="$HAB_CACHE_ARTIFACT_PATH/${pkg_origin}-${pkg_name}-${pkg_version}-${pkg_release}-${pkg_target}.${_artifact_ext}"
    pkg_dirname="$pkg_name-$pkg_version"
    pkg_prefix=$HAB_PKG_PATH/${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}
}

do_build() {
    cp -rf ../../{src,Cargo.toml,Cargo.lock} .
    cargo build --target=$rustc_target --release
    update_pkg_version
}

do_install() {
    install -v -D target/$rustc_target/${build_type#--}/hab-shell "$pkg_prefix"/bin/hab-shell
    ln -sf ${pkg_prefix}/bin/hab-shell ${pkg_prefix}/bin/hs
}
