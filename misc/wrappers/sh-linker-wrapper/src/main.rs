// Link wrapper script for SuperH with rustc_codegen_gcc
// Since SuperH is not a currently existing platform, we
// specify MIPS architecture in the target json config,
// then use this wrapper after the fact to rewrite the
// ELF header in each compiled object to the SuperH byte

use std::env::args;
use std::io::{Error, Read, Seek, SeekFrom, Write};
use std::fs::OpenOptions;
use std::process::{Command, exit};

const ARCH_OFFSET: u64 = 0x12;
const MIPS_ID: u8 = 0x08;
const SH_ID: u8 = 0x2A;

// Open object file and fix header from MIPS to SH
fn fix_header(path: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;

    // Read architecture ID from file
    file.seek(SeekFrom::Start(ARCH_OFFSET))?;
    let mut byte = [0u8; 1];
    file.read_exact(&mut byte)?;

    // If the byte indicates a MIPS header, rewrite to SH
    if byte[0] == MIPS_ID {
        file.seek(SeekFrom::Start(ARCH_OFFSET))?;
        file.write_all(&[SH_ID])?;
    }

    Ok(())
}

fn main() {
    let mut linker = Command::new("kos-cc");
    for arg in args().skip(1) {
        if arg.ends_with(".o") {
            if let Err(error) = fix_header(&arg) {
                eprintln!("{}", error);
                exit(-1);
            };
        }
        linker.arg(arg);
    }

    let output = linker.output().expect("Failed to run kos-cc");

    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        exit(output.status.code().unwrap());
    }
}
