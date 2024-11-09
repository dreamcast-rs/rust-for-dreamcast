use std::process::Command;

fn main() {
    let kos_base = std::env::var("KOS_BASE").expect("Missing $KOS_BASE -- KallistiOS environment not sourced!");
    let vqenc_cmd = kos_base + "/utils/vqenc/vqenc";
    Command::new(&vqenc_cmd)
        .arg("-t")
        .arg("-v")
        .arg("rsrc/tex_claw.jpg")
        .output()
        .expect("vqenc on tex_claw.jpg failed!");
    Command::new(&vqenc_cmd)
        .arg("-t")
        .arg("-v")
        .arg("rsrc/tex_dc.jpg")
        .output()
        .expect("vqenc on tex_dc.jpg failed!");
    Command::new(&vqenc_cmd)
        .arg("-t")
        .arg("-v")
        .arg("rsrc/tex_dcwiki.jpg")
        .output()
        .expect("vqenc on tex_dcwiki.jpg failed!");
    Command::new(&vqenc_cmd)
        .arg("-t")
        .arg("-v")
        .arg("rsrc/tex_gcc.jpg")
        .output()
        .expect("vqenc on tex_gcc.jpg failed!");
    Command::new(&vqenc_cmd)
        .arg("-t")
        .arg("-v")
        .arg("rsrc/tex_kos.jpg")
        .output()
        .expect("vqenc on tex_kos.jpg failed!");
    Command::new(&vqenc_cmd)
        .arg("-t")
        .arg("-v")
        .arg("rsrc/tex_rust.jpg")
        .output()
        .expect("vqenc on tex_rust.jpg failed!");
}
