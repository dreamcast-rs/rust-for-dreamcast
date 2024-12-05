#![feature(raw_ref_op)]
use kos::{
    dbglog, dbglog::DbgLevel::{ Debug, Info },
    ffi::{
        dc::{ pvr::*, video::*, maple },
        prelude::*, MAPLE_FOREACH,
    },
};
use std::mem::{MaybeUninit, size_of, zeroed};
use std::time::{SystemTime, UNIX_EPOCH};

kos::INIT_FLAGS!(INIT_DEFAULT);

enum Phase {
    Halve,
    Incr,
    Decr,
    Final,
}

struct PvrMark {
    hdr:        pvr_poly_hdr_t,
    polycnt:    i32,
    phase:      Phase,
    avgfps:     Option<f32>,
    begin:      u64,
    seed:       i32,
}

impl PvrMark {
    fn new() -> Self {
        let pvrmark = PvrMark {
            hdr:        unsafe { zeroed::<pvr_poly_hdr_t>() },
            polycnt:    0,
            phase:      Phase::Halve,
            avgfps:     None,
            begin:      0,
            #[allow(overflowing_literals)]
            seed:       0xdeadbeef,
        };

        pvrmark
    }
    
    fn run_benchmark(&mut self) {
        let mut pvr_params = pvr_init_params_t {
            opb_sizes:          [PVR_BINSIZE_16, PVR_BINSIZE_0, PVR_BINSIZE_0,
                                PVR_BINSIZE_0, PVR_BINSIZE_0],
            vertex_buf_size:    512 * 1024,
            dma_enabled:        0,
            fsaa_enabled:       0,
            autosort_disabled:  0,
            opb_overflow_count: 0,
        };

        unsafe {
            let mut cxt = zeroed::<pvr_poly_cxt_t>();

            pvr_init(&raw mut pvr_params);
            pvr_set_bg_color(0.0, 0.0, 0.0);
            
            pvr_poly_cxt_col(&raw mut cxt, PVR_LIST_OP_POLY);
            cxt.gen.shading = PVR_SHADE_FLAT;
            pvr_poly_compile(&raw mut self.hdr, &raw mut cxt);
        }

        self.switch_tests(2000000 / 60);
        self.begin = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                            
        let mut start_pressed = false;

        loop {
            unsafe {
                MAPLE_FOREACH!(maple::MAPLE_FUNC_CONTROLLER,
                               maple::controller::cont_state_t, state, || {
                    if ((*state).buttons & maple::controller::CONT_START) != 0 {
                        start_pressed = true;
                    }
                });
            }
        
            if start_pressed == true {
                break;
            }
            
            print!(" \r");
        
            self.do_frame();
        
            let stats = get_stats();
            
            if let Some(fps) = self.avgfps {
                self.avgfps = Some((fps + stats.frame_rate) / 2.0);
            } else {
                self.avgfps = Some(stats.frame_rate);
            }
            
            self.check_switch();
        }
    }

    fn switch_tests(&mut self, ppf: i32) {
        dbglog!(Debug, "Beginning new test: {} polys per frame ({} per second at 60fps)\n",
                ppf, ppf * 60);
        self.avgfps = None;
        self.polycnt = ppf;
    }

    fn check_switch(&mut self) {
        let avgfps = self.avgfps.unwrap();

        let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
        
        if now >= (self.begin + 5) {
            self.begin = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
            
            dbglog!(Debug, "  Average Frame Rate: ~{} fps ({} pps)\n",
                    avgfps, self.polycnt * avgfps as i32);
                
            match self.phase {
                Phase::Halve => {
                    if avgfps < 55.0 {
                        self.switch_tests(self.polycnt / 2);
                    } else {
                        dbglog!(Debug, "  Entering Phase::Incr\n");
                        self.phase = Phase::Incr;
                    }
                },
                Phase::Incr => {
                    if avgfps >= 55.0 {
                        self.switch_tests(self.polycnt + 500);
                    } else {
                        dbglog!(Debug, "  Entering Phase::Decr\n");
                        self.phase = Phase::Decr;
                    }
                },
                Phase::Decr => {
                    if avgfps < 55.0 {
                        self.switch_tests(self.polycnt - 200);
                    } else {
                        dbglog!(Debug, "  Entering Phase::Final\n");
                        self.phase = Phase::Final;
                    }
                },
                Phase::Final => {
                },
            }
        }
    }

    #[inline]
    fn nextnum(&mut self) {
        self.seed = self.seed.wrapping_mul(1164525).wrapping_add(1013904223);
    }

    #[inline]
    fn getnum(&mut self, nm: i32) -> i32 {
        self.seed & (nm - 1)
    }

    fn do_frame(&mut self) {
        unsafe {
            vid_border_color(0, 0, 0);
            pvr_wait_ready();
            vid_border_color(255, 0, 0);
            pvr_scene_begin();
            pvr_list_begin(PVR_LIST_OP_POLY);
            pvr_prim(&raw mut self.hdr as *mut c_void, size_of::<pvr_poly_hdr_t>() as i32);

            let mut dr_state: MaybeUninit<pvr_dr_state_t> = MaybeUninit::uninit();         
            pvr_dr_init(dr_state.as_mut_ptr());
            let mut dr_state = dr_state.assume_init();
            
            let mut x = self.getnum(1024);
            self.nextnum();
            let mut y = self.getnum(512);
            self.nextnum();
            let z = self.getnum(128) + 1;
            self.nextnum();
            let mut col = self.getnum(256) as u32;
            self.nextnum();

            let mut vert = pvr_dr_target(&mut dr_state);
            (*vert).flags = PVR_CMD_VERTEX;
            (*vert).x = x as f32;
            (*vert).y = y as f32;
            (*vert).z = z as f32;
            (*vert).u = 0.0;
            (*vert).v = 0.0;
            (*vert).argb = col | (col << 8) | (col << 16) | 0xff000000;
            (*vert).oargb = 0;
            pvr_dr_commit(vert as *const c_void);

            for i in 0..self.polycnt {
                x = x.wrapping_add(self.getnum(64) - 32) & 1023;
                self.nextnum();
                y = y.wrapping_add(self.getnum(64) - 32) % 511;
                self.nextnum();
                col = self.getnum(256) as u32;
                self.nextnum();
                vert = pvr_dr_target(&mut dr_state);
                (*vert).flags = PVR_CMD_VERTEX;
                (*vert).x = x as f32;
                (*vert).y = y as f32;
                (*vert).z = z as f32;
                (*vert).u = 0.0;
                (*vert).v = 0.0;
                (*vert).argb = col | (col << 8) | (col << 16) | 0xff000000;
                (*vert).oargb = 0;
                
                if i == (self.polycnt - 1) {
                    (*vert).flags = PVR_CMD_VERTEX_EOL;
                }

                pvr_dr_commit(vert as *const c_void);
            }
            
            pvr_list_finish();
            pvr_scene_finish();
            vid_border_color(0, 255, 0);
        }
    }
}

fn get_stats() -> pvr_stats_t {
    unsafe {
        let mut stats: MaybeUninit<pvr_stats_t> = MaybeUninit::uninit();
        pvr_get_stats(stats.as_mut_ptr());
        stats.assume_init()
    }
}

fn print_stats() {
    let stats = get_stats();
    dbglog!(Debug, "3D Stats: {} frames, frame rate ~{} fps\n",
            stats.vbl_count, stats.frame_rate);
}

fn main() {
    dbglog!(Info, "Hello, world from Rust! - pvrmark_strips_direct example\n");
    
    PvrMark::new().run_benchmark();
    
    print_stats();
}
