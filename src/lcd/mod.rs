#![allow(dead_code)]

pub use self::color::Color;
pub use self::init::init;

use board::ltdc::Ltdc;
use board::ltdc;
use core::ptr;
use embedded::interfaces::gpio::OutputPin;

#[macro_use]
mod init;
mod color;

const HEIGHT: usize = 272;
const WIDTH: usize = 480;

const LAYER_1_OCTETS_PER_PIXEL: usize = 2;
const LAYER_1_LENGTH: usize = HEIGHT * WIDTH * LAYER_1_OCTETS_PER_PIXEL;

const SDRAM_START: usize = 0xC000_0000;
const LAYER_1_START: usize = SDRAM_START;
const LAYER_1_START_2: usize = SDRAM_START +  1024 * 1024 * 1; // move backbuffer to second SDRAM bank

pub struct Lcd {
    controller: &'static mut Ltdc,
    display_enable: OutputPin,
    backlight_enable: OutputPin,
    layer_1_in_use: bool,
    write_to_buffer_2: bool,
}

impl Lcd {
    pub fn set_background_color(&mut self, color: Color) {
        self.controller.bccr.update(|r| r.set_bc(color.to_rgb()));
    }

    pub fn layer_1(&mut self) -> Option<Layer<FramebufferL8>> {
        if self.layer_1_in_use {
            None
        } else {
            Some(Layer {
                framebuffer: FramebufferL8::new(LAYER_1_START, LAYER_1_START_2),
            })
        }
    }
    pub fn swap_buffers(&mut self) {
        if self.write_to_buffer_2 {
            self.controller
                .l1cfbar
                .update(|r| r.set_cfbadd(LAYER_1_START as u32));
        } else {
            self.controller
                .l1cfbar
                .update(|r| r.set_cfbadd(LAYER_1_START_2 as u32));
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
    fn swap_buffers(&mut self);
}

pub struct FramebufferL8 {
    base_addr: usize,
    base_addr_2: usize,
    write_to_buffer_2: bool,
}

impl FramebufferL8 {
    fn new(base_addr: usize, base_addr_2: usize) -> Self {
        let write_to_buffer_2 = false;
        FramebufferL8 {
            base_addr,
            base_addr_2,
            write_to_buffer_2,
        }
    }

    fn current_base_addr(&mut self) -> usize {
        if self.write_to_buffer_2 {
            self.base_addr_2
        } else {
            self.base_addr
        }
    }
}

impl Framebuffer for FramebufferL8 {
    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let pixel = y * WIDTH + x;
        let pixel_ptr = (self.current_base_addr() + pixel * LAYER_1_OCTETS_PER_PIXEL) as *mut u16;
        unsafe { ptr::write_volatile(pixel_ptr, (color.to_l8() as u16)<<8|0xff ); };
    }

    fn swap_buffers(&mut self) {
        let src_start_ptr;
        let dest_start_ptr;

        if self.write_to_buffer_2 {
            src_start_ptr = LAYER_1_START_2 as *mut u32;
            dest_start_ptr = LAYER_1_START as *mut u32;
            
        } else {
            src_start_ptr = LAYER_1_START as *mut u32;
            dest_start_ptr = LAYER_1_START_2 as *mut u32;
        }

        self.write_to_buffer_2 = !self.write_to_buffer_2;
        
        unsafe {
            ptr::copy_nonoverlapping(
                src_start_ptr,
                dest_start_ptr,
                WIDTH * HEIGHT / 4  // we only store u8 for every pixel
            );
        } 
    }
}

pub struct Layer<T> {
    framebuffer: T,
}

impl<T: Framebuffer> Layer<T> {
    pub fn clear(&mut self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                self.framebuffer.set_pixel(j, i, Color::rgb(0,0,0));
            }
        }
    }

    pub fn print_point_at(&mut self, x: usize, y: usize) {
        self.print_point_color_at(x, y, Color::from_hex(0xff_ffff));
    }

    pub fn print_point_color_at(&mut self, x: usize, y: usize, color: Color) {
        assert!(x < WIDTH);
        assert!(y < HEIGHT);

        self.framebuffer.set_pixel(x, y, color);
    }

    pub fn swap_buffers(&mut self) {
        self.framebuffer.swap_buffers();
    }
}
