#!/usr/bin/env sh
# -*- coding: utf-8 -*-

export TEMP=/tmp
HAB_PKG="core/hab/0.70.0"

if [[ "$(uname -s)" == "Linux" ]]; then # Linux setup
    hab pkg path $HAB_PKG || sudo hab pkg install $HAB_PKG
    sudo hab pkg binlink core/hab -d /usr/bin/ --force
else # Windows setup
    cp -rf provision/.hab/cache c:/hab -rf
    cp -rf provision/.hab/etc c:/hab -rf
    hab pkg path $HAB_PKG || hab pkg install $HAB_PKG
    hab pkg install core/rust
    cp -rf {src,Cargo.toml,Cargo.lock} habitat/hab-shell
    hab studio build -s . -R habitat/hab-shell
    last_build=habitat/hab-shell/results/last_build.ps1
    pkg_artifact=$(cat $last_build | grep pkg_artifact | awk -F '=' '{print $2}' | sed $'s/[\r:\"]//g')
    hab pkg install habitat/hab-shell/results/$pkg_artifact
fi
