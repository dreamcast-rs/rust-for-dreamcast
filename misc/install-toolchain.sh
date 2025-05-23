#!/bin/bash
# Rust for KallistiOS/Dreamcast
# Copyright (C) 2024 Eric Fradella
# https://dreamcast.rs/

KOS_DC_CHAIN_DIR="/opt/toolchains/dc/rust/kos/utils/dc-chain"
KOS_DC_CHAIN_OUTPUT_DIR="/opt/toolchains/dc/rust/sh-elf"
KOS_DC_CHAIN_CONFIG_FILE="/opt/toolchains/dc/rust/misc/dc-chain/rust-toolchain.cfg"
KOS_DC_CHAIN_PATCH_FILE="/opt/toolchains/dc/rust/misc/dc-chain/gcc-rustc-kos.diff"
KOS_DC_CHAIN_PROFILE="/opt/toolchains/dc/rust/misc/dc-chain/profile.rustc-dev.mk"

# Get makejobs argument.
for arg in "$@"; do
    if [[ $arg == -j* ]]; then
        KOS_DC_CHAIN_MAKEJOBS="makejobs=${arg:2}"
    fi
done

KOS_DC_CHAIN_RCG_FLAGS="config_file=${KOS_DC_CHAIN_CONFIG_FILE} ${KOS_DC_CHAIN_MAKEJOBS} sh_toolchain_path=${KOS_DC_CHAIN_OUTPUT_DIR}"

set -e

echo -e "\033[1;32mRust for KallistiOS/Dreamcast Toolchain Installer\033[0m\n"

echo -e "\033[1;31m[1/4]\033[0m Checking prerequisites..."

if [ ! -d "${KOS_DC_CHAIN_DIR}" ]; then
    echo "A KallistiOS dc-chain directory at ${KOS_DC_CHAIN_DIR} was not found."
    echo " Please see the Rust-for-Dreamcast installation directions for cloning KallistiOS."
    exit 1
fi

if [ -d "${KOS_DC_CHAIN_OUTPUT_DIR}" ]; then
    echo "There already exists a directory at ${KOS_DC_CHAIN_OUTPUT_DIR}."
    echo " Please remove or rename this directory before building a new toolchain."
    exit 1
fi

cp ${KOS_DC_CHAIN_PATCH_FILE} ${KOS_DC_CHAIN_DIR}/patches/
cp ${KOS_DC_CHAIN_PROFILE} ${KOS_DC_CHAIN_DIR}/profiles/

echo -e "\033[1;31m[2/4]\033[0m Downloading sh-elf toolchain prerequsites..."
make -C ${KOS_DC_CHAIN_DIR} ${KOS_DC_CHAIN_RCG_FLAGS} fetch-sh4

echo -e "\033[1;31m[3/4]\033[0m Patching sh-elf toolchain components..."
make -C ${KOS_DC_CHAIN_DIR} ${KOS_DC_CHAIN_RCG_FLAGS} patch-sh4

echo -e "\033[1;31m[4/4]\033[0m Installing sh-elf toolchain..."
make -C ${KOS_DC_CHAIN_DIR} ${KOS_DC_CHAIN_RCG_FLAGS} build-sh4

echo -e "\n\033[1;32msh-elf toolchain installed!\033[0m"
