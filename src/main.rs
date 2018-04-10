#![no_std]
#![no_main]
#![feature(compiler_builtins_lib)]
#![feature(alloc)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]
#![feature(const_fn)]
#![allow(dead_code)] // TODO: remove if all features are used to find dead code4

extern crate compiler_builtins;
extern crate r0;
#[macro_use] // To get the hprintf! macro from semi-hosting
extern crate stm32f7_discovery as stm32f7;
#[macro_use]
extern crate alloc;
extern crate smoltcp;

mod fps;
mod graphics;
mod input;
mod lcd; // use custom LCD implementation
mod network;
mod physics;
mod racket;

use core::ptr;
use embedded::interfaces::gpio::Gpio;
use input::Input;
use lcd::Framebuffer;
use lcd::FramebufferL8;
use network::Network;
use network::{Client, EthClient, EthServer, GamestatePacket, InputPacket, Server};
use smoltcp::wire::{EthernetAddress, Ipv4Address};
use stm32f7::{board, embedded, ethernet, interrupts, sdram, system_clock, touch, i2c};
use stm32f7::lcd::FontRenderer;

const USE_DOUBLE_BUFFER: bool = true;
const ENABLE_FPS_OUTPUT: bool = false;
const PRINT_START_MESSAGE: bool = false;
const BGCOLOR: lcd::Color = lcd::Color::rgb(0, 0, 0);

const ETH_ADDR: EthernetAddress = EthernetAddress([0x00, 0x08, 0xdc, 0xab, 0xcd, 0xef]);
const IP_ADDR: Ipv4Address = Ipv4Address([141, 52, 46, 198]);
const PARTNER_IP_ADDR: Ipv4Address = Ipv4Address([141, 52, 46, 1]);

static TTF: &[u8] = include_bytes!("../res/RobotoMono-Bold.ttf");

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
        hprintln!(
            "\n[38;5;40m[1m🔦 Flash complete! ✔️\n[38;5;45m🚀 Program started.(B[m"
        );
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
        nvic,
        ethernet_mac,
        ethernet_dma,
        syscfg,
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

    // set up LCD
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

    // set up font renderer
    let font_renderer = FontRenderer::new(TTF, 40.0);
    let x_pos = &mut 30;
    let y_pos = &mut 100;
    let font_height = font_renderer.font_height() as usize;

    font_renderer.render("loading game...", |x, y, v| {
            if *x_pos + x >= lcd::WIDTH {
                *x_pos = 0;
                *y_pos += font_height;
            }
            if *y_pos + font_height >= lcd::HEIGHT {
                *y_pos = 0;
                // TODO: no place for text D:
            }
            let alpha = (v * 255.0 + 0.5) as u8;
            framebuffer.set_pixel_direct(*x_pos + x, *y_pos + y, alpha);
    });
    lcd.swap_buffers();
    framebuffer.swap_buffers();


    // init touch screen
    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);
    touch::check_family_id(&mut i2c_3).unwrap();


    let mut network = network::init(
        rcc,
        syscfg,
        ethernet_mac,
        ethernet_dma,
        &mut gpio,
        ETH_ADDR,
        IP_ADDR,
        PARTNER_IP_ADDR,
    ); // TODO: error handling

    interrupts::scope(
        nvic,
        |_| {},
        move |interrupt_table| {
            let should_draw_now = false;
            let should_draw_now_ptr = (&should_draw_now as *const bool) as usize;

            let _interrupt_handler = interrupt_table
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

            framebuffer.clear();
            hprintln!("Start run()");
            //// INIT COMPLETE ////
            let mut fps = fps::init();
            fps.output_enabled = ENABLE_FPS_OUTPUT;

            // Create Rackets
            let mut rackets: [racket::Racket; 2] = [racket::Racket::new(0), racket::Racket::new(1)];
            // Draw Start Position
            for racket in rackets.iter_mut() {
                racket.draw_racket(&mut framebuffer);
            }

            // setup local "network"
            let is_server = false; // Server is player 1
            let is_local = true;

            let mut client = network::EthClient::new();
            let mut server = network::EthServer::new();
            let mut server_gamestate = network::GamestatePacket::new();

            let mut local_input_1 = network::InputPacket::new();
            let mut local_input_2 = network::InputPacket::new();


            loop {
                let need_draw; // This memory space is accessed directly to achive synchronisation. Very unsafe!
                unsafe {
                    // Frame synchronisation
                    need_draw = ptr::read_volatile(should_draw_now_ptr as *mut bool);
                }
                if need_draw {
                    if USE_DOUBLE_BUFFER {
                        framebuffer.swap_buffers();
                    }

                    if is_local {
                        game_loop_local(
                            &mut framebuffer,
                            &mut i2c_3,
                            &fps,
                            &mut rackets,
                            &mut local_input_1,
                            &mut local_input_2,
                            &mut server_gamestate,
                        )
                    } else {
                        game_loop_network(
                            &mut framebuffer,
                            &mut i2c_3,
                            &fps,
                            &mut rackets,
                            &mut client,
                            &mut server,
                            &mut local_input_1,
                            &mut local_input_2,
                            &mut server_gamestate,
                            is_server,
                            network.as_mut().unwrap(),
                        );
                    }

                    // end of frame
                    fps.count_frame();
                    unsafe {
                        ptr::write_volatile(should_draw_now_ptr as *mut bool, false);
                    }
                }
            }
        },
    )
}

fn game_loop_local(
    framebuffer: &mut FramebufferL8,
    i2c_3: &mut i2c::I2C,
    fps: &fps::FpsCounter,
    rackets: &mut [racket::Racket; 2],
    local_input_1: &mut InputPacket,
    local_input_2: &mut InputPacket,
    local_gamestate: &mut GamestatePacket,
) {
    handle_local_calculations(local_gamestate, local_input_1, local_input_2);

    // handle input
    let input = input::evaluate_touch(
        i2c_3,
        rackets[0].get_ypos_centre(),
        rackets[1].get_ypos_centre(),
    );
    local_input_1.up = input.is_up_pressed();
    local_input_1.down = input.is_down_pressed();
    local_input_2.up = input.is_up_pressed2();
    local_input_2.down = input.is_down_pressed2();

    // move rackets and ball
    graphics::update_graphics(framebuffer, local_gamestate, rackets);

    graphics::draw_fps(framebuffer, fps);
}

fn game_loop_network(
    framebuffer: &mut FramebufferL8,
    i2c_3: &mut i2c::I2C,
    fps: &fps::FpsCounter,
    rackets: &mut [racket::Racket; 2],
    client: &mut EthClient,
    server: &mut EthServer,
    local_input_1: &mut InputPacket,
    local_input_2: &mut InputPacket,
    local_gamestate: &mut GamestatePacket,
    is_server: bool,
    network: &mut Network,
) {
    if is_server {
        handle_network_server(server, network, local_gamestate, local_input_1);
    } else {
        handle_network_client(client, network, local_gamestate, local_input_1);
    }

    // handle input
    let input = input::evaluate_touch(
        i2c_3,
        rackets[0].get_ypos_centre(),
        rackets[1].get_ypos_centre(),
    );

    local_input_1.up = input.is_up_pressed() || input.is_up_pressed2();
    local_input_1.down = input.is_down_pressed() || input.is_down_pressed2();

    // move rackets and ball
    graphics::update_graphics(framebuffer, local_gamestate, rackets);

    graphics::draw_fps(framebuffer, fps);
}

fn handle_local_calculations(
    local_gamestate: &mut GamestatePacket,
    local_input_1: &InputPacket,
    local_input_2: &InputPacket,
) {
    let inputs = [*local_input_1, *local_input_2];

    physics::calculate_physics(local_gamestate, inputs);

}

fn handle_network_server(
    server: &mut EthServer,
    network: &mut Network,
    local_gamestate: &mut GamestatePacket,
    local_input_1: &InputPacket,
) {
    let inputs = [*local_input_1, server.receive_input(network)];

    physics::calculate_physics(local_gamestate, inputs);

    server.send_gamestate(network, local_gamestate);
}

fn handle_network_client(
    client: &mut EthClient,
    network: &mut Network,
    local_gamestate: &mut GamestatePacket,
    local_input_1: &InputPacket,
) {
    *local_gamestate = client.receive_gamestate(network);
    client.send_input(network, local_input_1);
}

