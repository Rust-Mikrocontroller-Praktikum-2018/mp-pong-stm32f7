#![allow(dead_code)]

pub use self::init::init;
pub use self::text::TextWriter;

use alloc::Vec;
use board::ltdc;
use board::ltdc::L1clutwr;
use board::ltdc::Ltdc;
use core::ptr;
use embedded::interfaces::gpio::OutputPin;
use stm32f7::lcd::Color;

#[macro_use]
mod init;
mod text;

pub const HEIGHT: usize = 272;
pub const WIDTH: usize = 480;

const LAYER_1_OCTETS_PER_PIXEL: usize = 1;
const LAYER_1_LENGTH: usize = HEIGHT * WIDTH * LAYER_1_OCTETS_PER_PIXEL;

const SDRAM_START: usize = 0xC000_0000;
const LAYER_1_START: usize = SDRAM_START;
const LAYER_1_START_2: usize = SDRAM_START + 1024 * 1024 * 1; // move backbuffer to second SDRAM bank

static EMPTY_IMG: &[u8] = include_bytes!("../../res/empty.img");


pub struct Lcd {
    controller: &'static mut Ltdc,
    display_enable: OutputPin,
    backlight_enable: OutputPin,
    write_to_buffer_2: bool,
    pub framebuffer_addr: u32,
    pub backbuffer_addr: u32,
    pub clut: [(u8, u8, u8); 256],
}

impl Lcd {
    pub fn set_background_color(&mut self, color: Color) {
        self.controller.bccr.update(|r| r.set_bc(color.to_rgb()));
    }

    pub fn swap_buffers(&mut self) {
        if self.write_to_buffer_2 {
            let framebuffer_addr = self.framebuffer_addr;
            self.controller
                .l1cfbar
                .update(|r| r.set_cfbadd(framebuffer_addr));
        } else {
            let backbuffer_addr = self.backbuffer_addr;
            self.controller
                .l1cfbar
                .update(|r| r.set_cfbadd(backbuffer_addr));
        }

        // reload shadow registers
        self.controller.srcr.update(|r| r.set_imr(true)); // IMMEDIATE_RELOAD

        self.write_to_buffer_2 = !self.write_to_buffer_2;
    }

    pub fn clr_line_interrupt(&mut self) {
        let mut clr_flags = ltdc::Icr::default();
        clr_flags.set_clif(true);
        self.controller.icr.write(clr_flags);
    }

    // reads the rgb colors defined in the lcd.clut and sets them
    pub fn update_clut(&mut self) {
        // define CLUT for layer 1
        for c in 0..=255 {
            let mut clut = L1clutwr::default();
            clut.set_red(self.clut[c].0);
            clut.set_green(self.clut[c].1);
            clut.set_blue(self.clut[c].2);
            clut.set_clutadd(c as u8);
            self.controller.l1clutwr.write(clut);
        }
        self.controller.l1cr.update(|r| {
            r.set_cluten(true); // enable CLUT for layer 1
        });
    }
}

pub trait Framebuffer {
    fn set_pixel(&mut self, x: usize, y: usize, color: u8);
    fn swap_buffers(&mut self);
    fn clear(&mut self);
    fn copy_full(&mut self, src_start_ptr: *const u8);
}

pub struct FramebufferL8 {
    pub write_to_buffer_2: bool,
    pub framebuffer: Vec<u8>,
    pub backbuffer: Vec<u8>,
    pub framebuffer_addr: *const u8,
    pub backbuffer_addr: *const u8,
}

impl FramebufferL8 {
    pub fn new() -> Self {
        let write_to_buffer_2 = false;

        FramebufferL8 {
            write_to_buffer_2,
            framebuffer: vec![0; WIDTH * HEIGHT],
            backbuffer: vec![0; WIDTH * HEIGHT],
            framebuffer_addr: 0 as *const u8,
            backbuffer_addr: 0 as *const u8,
        }
    }

    pub fn init(&mut self) {
        // need to set these intern addresses correct
        self.framebuffer_addr = &(self.framebuffer[0]) as *const u8;
        self.backbuffer_addr = &(self.backbuffer[0]) as *const u8;
    }

    fn current_base_addr(&mut self) -> usize {
        if self.write_to_buffer_2 {
            self.get_backbuffer_addr() as usize
        } else {
            self.get_framebuffer_addr() as usize
        }
    }

    pub fn get_framebuffer_addr(&self) -> *const u8 {
        self.framebuffer_addr
        // &(self.framebuffer[0]) as *const u8
    }

    pub fn get_backbuffer_addr(&self) -> *const u8 {
        self.backbuffer_addr
        // &(self.backbuffer[0]) as *const u8
    }
}

impl Framebuffer for FramebufferL8 {
    fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        let pixel = y * WIDTH + x;

        let pixel_ptr = (self.current_base_addr() + pixel * LAYER_1_OCTETS_PER_PIXEL) as *mut u8;
        unsafe {
            ptr::write_volatile(pixel_ptr, color);
        };
    }

    fn swap_buffers(&mut self) {
        let src_start_ptr;
        let dest_start_ptr;

        if self.write_to_buffer_2 {
            src_start_ptr = self.get_backbuffer_addr() as *mut u32;
            dest_start_ptr = self.get_framebuffer_addr() as *mut u32;
        } else {
            src_start_ptr = self.get_framebuffer_addr() as *mut u32;
            dest_start_ptr = self.get_backbuffer_addr() as *mut u32;
        }

        self.write_to_buffer_2 = !self.write_to_buffer_2;

        let mut count = WIDTH * HEIGHT;
        if LAYER_1_OCTETS_PER_PIXEL == 1 {
            count /= 4; // we only store u8 for every pixel
        }
        if LAYER_1_OCTETS_PER_PIXEL == 2 {
            count /= 2;
        }
        unsafe {
            ptr::copy_nonoverlapping(src_start_ptr, dest_start_ptr, count);
        }
    }

    fn clear(&mut self) {
        // slow
        /*for i in 0..HEIGHT {
            for j in 0..WIDTH {
                self.set_pixel(j, i, 0);
            }
        }*/
        let src_start_ptr = &EMPTY_IMG[0] as *const u8;
        self.copy_full(src_start_ptr);
    }

    fn copy_full(&mut self, src_start_ptr: *const u8) {
        let dest_start_ptr = self.current_base_addr() as *mut u32;

        unsafe {
            ptr::copy_nonoverlapping(
                src_start_ptr as *const u32,
                dest_start_ptr,
                WIDTH * HEIGHT / 4  // we only store u8 for every pixel
            );
        }
    }
}
