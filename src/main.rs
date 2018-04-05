#![no_std]
#![no_main]
#![feature(compiler_builtins_lib)]

extern crate compiler_builtins;
extern crate r0;
extern crate stm32f7_discovery as stm32f7;

use embedded::interfaces::gpio::Gpio;
use stm32f7::{board, embedded, lcd, sdram, system_clock, touch, i2c};

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

    // Initialize the floating point unit
    let scb = stm32f7::cortex_m::peripheral::scb_mut();
    scb.cpacr.modify(|v| v | 0b1111 << 20);

    main(board::hw());
}

fn main(hw: board::Hardware) -> ! {
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
    let mut layer1 = lcd.layer_1().unwrap();
    let mut layer2 = lcd.layer_2().unwrap();


    // init touch screen
    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);

    touch::check_family_id(&mut i2c_3).unwrap();


    layer2.clear();
    layer1.clear();
    layer1.clear();

    let lookup = [
        136, 137, 139, 141, 143, 144, 146, 148, 150, 151, 153, 155, 157, 158, 160, 162, 164, 165,
        167, 169, 171, 172, 174, 176, 177, 179, 181, 182, 184, 186, 187, 189, 191, 192, 194, 195,
        197, 199, 200, 202, 203, 205, 206, 208, 209, 211, 212, 214, 215, 217, 218, 219, 221, 222,
        224, 225, 226, 227, 229, 230, 231, 233, 234, 235, 236, 237, 239, 240, 241, 242, 243, 244,
        245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 255, 256, 257, 258, 259, 259, 260,
        261, 261, 262, 263, 263, 264, 264, 265, 265, 266, 266, 267, 267, 268, 268, 268, 269, 269,
        269, 270, 270, 270, 270, 270, 271, 271, 271, 271, 271, 271, 271, 271, 271, 271, 271, 271,
        271, 270, 270, 270, 270, 270, 269, 269, 269, 268, 268, 268, 267, 267, 266, 266, 265, 265,
        264, 264, 263, 263, 262, 261, 261, 260, 259, 259, 258, 257, 256, 255, 255, 254, 253, 252,
        251, 250, 249, 248, 247, 246, 245, 244, 243, 242, 241, 240, 239, 237, 236, 235, 234, 233,
        231, 230, 229, 227, 226, 225, 224, 222, 221, 219, 218, 217, 215, 214, 212, 211, 209, 208,
        206, 205, 203, 202, 200, 199, 197, 195, 194, 192, 191, 189, 187, 186, 184, 182, 181, 179,
        177, 176, 174, 172, 171, 169, 167, 165, 164, 162, 160, 158, 157, 155, 153, 151, 150, 148,
        146, 144, 143, 141, 139, 137, 136, 134, 132, 130, 128, 127, 125, 123, 121, 120, 118, 116,
        114, 113, 111, 109, 107, 106, 104, 102, 100, 99, 97, 95, 94, 92, 90, 89, 87, 85, 84, 82,
        80, 79, 77, 76, 74, 72, 71, 69, 68, 66, 65, 63, 62, 60, 59, 57, 56, 54, 53, 52, 50, 49, 47,
        46, 45, 44, 42, 41, 40, 38, 37, 36, 35, 34, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21,
        20, 19, 18, 17, 16, 16, 15, 14, 13, 12, 12, 11, 10, 10, 9, 8, 8, 7, 7, 6, 6, 5, 5, 4, 4, 3,
        3, 3, 2, 2, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2,
        2, 3, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 10, 10, 11, 12, 12, 13, 14, 15, 16, 16, 17,
        18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 34, 35, 36, 37, 38, 40, 41, 42,
        44, 45, 46, 47, 49, 50, 52, 53, 54, 56, 57, 59, 60, 62, 63, 65, 66, 68, 69, 71, 72, 74, 76,
        77, 79, 80, 82, 84, 85, 87, 89, 90, 92, 94, 95, 97, 99, 100, 102, 104, 106, 107, 109, 111,
        113, 114, 116, 118, 120, 121, 123, 125, 127, 128, 130, 132, 134,
    ];

    let mut end = 480;
    if end > lookup.len() {
        end = lookup.len() - 1;
    }

    for x in 0..end {
        layer1.print_point_at(x, lookup[x]);
    }

    /*    let mut rect = |x: usize, y: usize, width: usize, height: usize, color: &lcd::Color| {
        for x in x..x + width {
            for y in y..y + height {
                layer1.print_point_color_at(x, y, *color);
            }
        }
    };*/

    let mut quad = |x: usize, y: usize, size: usize, color: &lcd::Color| {
        for x in x..x + size {
            for y in y..y + size {
                layer1.print_point_color_at(x, y, *color);
            }
        }
    };

    let red = lcd::Color {
        red: 255,
        green: 0,
        blue: 0,
        alpha: 255,
    };
    let blue = lcd::Color {
        red: 0,
        green: 0,
        blue: 255,
        alpha: 255,
    };

    let mut step = 5;

    loop {
        let mut x = (480 - 272) / 2;
        let mut y = 0;
        let mut width: i32 = 272;

        let mut color_blue = false;

        loop {
            if color_blue {
                quad(x, y, width as usize, &red);
            } else {
                quad(x, y, width as usize, &blue);
            }
            color_blue = !color_blue;
            x += step;
            y += step;
            width -= 2 * step as i32;
            if width < 0 {
                break;
            }
        }
        step -= 1;
    }

    /*

    quad(0, 0, 200, 200, );*/
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
