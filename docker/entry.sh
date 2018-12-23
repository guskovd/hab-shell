#!/usr/bin/env sh
# -*- coding: utf-8 -*-

mkdir -p /opt/qemu/{sock,images,volumes} 

/usr/bin/qemu-system-x86_64 \
    -vga std \
    -daemonize \
    -nodefaults \
    --enable-kvm \
    -m 2048 \
    -spice port=5930,addr=0.0.0.0,disable-ticketing,seamless-migration=on \
    -vnc 0.0.0.0:0 \
    -chardev socket,path=/opt/qemu/sock/${QEMU_IMG}.qga.sock,server,nowait,id=qga0 \
    -device virtio-serial \
    -device virtserialport,chardev=qga0,name=org.qemu.guest_agent.0 \
    -monitor unix:/opt/qemu/sock/${QEMU_IMG}.monitor.sock,server,nowait \
    -drive file=/opt/qemu/images/${QEMU_IMG}.qcow2,index=0,snapshot=on \
    -netdev user,id=net0,hostfwd=tcp::22-:22,hostfwd=tcp::873-:873,hostfwd=tcp::3389-:3389,hostfwd=tcp::4444-:4444,hostfwd=tcp::5555-:5555 \
    -device virtio-net-pci,netdev=net0 && \
    tail -f /dev/null
