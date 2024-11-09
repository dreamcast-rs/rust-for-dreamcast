#![no_main]
extern crate kos;
use std::path::Path;
use std::fs::{File, read_dir};
use std::io::Read;

fn list_dir_contents(dir_path: &Path) {
    if let Ok(entries) = read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    println!("Directory: {}", path.display());
                    list_dir_contents(&path);
                } else {
                    println!("File: {}", path.display());
                }
            }
        }
    } else {
        eprintln!("Error: Unable to read the directory {:?}", dir_path);
    }
}

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hello, world from Rust! - filesystem-io example");

    println!("Printing all dirs in the root of the filesystem...");
    let root_dir = Path::new("/");
    list_dir_contents(root_dir);

    println!("\nNow let's try to read the file abstract.txt from an inserted disc...");

    let mut file = File::open("/cd/abstract.txt").expect("Oh, no! File not found!");

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read file! Oops!");

    // Print the contents of the file
    println!("{}", contents);

    println!("Bye!");

    0
}
