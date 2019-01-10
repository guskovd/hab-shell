pkg_name=sample-repo
pkg_origin=guskovd
pkg_version=1.0.0
pkg_description="Habitat.sh shell sample"
pkg_maintainer='Guskovd'
pkg_upstream_url="https://github.com/guskovd/hab-shell"

pkg_deps=(
    core/sudo
    core/bash
    core/coreutils
    core/grep
    core/gawk
    core/findutils
    core/which
    core/python
)

do_build() {
    return 0
}

do_install() {
    return 0
}

do_shell() {

    # create virtualenv:
    
    venvs_path="$(realpath ~)/.virtualenvs"
    mkdir -p $venvs_path
    
    venv="$venvs_path/sample-repo"
    
    if [ ! -d $venv ]; then
    	python -m venv $venv
    fi
    
    . $venv/bin/activate

    . ~/.bashrc # load you .bashrc
}
