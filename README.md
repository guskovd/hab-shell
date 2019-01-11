# Welcome to hab-shell - Habitat Nix shell implementation.

## Installation via habitat.sh:

```bash
sudo hab pkg install guskovd/hab-shell
```

## Create binkink in /usr/local/bin:

```bash
sudo hab pkg binlink guskovd/hab-shell -d /usr/local/bin hs
```

## Usage:

### Create plan.sh containing:

```bash
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
    
    venv="$venvs_path/gsf"
    
    if [ ! -d $venv ]; then
    	python -m venv $venv
    fi
    
    . $venv/bin/activate

    . ~/.bashrc # load your .bashrc
}

```

### Build
```
hs build
```

### Run it!
```
hs shell
```

### Test:
```
which python
/home/guskov/.virtualenvs/sample-repo/bin/python
```

this example is available in sample subfolder
