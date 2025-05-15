# Sega Dreamcast Toolchains Maker (dc-chain)
# This file is part of KallistiOS.

# Toolchain versions for SH
sh_binutils_ver=2.44
sh_gcc_ver=rustc
newlib_ver=4.5.0.20241231
gdb_ver=16.2

# Overide SH toolchain download type
sh_gcc_download_type=git
sh_gcc_git_repo=https://github.com/dreamcast-rs/gcc.git
sh_gcc_git_branch=master

# Toolchain for ARM
# The ARM version of gcc/binutils is separated as support for the ARM7DI core
# used in the Dreamcast's AICA is not available in versions of GCC beyond 8.5.0.
arm_binutils_ver=2.44
arm_gcc_ver=8.5.0

# GCC custom dependencies
# Specify here if you want to use custom GMP, MPFR and MPC libraries when
# building GCC. It is recommended that you leave this variable commented, in
# which case these dependencies will be automatically downloaded by using the
# '/contrib/download_prerequisites' shell script provided within the GCC packages.
# The ISL dependency isn't mandatory; if desired, you may comment the version
# numbers (i.e. 'sh_isl_ver' and 'arm_isl_ver') to disable the ISL library.
#use_custom_dependencies=1

# GCC dependencies for SH
sh_gmp_ver=6.2.1
sh_mpfr_ver=4.1.0
sh_mpc_ver=1.2.1
sh_isl_ver=0.24

# GCC dependencies for ARM
arm_gmp_ver=6.1.0
arm_mpfr_ver=3.1.4
arm_mpc_ver=1.0.3
arm_isl_ver=0.18
