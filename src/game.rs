use ball;
use fps;
use graphics;
use graphics::GraphicsCache;
use input::Input;
use lcd::Framebuffer;
use lcd::FramebufferL8;
use lcd::TextWriter;
use network::{Client, EthClient, EthServer, GamestatePacket, InputPacket, Network, Server};
use physics;
use racket;
use network::packets::STATE_WON_PLAYER_1;
use physics::PhysicsCache;

pub enum GameState {
    Splash,
    ChooseLocalOrNetwork,
    ChooseClientOrServer,
    ChooseOnlyLocal,
    ConnectToNetwork,
    GameRunningLocal,
    WaitForPartner(Network),
    GameRunningNetwork(Network),
}

pub fn game_loop_local(
    just_entered_state: bool,
    framebuffer: &mut FramebufferL8,
    input: &mut Input,
    fps: &fps::FpsCounter,
    rackets: &mut [racket::Racket; 2],
    ball: &mut ball::Ball,
    local_input_1: &mut InputPacket,
    local_input_2: &mut InputPacket,
    local_gamestate: &mut GamestatePacket,
    menu_font: &mut TextWriter,
    cache: &mut GraphicsCache,
    total_time: usize,
    delta_time: usize,
    physics_cache: &mut PhysicsCache,
) {
    if just_entered_state {
        framebuffer.clear();
        graphics::draw_initial(framebuffer, rackets, ball);
    }

    handle_local_calculations(local_gamestate, local_input_1, local_input_2, total_time, physics_cache);

    // handle input
    if local_gamestate.state >= STATE_WON_PLAYER_1 {
        let touch = input.handle_menu();
        if touch.is_down && !touch.any_touch_last_frame {
            *local_gamestate = GamestatePacket::new(total_time);
            *physics_cache = PhysicsCache::new();
            *local_input_1 = InputPacket::new();
            *local_input_2 = InputPacket::new();
        }
    } else {
        input.evaluate_touch_two_players(local_input_1, local_input_2);
    }

    // move rackets and ball
    graphics::update_graphics(
        framebuffer,
        local_gamestate,
        rackets,
        ball,
        menu_font,
        cache,
        total_time,
        delta_time,
    );

    graphics::draw_fps(framebuffer, fps);
}

pub fn game_loop_network(
    just_entered_state: bool,
    framebuffer: &mut FramebufferL8,
    input: &mut Input,
    fps: &fps::FpsCounter,
    rackets: &mut [racket::Racket; 2],
    ball: &mut ball::Ball,
    client: &mut EthClient,
    server: &mut EthServer,
    local_input_1: &mut InputPacket,
    local_gamestate: &mut GamestatePacket,
    is_server: bool,
    network: &mut Network,
    menu_font: &mut TextWriter,
    cache: &mut GraphicsCache,
    total_time: usize,
    delta_time: usize,
    physics_cache: &mut PhysicsCache,
) {
    if just_entered_state {
        framebuffer.clear();
        graphics::draw_initial(framebuffer, rackets, ball);
    }

    if is_server {
        handle_network_server(server, network, local_gamestate, local_input_1, total_time, physics_cache);
    } else {
        handle_network_client(client, network, local_gamestate, local_input_1);
    }

    if is_server && local_gamestate.state >= STATE_WON_PLAYER_1
     {
        let touch = input.handle_menu();
        if touch.is_down && !touch.any_touch_last_frame {
            *local_gamestate = GamestatePacket::new(total_time);
            *physics_cache = PhysicsCache::new();
            *local_input_1 = InputPacket::new();
        }
    } else {
        // handle input
        input.evaluate_touch_one_player(local_input_1);
    }
    // move rackets and ball
    graphics::update_graphics(
        framebuffer,
        local_gamestate,
        rackets,
        ball,
        menu_font,
        cache,
        total_time,
        delta_time,
    );

    graphics::draw_fps(framebuffer, fps);
}

fn handle_local_calculations(
    local_gamestate: &mut GamestatePacket,
    local_input_1: &InputPacket,
    local_input_2: &InputPacket,
    total_time: usize,
    physics_cache: &mut PhysicsCache,
) {
    let inputs = [*local_input_1, *local_input_2];
    physics::calculate_physics(local_gamestate, inputs, total_time, physics_cache);
}

fn handle_network_server(
    server: &mut EthServer,
    network: &mut Network,
    local_gamestate: &mut GamestatePacket,
    local_input_1: &InputPacket,
    total_time: usize,
    physics_cache: &mut PhysicsCache,
) {
    let inputs = [*local_input_1, server.receive_input(network)];
    physics::calculate_physics(local_gamestate, inputs, total_time, physics_cache);
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
