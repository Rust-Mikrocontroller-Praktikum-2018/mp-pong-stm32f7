#![allow(dead_code)]

pub use self::color::Color;
pub use self::init::init;

use board::ltdc::Ltdc;
use core::ptr;
use embedded::interfaces::gpio::OutputPin;

#[macro_use]
mod init;
mod color;

const HEIGHT: usize = 272;
const WIDTH: usize = 480;

const LAYER_1_OCTETS_PER_PIXEL: usize = 4;
const LAYER_1_LENGTH: usize = HEIGHT * WIDTH * LAYER_1_OCTETS_PER_PIXEL;

const SDRAM_START: usize = 0xC000_0000;
const LAYER_1_START: usize = SDRAM_START;
const LAYER_1_START_2: usize = LAYER_1_START + WIDTH * HEIGHT * LAYER_1_OCTETS_PER_PIXEL;

pub struct Lcd {
    controller: &'static mut Ltdc,
    display_enable: OutputPin,
    backlight_enable: OutputPin,
    layer_1_in_use: bool,
    use_buffer_2: bool,
}

impl Lcd {
    pub fn set_background_color(&mut self, color: Color) {
        self.controller.bccr.update(|r| r.set_bc(color.to_rgb()));
    }

    pub fn layer_1(&mut self) -> Option<Layer<FramebufferArgb8888>> {
        if self.layer_1_in_use {
            None
        } else {
            Some(Layer {
                framebuffer: FramebufferArgb8888::new(LAYER_1_START),
            })
        }
    }
    pub fn swap_buffers(&mut self) {
        self.use_buffer_2 = !self.use_buffer_2;
        if self.use_buffer_2 {
            self.controller
                .l1cfbar
                .update(|r| r.set_cfbadd(LAYER_1_START_2 as u32));
        } else {
            self.controller
                .l1cfbar
                .update(|r| r.set_cfbadd(LAYER_1_START as u32));
        }

        // configure color frame buffer line length and pitch
        self.controller.l1cfblr.update(|r| {
            r.set_cfbp((WIDTH * LAYER_1_OCTETS_PER_PIXEL) as u16); // pitch
            r.set_cfbll((WIDTH * LAYER_1_OCTETS_PER_PIXEL + 3) as u16); // line_length
        });

        // configure frame buffer line number
        self.controller.l1cfblnr.update(|r| r.set_cfblnbr(HEIGHT as u16)); // line_number

        // enable layers
        self.controller.l1cr.update(|r| r.set_len(true));

        // reload shadow registers
        self.controller.srcr.update(|r| r.set_imr(true)); // IMMEDIATE_RELOAD
    }
}

pub trait Framebuffer {
    fn set_pixel(&mut self, x: usize, y: usize, color: Color);
}

pub struct FramebufferArgb8888 {
    base_addr: usize,
}

impl FramebufferArgb8888 {
    fn new(base_addr: usize) -> Self {
        Self { base_addr }
    }
}

impl Framebuffer for FramebufferArgb8888 {
    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let pixel = y * WIDTH + x;
        let pixel_ptr = (self.base_addr + pixel * LAYER_1_OCTETS_PER_PIXEL) as *mut u32;
        unsafe { ptr::write_volatile(pixel_ptr, color.to_argb8888()) };
    }
}

pub struct Layer<T> {
    framebuffer: T,
}

impl<T: Framebuffer> Layer<T> {
    pub fn horizontal_stripes(&mut self) {
        let colors = [
            0xffffff, 0xcccccc, 0x999999, 0x666666, 0x333333, 0x0, 0xff0000, 0x0000ff
        ];

        // horizontal stripes
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                self.framebuffer.set_pixel(
                    j,
                    i,
                    Color::from_rgb888(colors[(i / 10) % colors.len()]),
                );
            }
        }
    }

    pub fn vertical_stripes(&mut self) {
        let colors = [
            0xcccccc, 0x999999, 0x666666, 0x333333, 0x0, 0xff0000, 0x0000ff, 0xffffff
        ];

        // vertical stripes
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                self.framebuffer.set_pixel(
                    j,
                    i,
                    Color::from_rgb888(colors[(j / 10) % colors.len()]),
                );
            }
        }
    }

    pub fn clear(&mut self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                self.framebuffer.set_pixel(j, i, Color::from_argb8888(0));
            }
        }
    }

    pub fn print_point_at(&mut self, x: usize, y: usize) {
        self.print_point_color_at(x, y, Color::from_hex(0xffffff));
    }

    pub fn print_point_color_at(&mut self, x: usize, y: usize, color: Color) {
        assert!(x < WIDTH);
        assert!(y < HEIGHT);

        self.framebuffer.set_pixel(x, y, color);
    }
}
