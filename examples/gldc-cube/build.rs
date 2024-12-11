fn main() {
    generate_textures(vec!["tex_claw", "tex_dc", "tex_dcwiki", "tex_gcc", "tex_kos", "tex_rust"]);
}

fn generate_textures(names: Vec<&str>) {
    let vqenc_cmd = format!("{}/utils/vqenc/vqenc", std::env::var("KOS_BASE").unwrap());

    for texname in names {
        let jpgfile = format!("rsrc/{}.jpg", texname);
        let vqfile = format!("rsrc/{}.vq", texname);
        let outfile = format!("{}/{}.vq", std::env::var("OUT_DIR").unwrap(), texname);
        std::process::Command::new(&vqenc_cmd)
            .args(["-t", "-v", &jpgfile])
            .output()
            .expect("vqenc on {&vqfile} failed!");
        let _ = std::fs::rename(&vqfile, &outfile);
    }
}
