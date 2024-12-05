// Use the kos_romdisk_dir!() macro to build and include a romdisk in your KallistiOS project.
// When using this macro, make sure "cc" is listed under [build-dependencies] in Cargo.toml, like so:
//
// [build-dependencies]
// cc = "1.0"

macro_rules! kos_romdisk_dir {
    ($path:expr) => {
        println!("cargo:rerun-if-changed={}", $path);
        println!("cargo:rustc-link-lib=static:+whole-archive=romdiskbase");

        let genromfs_cmd = format!("{}/utils/genromfs/genromfs", std::env::var("KOS_BASE").unwrap());
        let romdisk_img_path = format!("{}/romdisk.img", std::env::var("OUT_DIR").unwrap());
        std::process::Command::new(genromfs_cmd)
            .args(&["-f", &romdisk_img_path, "-d", $path])
            .status()
            .expect("Failed to execute genromfs");

        let romdisk_code_path = format!("{}/romdisk_tmp.c", std::env::var("OUT_DIR").unwrap());
        let bin2c_cmd = format!("{}/utils/bin2c/bin2c", std::env::var("KOS_BASE").unwrap());
        std::process::Command::new(bin2c_cmd)
            .args(&[&romdisk_img_path, &romdisk_code_path, "romdisk"])
            .status()
            .expect("Failed to execute bin2c");

        cc::Build::new()
            .compiler("kos-cc")
            .try_flags_from_environment("KOS_CFLAGS")
            .expect("Missing $KOS_CFLAGS -- KallistiOS environment not sourced!")
            .file(&romdisk_code_path)
            .compile("kosromdiskc");
    };
}

fn main() {
    kos_romdisk_dir!("romdisk");
}
