. ../../plan.sh

do_build() {
    pushd ../..
    cargo build --release
}

do_install() {
    pushd ../..
    cargo install --root "${pkg_prefix}"
}
