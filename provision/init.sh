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
fi

