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
use lcd::FramebufferL8;
use lcd::Framebuffer;
mod fps;

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
    hprintln!("🔦 Flash complete!\n🚀 Program started.");
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

    let mut lcd = lcd::init(ltdc, rcc, &mut gpio);
    lcd.set_background_color(lcd::Color {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 255,
    });
    // let mut layer1 = lcd.layer_1().unwrap();
    let mut framebuffer:FramebufferL8 = FramebufferL8::new();
    lcd.framebuffer_addr = framebuffer.get_framebuffer_addr() as u32;
    lcd.backbuffer_addr = framebuffer.get_backbuffer_addr() as u32;
    /*let mut layer1  = lcd::Layer {
                framebuffer: framebuffer,
    };*/
    

    // init touch screen
    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);

    touch::check_family_id(&mut i2c_3).unwrap();

    // clear both buffers
   /* layer1.clear();
    layer1.swap_buffers();
    layer1.clear();
    layer1.swap_buffers();*/

    let use_double_buffer = true;

    if !use_double_buffer {
        lcd.swap_buffers();
    }
    lcd.swap_buffers();

    //// INIT COMPLETE ////
    let mut fps = fps::init();
    fps.output_enabled = false;

    let red = &lcd::Color {
        red: 255,
        green: 0,
        blue: 0,
        alpha: 255,
    };
    let green = &lcd::Color {
        red: 128,
        green: 255,
        blue: 0,
        alpha: 255,
    };
    let blue = &lcd::Color {
        red: 0,
        green: 0,
        blue: 255,
        alpha: 255,
    };
    /*quad(30, 1 + 30, 50, &red, &mut layer1);
    quad(30, 1 + 30 + 80, 50, &green, &mut layer1);
    quad(30, 1 + 30 + 80 + 80, 50, &blue, &mut layer1);*/

    let mut current_color = red;

    let mut running_x = 40;
    let mut running_y = 0;

    let should_draw_now = interrupts::primask_mutex::PrimaskMutex::new(false);

    

    let mut last_frame = 0;
    loop {
        let current_time = system_clock::ticks();
        if current_time - last_frame < 16 {
            continue;
        }
        last_frame = current_time;

        logic(&mut running_x, &mut running_y);
        draw(&mut framebuffer, &running_x, &running_y, &current_color);
        // draw_number(&mut layer1, 0, 10, x);
        
         for i in 0..40 {
            quad(32, 32, 200, current_color, &mut framebuffer);
       }
         
        draw_fps(&mut framebuffer, &mut fps);

        for touch in &touch::touches(&mut i2c_3).unwrap() {
            framebuffer.set_pixel(touch.x as usize, touch.y as usize, *current_color);

            if in_rect(touch.x as usize, touch.y as usize, 30, 30, 50, 50) {
                current_color = &red;
            } else if in_rect(touch.x as usize, touch.y as usize, 30, 30 + 80, 50, 50) {
                current_color = green;
            } else if in_rect(touch.x as usize, touch.y as usize, 30, 30 + 80 + 80, 50, 50) {
                current_color = blue;
            }
        }
        if use_double_buffer {
            framebuffer.swap_buffers();
            lcd.swap_buffers();
        }
        fps.count_frame();
    }
}

fn draw(
    layer1: &mut lcd::FramebufferL8,
    running_x: &usize,
    running_y: &usize,
    current_color: &lcd::Color,
) {
    //for _x in 0..1000 {
    layer1.set_pixel(*running_x, *running_y, *current_color);

    // }
    // quad(50, 30, 40, current_color, layer1);
    //  quad(0,0, 20, &lcd::Color::rgb(0,0,0), layer1);
}

fn draw_fps(layer1: &mut lcd::FramebufferL8, fps: &mut fps::FpsCounter) {
    let mut number = fps.last_fps;
    if number > 99 {
        number = 99;
    }
    draw_number(layer1, 0, 0, number / 10);
    draw_number(layer1, 5, 0, number % 10);
}
fn draw_number(layer1: &mut lcd::FramebufferL8, x: usize, y: usize, number: usize) {
    if number == 0 {
        draw_seven_segment(layer1, x, y, true, true, true, false, true, true, true);
    } else if number == 1 {
        draw_seven_segment(layer1, x, y, false, false, true, false, false, true, false);
    } else if number == 2 {
        draw_seven_segment(layer1, x, y, true, false, true, true, true, false, true);
    } else if number == 3 {
        draw_seven_segment(layer1, x, y, true, false, true, true, false, true, true);
    } else if number == 4 {
        draw_seven_segment(layer1, x, y, false, true, true, true, false, true, false);
    } else if number == 5 {
        draw_seven_segment(layer1, x, y, true, true, false, true, false, true, true);
    } else if number == 6 {
        draw_seven_segment(layer1, x, y, true, true, false, true, true, true, true);
    } else if number == 7 {
        draw_seven_segment(layer1, x, y, true, false, true, false, false, true, false);
    } else if number == 8 {
        draw_seven_segment(layer1, x, y, true, true, true, true, true, true, true);
    } else if number == 9 {
        draw_seven_segment(layer1, x, y, true, true, true, true, false, true, true);
    }
}
fn draw_seven_segment(
    layer1: &mut lcd::FramebufferL8,
    x: usize,
    y: usize,
    top: bool,
    top_left: bool,
    top_right: bool,
    center: bool,
    bottom_left: bool,
    bottom_right: bool,
    bottom: bool,
) {
    let black = 0;
    let white = 255;
    layer1.set_pixel_direct(x + 0, y + 0, if top { white } else { black });
    layer1.set_pixel_direct(x + 1, y + 0, if top { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 0, if top { white } else { black });
    layer1.set_pixel_direct(x + 0, y + 1, if top_left { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 1, if top_right { white } else { black });
    layer1.set_pixel_direct(x + 0, y + 2, if top_left { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 2, if top_right { white } else { black });
    layer1.set_pixel_direct(x + 0, y + 3, if center { white } else { black });
    layer1.set_pixel_direct(x + 1, y + 3, if center { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 3, if center { white } else { black });
    layer1.set_pixel_direct(x + 0, y + 4, if bottom_left { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 4, if bottom_right { white } else { black });
    layer1.set_pixel_direct(x + 0, y + 5, if bottom_left { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 5, if bottom_right { white } else { black });
    layer1.set_pixel_direct(x + 0, y + 6, if bottom { white } else { black });
    layer1.set_pixel_direct(x + 1, y + 6, if bottom { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 6, if bottom { white } else { black });
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

fn quad(
    x: usize,
    y: usize,
    size: usize,
    color: &lcd::Color,
    layer1: &mut lcd::FramebufferL8,
) {
    for y in y..y + size {
        for x in x..x + size {
            layer1.set_pixel(x, y, *color);
        }
    }
}
