use fps;
use graphics;
use input;
use lcd::FramebufferL8;
use lcd::Framebuffer;
use network::{Client, EthClient, EthServer, GamestatePacket, InputPacket, Network, Server};
use physics;
use racket;
use stm32f7::i2c;

#[derive(PartialEq, Clone)]
pub enum GameState {
    Splash,
    ChooseLocalOrNetwork,
    ChooseClientOrServer,
    ConnectToNetwork,
    GameRunningLocal,
    GameRunningNetwork,
}

pub fn game_loop_local(
    just_entered_state: bool,
    framebuffer: &mut FramebufferL8,
    i2c_3: &mut i2c::I2C,
    fps: &fps::FpsCounter,
    rackets: &mut [racket::Racket; 2],
    ball:&mut ball::Ball,
    local_input_1: &mut InputPacket,
    local_input_2: &mut InputPacket,
    local_gamestate: &mut GamestatePacket,
) {
    if just_entered_state {
        framebuffer.clear();
        graphics::draw_initial(framebuffer, rackets);
    }

    handle_local_calculations(local_gamestate, local_input_1, local_input_2);

    // handle input
    input::evaluate_touch_two_players(i2c_3, local_input_1, local_input_2);

    // move rackets and ball
    graphics::update_graphics(framebuffer, local_gamestate, rackets,ball);

    graphics::draw_fps(framebuffer, fps);
}

pub fn game_loop_network(
    just_entered_state: bool, 
    framebuffer: &mut FramebufferL8,
    i2c_3: &mut i2c::I2C,
    fps: &fps::FpsCounter,
    rackets: &mut [racket::Racket; 2],
    ball:&mut ball::Ball,
    client: &mut EthClient,
    server: &mut EthServer,
    local_input_1: &mut InputPacket,
    local_gamestate: &mut GamestatePacket,
    is_server: bool,
    network: &mut Network,
) {
    if just_entered_state {
        framebuffer.clear();
        graphics::draw_initial(framebuffer, rackets);
    }

    if is_server {
        handle_network_server(server, network, local_gamestate, local_input_1);
    } else {
        handle_network_client(client, network, local_gamestate, local_input_1);
    }

    // handle input
    input::evaluate_touch_one_player(i2c_3, local_input_1);

    // move rackets and ball
    graphics::update_graphics(framebuffer, local_gamestate, rackets,ball);

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
