#![allow(dead_code)]

pub use self::color::Color;
pub use self::init::init;

use alloc::Vec;
use board::ltdc;
use board::ltdc::Ltdc;
use core::ptr;
use embedded::interfaces::gpio::OutputPin;

#[macro_use]
mod init;
mod color;

const HEIGHT: usize = 272;
const WIDTH: usize = 480;

const LAYER_1_OCTETS_PER_PIXEL: usize = 1;
const LAYER_1_LENGTH: usize = HEIGHT * WIDTH * LAYER_1_OCTETS_PER_PIXEL;

const SDRAM_START: usize = 0xC000_0000;
const LAYER_1_START: usize = SDRAM_START;
const LAYER_1_START_2: usize = SDRAM_START + 1024 * 1024 * 1; // move backbuffer to second SDRAM bank

pub struct Lcd {
    controller: &'static mut Ltdc,
    display_enable: OutputPin,
    backlight_enable: OutputPin,
    write_to_buffer_2: bool,
    pub framebuffer_addr: u32,
    pub backbuffer_addr: u32,
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
}

pub trait Framebuffer {
    fn set_pixel(&mut self, x: usize, y: usize, color: Color);
    fn set_pixel_direct(&mut self, x: usize, y: usize, color: u8);
    fn swap_buffers(&mut self);
    fn clear(&mut self);
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
    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.set_pixel_direct(x, y, color.to_l8());
    }
    fn set_pixel_direct(&mut self, x: usize, y: usize, color: u8) {
        let pixel = y * WIDTH + x;
        // ARGB8888
/*        let pixel_ptr = (self.current_base_addr() + pixel * LAYER_1_OCTETS_PER_PIXEL) as *mut u32;
        unsafe { ptr::write_volatile(pixel_ptr,
        (color as u32) << 8 | (color as u32) | 0xffff_0000); };*/

        // AL88
        /*let pixel_ptr = (self.current_base_addr() + pixel * LAYER_1_OCTETS_PER_PIXEL) as *mut u16;
        unsafe { ptr::write_volatile(pixel_ptr, (color as u16) | 0xff00 ); };*/

        // L8
        /*
        // This is horribly slow... why?
        if self.write_to_buffer_2 {
            self.backbuffer[pixel] = color;
        } else {
            self.framebuffer[pixel] = color;
        }*/

        let pixel_ptr = (self.current_base_addr() + pixel * LAYER_1_OCTETS_PER_PIXEL) as *mut u8;
        unsafe {
            ptr::write_volatile(pixel_ptr, color);
        };

        // L8 fix(?)
        /*let pixel_half = pixel / 2;
        let is_left_pixel = pixel % 2 == 0;
        let pixel_ptr = (self.current_base_addr() + pixel_half * 2) as *mut u16;
        unsafe {
            if is_left_pixel  {
                let right_pixel_ptr = (self.current_base_addr() + pixel_half * 2 + 1) as *const u8;
                let right_pixel = *right_pixel_ptr;// ptr::read_volatile(right_pixel_ptr);
                ptr::write_volatile(pixel_ptr, (color as u16) | ((right_pixel as u16) <<8) );
            } else {
                let left_pixel_ptr = (self.current_base_addr() + pixel_half * 2) as *const u8;
                let left_pixel = *left_pixel_ptr;//ptr::read_volatile(left_pixel_ptr);
                ptr::write_volatile(pixel_ptr, (color as u16) << 8 | (left_pixel as u16) );
            }
            
         };*/    }

    fn swap_buffers(&mut self) {
        // here this is faster than the alternative below
/*        if self.write_to_buffer_2 {
            self.framebuffer = self.backbuffer.copy();
        } else {
            self.backbuffer = self.framebuffer;
        }
        self.write_to_buffer_2 = !self.write_to_buffer_2;*/

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
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                self.set_pixel_direct(j, i, 22);
            }
        }
    }
}
