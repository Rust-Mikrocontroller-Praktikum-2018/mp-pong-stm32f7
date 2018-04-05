#![no_std]
#![no_main]
#![feature(compiler_builtins_lib)]

extern crate compiler_builtins;
extern crate r0;
extern crate stm32f7_discovery as stm32f7;

use stm32f7::{board, embedded, system_clock};

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
        rcc, pwr, flash, ..
    } = hw;

    system_clock::init(rcc, pwr, flash);

    /*
// high level

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

    loop {}
}
