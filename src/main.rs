#![no_std]
#![no_main]
#![feature(compiler_builtins_lib)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate compiler_builtins;
extern crate r0;
#[macro_use] // To get the hprintf! macro from semi-hosting
extern crate stm32f7_discovery as stm32f7;

use embedded::interfaces::gpio::Gpio;
use stm32f7::{board, embedded, interrupts, sdram, system_clock, touch, i2c};
mod lcd; // use custom LCD implementation
use lcd::Framebuffer;
use lcd::FramebufferL8;
mod fps;
use core::ptr;
mod graphics;

const USE_DOUBLE_BUFFER: bool = true;
const ENABLE_FPS_OUTPUT: bool = false;
const PRINT_START_MESSAGE: bool = true;

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static __DATA_END: u32;
        static mut __DATA_START: u32;

        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }

    let data_load = &__DATA_LOAD;
    let data_start = &mut __DATA_START;
    let data_end = &__DATA_END;

    let bss_start = &mut __BSS_START;
    let bss_end = &__BSS_END;

    // initializes the .data section
    // (copy the data segment initializers from flash to RAM)
    r0::init_data(data_start, data_end, data_load);
    // zeroes the .bss section
    r0::zero_bss(bss_start, bss_end);

    stm32f7::heap::init();

    // Initialize the floating point unit
    let scb = stm32f7::cortex_m::peripheral::scb_mut();
    scb.cpacr.modify(|v| v | 0b1111 << 20);

    main(board::hw());
}

fn main(hw: board::Hardware) -> ! {
    if PRINT_START_MESSAGE {
        hprintln!("\n[38;5;40m[1m🔦 Flash complete! ✔️\n[38;5;45m🚀 Program started.(B[m");
    }
    let board::Hardware {
        rcc,
        pwr,
        flash,
        fmc,
        ltdc,
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
        i2c_3,
        sai_2,
        syscfg,
        ethernet_mac,
        ethernet_dma,
        nvic,
        ..
    } = hw;
    interrupts::scope(
        nvic,
        |_| {},
        move |interrupt_table| {
            let mut gpio = Gpio::new(
                gpio_a,
                gpio_b,
                gpio_c,
                gpio_d,
                gpio_e,
                gpio_f,
                gpio_g,
                gpio_h,
                gpio_i,
                gpio_j,
                gpio_k,
            );

            system_clock::init(rcc, pwr, flash);

            // enable all gpio ports
            rcc.ahb1enr.update(|r| {
                r.set_gpioaen(true);
                r.set_gpioben(true);
                r.set_gpiocen(true);
                r.set_gpioden(true);
                r.set_gpioeen(true);
                r.set_gpiofen(true);
                r.set_gpiogen(true);
                r.set_gpiohen(true);
                r.set_gpioien(true);
                r.set_gpiojen(true);
                r.set_gpioken(true);
            });

            // init sdram (for display)
            sdram::init(rcc, fmc, &mut gpio);

            // init touch screen
            i2c::init_pins_and_clocks(rcc, &mut gpio);
            let mut i2c_3 = i2c::init(i2c_3);
            touch::check_family_id(&mut i2c_3).unwrap();

            let mut lcd = lcd::init(ltdc, rcc, &mut gpio);
            lcd.set_background_color(lcd::Color {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 255,
            });
            let mut framebuffer = FramebufferL8::new();
            framebuffer.init();
            lcd.framebuffer_addr = framebuffer.get_framebuffer_addr() as u32;
            lcd.backbuffer_addr = framebuffer.get_backbuffer_addr() as u32;

            if !USE_DOUBLE_BUFFER {
                lcd.swap_buffers();
            }
            lcd.swap_buffers();

            let should_draw_now = false;
            let should_draw_now_ptr = (&should_draw_now as *const bool) as usize;

            let interrupt_handler = interrupt_table
                .register(
                    interrupts::interrupt_request::InterruptRequest::LcdTft,
                    interrupts::Priority::P1,
                    move || {
                        unsafe {
                            let need_draw = ptr::read_volatile(should_draw_now_ptr as *mut bool);
                            if !need_draw {
                                if USE_DOUBLE_BUFFER {
                                    lcd.swap_buffers();
                                }
                                ptr::write_volatile(should_draw_now_ptr as *mut bool, true);
                            }
                        }
                        lcd.clr_line_interrupt();
                    },
                )
                .expect("LcdTft interrupt already used");

            run(&mut framebuffer, &mut i2c_3, should_draw_now_ptr)
        },
    )
}

fn run(framebuffer: &mut FramebufferL8, i2c_3: &mut i2c::I2C, should_draw_now_ptr: usize) -> ! {
    hprintln!("Start run()");
    //// INIT COMPLETE ////
    let mut fps = fps::init();
    fps.output_enabled = ENABLE_FPS_OUTPUT;

    let mut current_color = 255;

    let mut running_x = 40;
    let mut running_y = 0;

    loop {
        let mut need_draw = false;
        unsafe {
            // Frame synchronisation
            need_draw = ptr::read_volatile(should_draw_now_ptr as *mut bool);
        }
        if need_draw {
            if USE_DOUBLE_BUFFER {
                framebuffer.swap_buffers();
            }

            game_loop(&mut running_x, &mut running_y, framebuffer, &mut current_color, i2c_3, &fps);

            // end of frame
            fps.count_frame();
            unsafe {
                ptr::write_volatile(should_draw_now_ptr as *mut bool, false);
            }
        }
    }
}

fn game_loop(running_x: &mut usize, running_y: &mut usize, framebuffer: &mut FramebufferL8, current_color: &mut u8, i2c_3: &mut i2c::I2C, fps: &fps::FpsCounter) {
    logic(running_x, running_y);


    for i in 0..10 {
        framebuffer.clear();
        graphics::quad(32, 32, 200, current_color, framebuffer);
    }

    draw(framebuffer, &running_x, &running_y, &current_color);
    graphics::draw_fps(framebuffer, fps);

    for touch in &touch::touches(i2c_3).unwrap() {
        framebuffer.set_pixel_direct(touch.x as usize, touch.y as usize, *current_color);

        if in_rect(touch.x as usize, touch.y as usize, 30, 30, 50, 50) {
            *current_color = 255;
        } else if in_rect(touch.x as usize, touch.y as usize, 30, 30 + 80, 50, 50) {
            *current_color = 128;
        } else if in_rect(touch.x as usize, touch.y as usize, 30, 30 + 80 + 80, 50, 50) {
            *current_color = 0;
        }
    }
}

fn draw(
    layer1: &mut lcd::FramebufferL8,
    running_x: &usize,
    running_y: &usize,
    current_color: &u8,
) {
    layer1.set_pixel_direct(*running_x, *running_y, *current_color);
}

fn logic(running_x: &mut usize, running_y: &mut usize) {
    *running_x += 1;
    if *running_x >= 480 {
        *running_x = 40;
        *running_y += 1;
        if *running_y >= 272 {
            *running_y = 0;
        }
    }
}

/*
// high level
fn main() {
    println!("Hello, world!");

    let is_server = false;

    loop {
        if is_server {
            server_loop();
        }

        game_loop();
    }
}

fn server_loop() {
    receive_input_from_clients();
    game_update();
    send_state_to_clients();
}


fn game_loop() {
    receive_state_from_server();
    read_input();
    send_input_to_server();
    draw_stuff();
}
    
    */

fn in_rect(
    pos_x: usize,
    pos_y: usize,
    rect_x: usize,
    rect_y: usize,
    rect_width: usize,
    rect_height: usize,
) -> bool {
    pos_x > rect_x && pos_y > rect_y && pos_x < rect_x + rect_width && pos_y < rect_y + rect_height
}

