. ../../plan.sh

pkg_name=hab-shell
pkg_version='0.1.0'

pkg_bin_dirs=(bin)

do_build() {
    cp -rf ../../{src,Cargo.toml,Cargo.lock} .
    cargo build --release
}

do_install() {
    cargo install --root "${pkg_prefix}"
}
