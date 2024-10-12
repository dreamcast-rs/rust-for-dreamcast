#!/bin/bash
# Rust for KallistiOS/Dreamcast
# Copyright (C) 2024 Eric Fradella
# https://dreamcast.rs/

### Nightlies available in dreamcast-rs repo
#RUST_NIGHTLY=2024-03-09
#RUST_NIGHTLY=2024-07-01
RUST_NIGHTLY=2024-08-10

### Nightlies in progress upstream
#RUST_NIGHTLY=2024-10-08

### Use libc/sysroot locally instead of from dreamcast-rs repos
#USE_LOCAL_LIBC=/home/darc/rust-kos/libc
#USE_LOCAL_SYSROOT=/home/darc/rust-kos/nightly-${RUST_NIGHTLY}/rust

set -e

echo -e "\033[1;32mRust for KallistiOS/Dreamcast Installer\033[0m\n"

echo -e "\033[1;31m[1/9]\033[0m Checking prerequisites..."
### Check if KOS environment is sourced
if [ -z "${KOS_BASE}" ]; then
    echo "KallistiOS environment is not sourced. Please source environ.sh before"
    echo " attempting to build and install support for Rust."
    exit 1
fi

### Check if Rust-for-Dreamcast variables are set
if [ -z "${KOS_RUST_BASE}" ]; then
    echo "Rust-for-Dreamcast environment is not sourced. Please source"
    echo " environ.sh before attempting to build and install support for Rust."
    exit 1
fi

### Check for libgccjit.so
if [ ! -e "${KOS_CC_BASE}/lib/libgccjit.so" ]; then
    echo "libgccjit.so not found in sh-elf toolchain. Please use dc-chain to"
    echo " install a toolchain using the rustc-dev profile with libgccjit enabled."
    exit 1
fi

### Clone the rustc_codegen_gcc repo to $KOS_RCG_BASE
echo -e "\033[1;31m[2/9]\033[0m Cloning rustc_codegen_gcc repository..."
rm -rf ${KOS_RUST_BASE}/rustc_codegen_gcc
mkdir -p ${KOS_RUST_BASE}
mkdir -p ${KOS_RCG_BASE}
case "${RUST_NIGHTLY}" in
    "2024-10-08" )
        git clone https://github.com/rust-lang/rustc_codegen_gcc.git -b sync_from_rust_2024_10_08 ${KOS_RCG_BASE}
        ;;
    "2024-03-09" | "2024-07-01" | "2024-08-10" | * )
        git clone https://github.com/dreamcast-rs/rustc_codegen_gcc.git -b ${RUST_NIGHTLY} ${KOS_RCG_BASE}
        ;;
esac

### Clone the libc repo to ${KOS_RCG_BASE}/libc
echo -e "\033[1;31m[3/9]\033[0m Cloning Rust libc crate with KallistiOS support..."
rm -rf ${KOS_RUST_BASE}/libc
if [ -n "${USE_LOCAL_LIBC}" ]; then
    cp -R ${USE_LOCAL_LIBC} ${KOS_RUST_BASE}/libc
else
    git clone --depth=1 https://github.com/dreamcast-rs/libc -b libc-0.2-kos ${KOS_RUST_BASE}/libc
    rm -rf ${KOS_RUST_BASE}/libc/.git
fi

### Clone the rust repo to $KOS_RCG_BASE/kos-rust
echo -e "\033[1;31m[4/9]\033[0m Cloning Rust sysroot with KallistiOS support..."
rm -rf ${KOS_RUST_BASE}/sysroot
if [ -n "${USE_LOCAL_SYSROOT}" ]; then
    cp -R ${USE_LOCAL_SYSROOT} ${KOS_RUST_BASE}/sysroot
else
    git clone --depth=1 https://github.com/dreamcast-rs/rust -b kos-${RUST_NIGHTLY} ${KOS_RUST_BASE}/sysroot
    git -C ${KOS_RUST_BASE}/sysroot submodule update --init library/stdarch
    git -C ${KOS_RUST_BASE}/sysroot submodule update --init library/backtrace
fi

### Enter rustc_codegen_gcc dir
pushd ${KOS_RCG_BASE} > /dev/null

### Apply any patches and adjustments, if necessary
echo -e "\033[1;31m[5/9]\033[0m Applying patches and adjustments..."
### Remove extra file that rustc_codegen_gcc scripts will patch into existence
rm -f ${KOS_RUST_BASE}/sysroot/library/stdarch/Cargo.toml
### Write GCC path to rustc_codegen_gcc config
echo "gcc-path = \"${KOS_CC_BASE}/lib\"" > ${KOS_RCG_BASE}/config.toml

### Install sh-linker-wrapper
echo -e "\033[1;31m[6/9]\033[0m Building/Installing sh-linker-wrapper..."
pushd ${KOS_RUST_BASE}/misc/sh-linker-wrapper > /dev/null
cargo build --release
cp target/release/sh-linker-wrapper ${KOS_BASE}/utils/build_wrappers/.
popd > /dev/null

### Enter build system dir, build the build_system, and return back to rustc_codegen_gcc dir
echo -e "\033[1;31m[7/9]\033[0m Building rustc_codegen_gcc build system..."
pushd ${KOS_RCG_BASE}/build_system > /dev/null
cargo build --release
popd > /dev/null

### Prepare rustc_codegen_gcc using our custom sysroot source
echo -e "\033[1;31m[8/9]\033[0m Running rustc_codegen_gcc cross-compiler preparation stage..."
CG_RUSTFLAGS="${KOS_RCG_RUSTFLAGS}" CHANNEL="release" \
    ${KOS_RCG_CARGO} prepare \
    --cross \
    --sysroot-source ${KOS_RUST_BASE}/sysroot

echo -e "\033[1;31m[9/9]\033[0m Running rustc_codegen_gcc sysroot build stage..."
case "${RUST_NIGHTLY}" in
    "2024-03-09" | "2024-07-01" )
        RCG_FEATURES=master
        ;;
    "2024-08-10" | "2024-10-08" | * )
        RCG_FEATURES=compiler_builtins/no-f16-f128
        ;;
esac

CG_RUSTFLAGS="${KOS_RCG_RUSTFLAGS}" CHANNEL="release" \
    ${KOS_RCG_CARGO} build \
    --sysroot --release --release-sysroot \
    --features ${RCG_FEATURES} \
    --target-triple sh-elf \
    --target ${KOS_RUST_BASE}/misc/sh-elf.json

popd > /dev/null

echo -e "\n\033[1;32mRust for KallistiOS/Dreamcast installed!\033[0m"
