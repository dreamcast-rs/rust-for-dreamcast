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

# rustc_codegen_gcc was compiled in release mode
export CHANNEL="release"

# Warn user if the KOS_SH4_PRECISION setting is set to something other than -m4-single.
# Even though we are not using the value from the environment, we should warn the user
# about a potential mismatch in the configuration.
if [ ! "$KOS_SH4_PRECISION" == "-m4-single" ] && [ ! "$KOS_SH4_PRECISION" == "" ]; then
    echo "! WARNING: Your KOS_SH4_PRECISION environment variable is not set to -m4-single!"
fi

# Add desired flags for rustc to pass to GCC here
CG_GCCFLAGS="-m4-single -ml -ffunction-sections -fdata-sections -matomic-model=soft-imask -ftls-model=local-exec $CG_GCCFLAGS"

# This will format and add the GCC flags to the CG_RUSTFLAGS for you
# These will in turn be added to invocations of cargo and rustc
for flag in $CG_GCCFLAGS; do
    addflags+="-Cllvm-args=$flag "
done
export CG_RUSTFLAGS="$addflags $CG_RUSTFLAGS"
