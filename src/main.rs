#![no_std]
#![no_main]
#![feature(compiler_builtins_lib)]
#![feature(alloc)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]
#![feature(const_fn)]
#![feature(placement_in_syntax)]
#![allow(dead_code)] // TODO: remove if all features are used to find dead code4

extern crate compiler_builtins;
extern crate r0;
#[macro_use] // To get the hprintf! macro from semi-hosting
extern crate stm32f7_discovery as stm32f7;
#[macro_use]
extern crate alloc;
extern crate smoltcp;

mod ball;
mod fps;
mod game;
mod graphics;
mod input;
mod lcd; // use custom LCD implementation
mod menu;
mod network;
mod physics;
mod racket;

use core::mem::discriminant;
use core::ptr;
use embedded::interfaces::gpio::Gpio;
use game::GameState;
use lcd::Framebuffer;
use lcd::FramebufferL8;
use lcd::TextWriter;
use network::{Client, Server};
use smoltcp::wire::{EthernetAddress, Ipv4Address};
use stm32f7::lcd::Color;
use stm32f7::{board, embedded, ethernet, interrupts, sdram, system_clock, touch, i2c};

const USE_DOUBLE_BUFFER: bool = true;
const ENABLE_FPS_OUTPUT: bool = false;
const PRINT_START_MESSAGE: bool = false;
const BGCOLOR: u8 = 0;

const CLIENT_ETH_ADDR: EthernetAddress = EthernetAddress([0x00, 0x11, 0x22, 0x33, 0x44, 0x01]);
const CLIENT_IP_ADDR: Ipv4Address = Ipv4Address([141, 52, 46, 2]);
const SERVER_ETH_ADDR: EthernetAddress = EthernetAddress([0x00, 0x11, 0x22, 0x33, 0x44, 0x02]);
const SERVER_IP_ADDR: Ipv4Address = Ipv4Address([141, 52, 46, 1]);

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
    lcd.set_background_color(Color {
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

    // for i in 0..255 {
    // lcd.clut[i] = (0, i as u8, 0);
    // }
    // lcd.clut[255] = (255, 0, 0);
    // lcd.update_clut();

    // set up font renderer
    let mut loading_font = TextWriter::new(TTF, 40.0);
    loading_font.write(&mut framebuffer, "loading...");

    let mut menu_font = TextWriter::new(TTF, 20.0);
    let mut debug_font = TextWriter::new(TTF, 20.0);

    lcd.swap_buffers(); // show loading text
    framebuffer.swap_buffers();

    // init touch screen
    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);
    touch::check_family_id(&mut i2c_3).unwrap();

    let mut network = Some((ethernet_dma, ethernet_mac));

    let mut gamestate = GameState::Splash;
    let mut previous_gamestate = core::mem::discriminant(&gamestate); // Get the descriminant to be able to compare this

    interrupts::scope(
        nvic,
        |_| {},
        move |interrupt_table| {
            let mut should_draw_now = false;
            let should_draw_now_ptr = &mut should_draw_now as *mut bool as usize;

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
            hprintln!("Start run()");
            //// INIT COMPLETE ////
            let mut fps = fps::init();
            fps.output_enabled = ENABLE_FPS_OUTPUT;

            // Create Rackets
            let mut rackets: [racket::Racket; 2] = [racket::Racket::new(0), racket::Racket::new(1)];
            let mut ball: ball::Ball = ball::Ball::new();

            // setup local "network"
            let mut is_server = true; // Server is player 1

            let mut client = network::EthClient::new();
            let mut server = network::EthServer::new();
            let mut server_gamestate = network::GamestatePacket::new();

            let mut local_input_1 = network::InputPacket::new();
            let mut local_input_2 = network::InputPacket::new();

            let mut input = input::Input::new(i2c_3);

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

                    let just_entered_state = !(previous_gamestate == discriminant(&gamestate));
                    previous_gamestate = discriminant(&gamestate);

                    gamestate = match gamestate {
                        GameState::Splash => GameState::ChooseLocalOrNetwork,
                        GameState::ChooseLocalOrNetwork => menu::choose_local_network(
                            just_entered_state,
                            &mut framebuffer,
                            &mut menu_font,
                            &mut input,
                        ),
                        GameState::ChooseClientOrServer => menu::choose_client_server(
                            just_entered_state,
                            &mut framebuffer,
                            &mut menu_font,
                            &mut input,
                            &mut is_server,
                        ),
                        GameState::ChooseOnlyLocal => menu::choose_only_local(
                            just_entered_state,
                            &mut framebuffer,
                            &mut menu_font,
                            &mut input,
                        ),
                        GameState::ConnectToNetwork => {
                            framebuffer.clear();
                            loading_font.write_at(
                                &mut framebuffer,
                                "Initializing network...",
                                0,
                                0,
                            );
                            framebuffer.swap_buffers();
                            match network.take() {
                                Some((ethernet_dma, ethernet_mac)) => {
                                    let network_option = if is_server {
                                        network::init(
                                            rcc,
                                            syscfg,
                                            ethernet_mac,
                                            ethernet_dma,
                                            &mut gpio,
                                            SERVER_ETH_ADDR,
                                            SERVER_IP_ADDR,
                                            CLIENT_IP_ADDR,
                                        )
                                    } else {
                                        network::init(
                                            rcc,
                                            syscfg,
                                            ethernet_mac,
                                            ethernet_dma,
                                            &mut gpio,
                                            CLIENT_ETH_ADDR,
                                            CLIENT_IP_ADDR,
                                            SERVER_IP_ADDR,
                                        )
                                    };

                                    match network_option {
                                        Ok(network) => GameState::WaitForPartner(network),
                                        Err(e) => {
                                            framebuffer.clear();
                                            debug_font.write(
                                                &mut framebuffer,
                                                &format!("Network error: {:?}", e),
                                            );
                                            GameState::ChooseOnlyLocal
                                        }
                                    }
                                }
                                None => panic!(),
                            }
                        }
                        GameState::WaitForPartner(mut network) => {
                            if just_entered_state {
                                menu_font.write_at(
                                    &mut framebuffer,
                                    if is_server {
                                        "Waiting for client..."
                                    } else {
                                        "Waiting for server..."
                                    },
                                    0,
                                    50,
                                );
                            }

                            if is_server {
                                server.send_whoami(&mut network);
                                if server.is_client_connected(&mut network) {
                                    GameState::GameRunningNetwork(network)
                                } else {
                                    GameState::WaitForPartner(network)
                                }
                            } else {
                                client.send_whoami(&mut network);
                                if client.is_server_connected(&mut network) {
                                    GameState::GameRunningNetwork(network)
                                } else {
                                    GameState::WaitForPartner(network)
                                }
                            }
                        }
                        GameState::GameRunningLocal => {
                            game::game_loop_local(
                                just_entered_state,
                                &mut framebuffer,
                                &mut input,
                                &fps,
                                &mut rackets,
                                &mut ball,
                                &mut local_input_1,
                                &mut local_input_2,
                                &mut server_gamestate,
                                &mut loading_font,
                            );
                            GameState::GameRunningLocal
                        }
                        GameState::GameRunningNetwork(mut network) => {
                            game::game_loop_network(
                                just_entered_state,
                                &mut framebuffer,
                                &mut input,
                                &fps,
                                &mut rackets,
                                &mut ball,
                                &mut client,
                                &mut server,
                                &mut local_input_1,
                                &mut server_gamestate,
                                is_server,
                                &mut network,
                                &mut loading_font,
                            );
                            GameState::GameRunningNetwork(network)
                        }
                    };

                    graphics::draw_guidelines(&mut framebuffer);
                    graphics::draw_fps(&mut framebuffer, &fps);
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
