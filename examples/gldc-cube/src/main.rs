#![no_main]
use kos::ffi::{
    dc::maple,
    MAPLE_FOREACH,
};
use gldc::{
    gl::*, glext::*, glkos::*, glu::*,
};

// Include texture data generated by build script
static CLAW_DATA: &[u8] = include_bytes!("../rsrc/tex_claw.vq");
static DC_DATA: &[u8] = include_bytes!("../rsrc/tex_dc.vq");
static DCWIKI_DATA: &[u8] = include_bytes!("../rsrc/tex_dcwiki.vq");
static GCC_DATA: &[u8] = include_bytes!("../rsrc/tex_gcc.vq");
static KOS_DATA: &[u8] = include_bytes!("../rsrc/tex_kos.vq");
static RUST_DATA: &[u8] = include_bytes!("../rsrc/tex_rust.vq");

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        // Initialize GLdc
        glKosInit();

        // Say hello to the world!
        println!("\nHello, world from Rust! - gldc-cube example");

        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        gluPerspective(45.0, 640.0 / 480.0, 0.1, 100.0);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();

        glEnable(GL_TEXTURE_2D);
        glShadeModel(GL_SMOOTH);
        glClearColor(0.0, 0.0, 0.0, 0.5);
        glClearDepth(1.0);
        glEnable(GL_DEPTH_TEST);
        glDepthFunc(GL_LEQUAL);

        let mut tex_claw: u32 = 0;
        glGenTextures(1, &mut tex_claw);
        glBindTexture(GL_TEXTURE_2D, tex_claw);
        glCompressedTexImage2DARB(GL_TEXTURE_2D, 0, GL_COMPRESSED_RGB_565_VQ_TWID_KOS,
                                  512, 512, 0, CLAW_DATA.len() as u32, CLAW_DATA.as_ptr() as *const c_void);

        let mut tex_dc: u32 = 0;
        glGenTextures(1, &mut tex_dc);
        glBindTexture(GL_TEXTURE_2D, tex_dc);
        glCompressedTexImage2DARB(GL_TEXTURE_2D, 0, GL_COMPRESSED_RGB_565_VQ_TWID_KOS,
                                  512, 512, 0, DC_DATA.len() as u32, DC_DATA.as_ptr() as *const c_void);

        let mut tex_dcwiki: u32 = 0;
        glGenTextures(1, &mut tex_dcwiki);
        glBindTexture(GL_TEXTURE_2D, tex_dcwiki);
        glCompressedTexImage2DARB(GL_TEXTURE_2D, 0, GL_COMPRESSED_RGB_565_VQ_TWID_KOS,
                                  512, 512, 0, DCWIKI_DATA.len() as u32, DCWIKI_DATA.as_ptr() as *const c_void);

        let mut tex_gcc: u32 = 0;
        glGenTextures(1, &mut tex_gcc);
        glBindTexture(GL_TEXTURE_2D, tex_gcc);
        glCompressedTexImage2DARB(GL_TEXTURE_2D, 0, GL_COMPRESSED_RGB_565_VQ_TWID_KOS,
                                  512, 512, 0, GCC_DATA.len() as u32, GCC_DATA.as_ptr() as *const c_void);

        let mut tex_kos: u32 = 0;
        glGenTextures(1, &mut tex_kos);
        glBindTexture(GL_TEXTURE_2D, tex_kos);
        glCompressedTexImage2DARB(GL_TEXTURE_2D, 0, GL_COMPRESSED_RGB_565_VQ_TWID_KOS,
                                  512, 512, 0, KOS_DATA.len() as u32, KOS_DATA.as_ptr() as *const c_void);

        let mut tex_rust: u32 = 0;
        glGenTextures(1, &mut tex_rust);
        glBindTexture(GL_TEXTURE_2D, tex_rust);
        glCompressedTexImage2DARB(GL_TEXTURE_2D, 0, GL_COMPRESSED_RGB_565_VQ_TWID_KOS,
                                  512, 512, 0, RUST_DATA.len() as u32, RUST_DATA.as_ptr() as *const c_void);

        let mut xrot: f32 = 0.0;
        let mut yrot: f32 = 0.0;
        let mut zrot: f32 = 0.0;

        let mut draw_gl = || {
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            glLoadIdentity();
            glTranslatef(0.0, 0.0, -5.0);

            glRotatef(xrot, 1.0, 0.0, 0.0);
            glRotatef(yrot, 0.0, 1.0, 0.0);
            glRotatef(zrot, 0.0, 0.0, 1.0);

            // Front face
            glBindTexture(GL_TEXTURE_2D, tex_claw);
            glBegin(GL_QUADS);

            glColor3f(1.0, 1.0, 1.0);
            glTexCoord2f(1.0, 0.0);
            glVertex3f(-1.0, -1.0,  1.0);
            glTexCoord2f(0.0, 0.0);
            glVertex3f(1.0, -1.0,  1.0);
            glTexCoord2f(0.0, 1.0);
            glVertex3f(1.0,  1.0,  1.0);
            glTexCoord2f(1.0, 1.0);
            glVertex3f(-1.0,  1.0,  1.0);

            glEnd();

            // Back face
            glBindTexture(GL_TEXTURE_2D, tex_gcc);
            glBegin(GL_QUADS);

            glColor3f(1.0, 1.0, 1.0);
            glTexCoord2f(0.0, 0.0);
            glVertex3f(-1.0, -1.0, -1.0);
            glTexCoord2f(0.0, 1.0);
            glVertex3f(-1.0,  1.0, -1.0);
            glTexCoord2f(1.0, 1.0);
            glVertex3f(1.0,  1.0, -1.0);
            glTexCoord2f(1.0, 0.0);
            glVertex3f(1.0, -1.0, -1.0);

            glEnd();

            // Top face
            glBindTexture(GL_TEXTURE_2D, tex_dcwiki);
            glBegin(GL_QUADS);

            glColor3f(1.0, 1.0, 1.0);
            glTexCoord2f(1.0, 1.0);
            glVertex3f(-1.0,  1.0, -1.0);
            glTexCoord2f(1.0, 0.0);
            glVertex3f(-1.0,  1.0,  1.0);
            glTexCoord2f(0.0, 0.0);
            glVertex3f(1.0,  1.0,  1.0);
            glTexCoord2f(0.0, 1.0);
            glVertex3f(1.0,  1.0, -1.0);

            glEnd();

            // Bottom face
            glBindTexture(GL_TEXTURE_2D, tex_kos);
            glBegin(GL_QUADS);

            glColor3f(1.0, 1.0, 1.0);
            glTexCoord2f(0.0, 1.0);
            glVertex3f(-1.0, -1.0, -1.0);
            glTexCoord2f(1.0, 1.0);
            glVertex3f(1.0, -1.0, -1.0);
            glTexCoord2f(1.0, 0.0);
            glVertex3f(1.0, -1.0,  1.0);
            glTexCoord2f(0.0, 0.0);
            glVertex3f(-1.0, -1.0,  1.0);

            glEnd();

            // Right face
            glBindTexture(GL_TEXTURE_2D, tex_dc);
            glBegin(GL_QUADS);

            glColor3f(0.0, 1.0, 0.0);
            glTexCoord2f(0.0, 0.0);
            glVertex3f(1.0, -1.0, -1.0);
            glColor3f(0.3, 0.5, 1.0);
            glTexCoord2f(0.0, 1.0);
            glVertex3f(1.0,  1.0, -1.0);
            glColor3f(1.0, 0.3, 0.5);
            glTexCoord2f(1.0, 1.0);
            glVertex3f(1.0,  1.0,  1.0);
            glColor3f(0.5, 0.5, 0.5);
            glTexCoord2f(1.0, 0.0);
            glVertex3f(1.0, -1.0,  1.0);

            glEnd();

            // Left face
            glBindTexture(GL_TEXTURE_2D, tex_rust);
            glBegin(GL_QUADS);

            glColor3f(1.0, 0.0, 0.0);
            glTexCoord2f(1.0, 0.0);
            glVertex3f(-1.0, -1.0, -1.0);
            glColor3f(1.0, 1.0, 0.0);
            glTexCoord2f(0.0, 0.0);
            glVertex3f(-1.0, -1.0,  1.0);
            glColor3f(0.0, 1.0, 1.0);
            glTexCoord2f(0.0, 1.0);
            glVertex3f(-1.0,  1.0,  1.0);
            glColor3f(0.0, 0.0, 1.0);
            glTexCoord2f(1.0, 1.0);
            glVertex3f(-1.0,  1.0, -1.0);

            glEnd();

            xrot += 56.0;
            yrot += 44.0;
            zrot += 72.0;
        };


        // Done setting up, now let's loop!
        loop {
            let mut done = false;

            MAPLE_FOREACH!(maple::MAPLE_FUNC_CONTROLLER, maple::controller::cont_state_t, state, || {
                if ((*state).buttons & maple::controller::CONT_START) != 0 {
                    println!("Start pressed! Quitting...");
                    done = true;
                }
            });


            if done == true {
                break;
            }

            draw_gl();

            glKosSwapBuffers();
        }

        // Clean up our textures
        glDeleteTextures(1, &mut tex_claw);
        glDeleteTextures(1, &mut tex_dc);
        glDeleteTextures(1, &mut tex_dcwiki);
        glDeleteTextures(1, &mut tex_gcc);
        glDeleteTextures(1, &mut tex_kos);
        glDeleteTextures(1, &mut tex_rust);
    }

    println!("Bye!");

    0
}
