#!/bin/bash

# Check for needed programs for our scripts
if ! command -v jq &> /dev/null
then
    echo "jq is not installed. Please install jq to use this script."
    exit 1
fi

if ! command -v xxd &> /dev/null
then
    echo "xxd is not installed. Please install xxd to use this script."
    exit 1
fi

# If user does not override $CG_GCC_JIT_DIR in their environment,
# we'll use the standard path for rustc_codegen_gcc.
if [ "$CG_GCCJIT_DIR" = "" ]; then
    export CG_GCCJIT_DIR="/opt/toolchains/dc/rust/rustc_codegen_gcc";
fi;

# Get the path to our cross-compiling libgccjit library
if [ -e "$CG_GCCJIT_DIR/config.toml" ]; then
    export CG_GCCJIT_GCC_PATH=$(grep -oP '^gcc-path = \"\K[^\"]+' $CG_GCCJIT_DIR/config.toml)
else
    echo "config.toml file with gcc-path not found in $CG_GCCJIT_DIR!"
    exit 1
fi

# Add our toolchain to PATH for the duration of this script
export PATH="$CG_GCCJIT_GCC_PATH/../bin:$PATH"

# Use -m4-single precision as -m4-single-only is incompatible
export CG_GCCJIT_SH4_PRECISION="$KOS_SH4_PRECISION"

export CHANNEL="release"
