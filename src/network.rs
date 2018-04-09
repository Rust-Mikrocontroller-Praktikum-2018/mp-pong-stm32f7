#[derive(Debug, Copy, Clone)]
pub struct GamestatePacket {
    pub rackets: [RacketPacket; 2],
    pub ball: BallPacket,
    pub score: [u8; 2],
}
#[derive(Debug, Copy, Clone)]
pub struct RacketPacket {
    pub x: i16, // center_x
    pub y: i16, // center_y
}
#[derive(Debug, Copy, Clone)]
pub struct BallPacket {
    pub x: i16, // center_x
    pub y: i16, // center_y
    pub x_vel: i16,
    pub y_vel: i16,
}
#[derive(Debug, Copy, Clone)]
pub struct InputPacket {
    pub up: bool,
    pub down: bool,
}

impl GamestatePacket {
    pub fn new() -> GamestatePacket {
        GamestatePacket {
            rackets: [
                RacketPacket { x: 0, y: 100 },
                RacketPacket { x: 400, y: 100 },
            ],
            ball: BallPacket {
                x: 200,
                y: 100,
                x_vel: 1,
                y_vel: 1,
            },
            score: [0, 0],
        }
    }
}

impl InputPacket {
    pub fn new() -> InputPacket {
        InputPacket {
            up: false,
            down: false,
        }
    }
}

pub trait Client {
    fn send_input(&mut self, input: &InputPacket);
    fn receive_gamestate(&self) -> GamestatePacket;
}

pub trait Server {
    fn receive_inputs(&self) -> [InputPacket; 2];
    fn send_gamestate(&mut self, gamestate: &GamestatePacket);
}

pub struct LocalClient {
    gamestate: GamestatePacket,
    input: InputPacket,
}

impl LocalClient {
    pub fn new() -> LocalClient {
        LocalClient {
            gamestate: GamestatePacket::new(),
            input: InputPacket::new(),
        }
    }
}

impl Client for LocalClient {
    fn send_input(&mut self, input: &InputPacket) {
        self.input = *input;
    }
    fn receive_gamestate(&self) -> GamestatePacket {
        self.gamestate
    }
}

pub struct LocalServer {
    gamestate: GamestatePacket,
    player_inputs: [InputPacket; 2],
}

impl LocalServer {
    pub fn new() -> LocalServer {
        LocalServer {
            gamestate: GamestatePacket::new(),
            player_inputs: [InputPacket::new(), InputPacket::new()],
        }
    }
}

impl Server for LocalServer {
    fn receive_inputs(&self) -> [InputPacket; 2] {
        self.player_inputs
    }
    fn send_gamestate(&mut self, gamestate: &GamestatePacket) {
        self.gamestate = *gamestate;
    }
}

pub fn handle_local(
    client1: &mut LocalClient,
    client2: &mut LocalClient,
    server: &mut LocalServer,
) {
    client1.gamestate = server.gamestate;
    client2.gamestate = server.gamestate;
    server.player_inputs = [client1.input, client2.input];
}
