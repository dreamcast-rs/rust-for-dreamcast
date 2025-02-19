### Rust / KallistiOS Build Environment Settings

### Base for Rust toolchain files
export KOS_RUST_BASE="/opt/toolchains/dc/rust"

### Set the expected path to rustc_codegen_gcc.
export KOS_RCG_BASE="${KOS_RUST_BASE}/rustc_codegen_gcc";

### Path to built rustc_codegen_gcc binary
export KOS_RCG_CARGO="${KOS_RCG_BASE}/build_system/target/release/y"

### KOS Root Paths
export KOS_BASE="${KOS_RUST_BASE}/kos"
export KOS_PORTS="${KOS_RUST_BASE}/kos-ports"

### Build Architecture
export KOS_ARCH="dreamcast"
export KOS_SUBARCH="pristine"

### SH Compiler Prefixes
# Specifies the path to and prefix for the main SH
# GCC toolchain used to target the Dreamcast's SH4 CPU.
export KOS_CC_BASE="${KOS_RUST_BASE}/sh-elf"
export KOS_CC_PREFIX="sh-elf"

### ARM Compiler Prefixes
# Specifies the path to and prefix for the additional ARM
# GCC toolchain used to target the Dreamcast's AICA SPU.
export DC_ARM_BASE="/opt/toolchains/dc/arm-eabi"
export DC_ARM_PREFIX="arm-eabi"

### External Dreamcast Tools Path
# Specifies the path where Dreamcast tools that are not part of KOS are to be
# installed. This includes, for example, dc-tool-ip, dc-tool-serial, and the
# mrbc bytecode compiler. This directory, along with SH and ARM compiler
# toolchains, will be added to your PATH environment variable.
export DC_TOOLS_BASE="/opt/toolchains/dc/rust/bin"

### CMake Toolchain Path
# Specifies the path to the toolchain file used to target
# KOS with the "cmake" build utility.
export KOS_CMAKE_TOOLCHAIN="${KOS_BASE}/utils/cmake/dreamcast.toolchain.cmake"

### Genromfs Utility Path
# Specifies the path to the utility which is used by KOS
# to create romdisk filesystem images.
export KOS_GENROMFS="${KOS_BASE}/utils/genromfs/genromfs"

### Make Utility
# Configures the tool to be used as the main "make" utility
# for building GNU Makefiles. On a platform such as BSD,
# the default can be changed to "gmake," for the GNU
# implementation.
export KOS_MAKE="make"
#export KOS_MAKE="gmake"

### Loader Utility
# Specifies the loader to be used with the "make run" targets
# in the KOS examples. Defaults to using a preconfigured version
# of dc-tool. Use one of the other options for a manual dc-tool-ip
# or dc-tool-serial configuration, remembering to change the values
# for the Dreamcast's IP address or the serial port interface.
export KOS_LOADER="dc-tool -x"
#export KOS_LOADER="dc-tool-ip -t 192.168.1.100 -x"
#export KOS_LOADER="dc-tool-ser -t /dev/ttyS0 -x"

### Default Compiler Flags
# Resets build flags. You can also initialize them to some preset
# default values here if you wish.
export KOS_RCG_RUSTFLAGS=""
export KOS_INC_PATHS=""
export KOS_CFLAGS=""
export KOS_CPPFLAGS=""
export KOS_LDFLAGS=""
export KOS_AFLAGS=""
export DC_ARM_LDFLAGS=""

### Rust Flags
# Add optional build flags to pass to rustc here.
# Flags prefixed with "-Cllvm-args=" will be passed through to GCC via rustc_codegen_gcc.
#export KOS_RCG_RUSTFLAGS="${KOS_RCG_RUSTFLAGS}"
#export KOS_RCG_RUSTFLAGS="${KOS_RCG_RUSTFLAGS} -Cllvm-args="

### Debug Builds
# Controls whether to disable additional debugging checks and assertions,
# such as for parameter validation or internal errors. Uncomment this if
# you do not wish to compile with this additional logic enabled.
#export KOS_CFLAGS="${KOS_CFLAGS} -DNDEBUG"

### Optimization Level
# Controls the baseline optimization level to use when building.
# Typically this is -Og (debugging), -O0, -O1, -O2, or -O3.
# NOTE: For our target, -O4 is a valid optimization level that has
# been seen to have some performance impact as well.
export KOS_CFLAGS="${KOS_CFLAGS} -O2"

### Additional Optimizations
# Uncomment this to enable what has been found empirically to be
# the optimal set of additional flags for release build performance
# on the current stable toolchain. NOTE: Certain KOS-ports and examples
# do not work properly with "-flto=auto"!
#export KOS_CFLAGS="${KOS_CFLAGS} -freorder-blocks-algorithm=simple -flto=auto"

### Frame Pointers
# Controls whether frame pointers are emitted or not. Disabled by
# default. Enable them if you plan to use GDB for debugging.
export KOS_CFLAGS="${KOS_CFLAGS} -fomit-frame-pointer"
#export KOS_CFLAGS="${KOS_CFLAGS} -fno-omit-frame-pointer -DFRAME_POINTERS"

### Stack Protector
# Controls whether GCC emits extra code to check for buffer overflows or stack
# smashing, which can be very useful for debugging. -fstack-protector only
# covers vulnerable objects, while -fstack-protector-strong provides medium
# coverage, and -fstack-protector-all provides full coverage. You may also
# override the default stack excepton handler by providing your own
# implementation of "void __stack_chk_fail(void)."
#export KOS_CFLAGS="${KOS_CFLAGS} -fstack-protector-all"

### GCC Builtin Functions
# Uncomment this line to prevent GCC from using its own builtin implementations
# of certain standard library functions.
export KOS_CFLAGS="${KOS_CFLAGS} -fno-builtin"

### Fast Math Instructions
# Comment this line to disable the use of optimized fast-math instructions (FSSRA,
# FSCA) for calculating sin/cos and inverse square root, and any math optimization
# that does not guarantee compliance with the IEEE floating-point standard.
# NOTE: Enabling this option will also override -fno-builtin!
#export KOS_CFLAGS="${KOS_CFLAGS} -fbuiltin -ffast-math -ffp-contract=fast"

### SH4 Floating Point Arithmetic Precision
# Make sure KallistiOS is set up to use "-m4-single" for floating point operations.
# When using Rust, make sure KOS, KOS ports, and any libraries you plan on linking
# to your projects are compiled using -m4-single!
export KOS_SH4_PRECISION="-m4-single"

### Use LRA (Local Register Allocator) Pass
# Uncomment this line to use the modern Local Register Allocator pass during
# code generation instead of the default older reload pass. This option is
# known to be unstable or less performant for SH at this time, but will likely
# become mandatory in future versions of GCC, so feel free to help us test.
# Only enable this setting if you understand what you are doing!
#export KOS_CFLAGS="${KOS_CFLAGS} -mlra"

### Shared Compiler Configuration
# Include sub architecture-independent configuration file for shared
# environment settings. If you want to configure additional compiler
# options or see where other build flags are set, look at this file.
. ${KOS_BASE}/environ_base.sh

### Default KOS_RCG_RUSTFLAGS
# These RUSTFLAGS are required to be enabled and will likely be upstreamed to
# environ_dreamcast.sh in upstream KallistiOS at a later time. Do not modify
# them unless you know exactly what you are doing.
export KOS_RCG_RUSTFLAGS="${KOS_RCG_RUSTFLAGS} -Cllvm-args=${KOS_SH4_PRECISION} -Cllvm-args=-ml"
export KOS_RCG_RUSTFLAGS="${KOS_RCG_RUSTFLAGS} -Cllvm-args=-ffunction-sections -Cllvm-args=-fdata-sections"
export KOS_RCG_RUSTFLAGS="${KOS_RCG_RUSTFLAGS} -Cllvm-args=-matomic-model=soft-imask -Cllvm-args=-ftls-model=local-exec"
