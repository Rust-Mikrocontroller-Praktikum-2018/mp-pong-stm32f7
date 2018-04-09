pub struct GamestatePacket {
    rackets: [RacketPacket; 22],
    ball: BallPacket,
    score: [u8; 2],
}

struct RacketPacket {
    x: s16, // center_x
    y: s16, // center_y
}

struct BallPacket {
    x: s16, // center_x
    y: s16, // center_y
    x_vel: s16,
    y_vel: s16,
}

pub struct InputPacket {
    up: bool,
    down: bool,
}

impl GamestatePacket {
    pub fn new() -> GamestatePacket {
        GamestatePacket {
            rackets: [Racket { x: 0, y: 100 }, Racket { x: 400, y: 100 }],
            ball: Ball {
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

trait Client {
    fn send_input(&mut self, input: &InputPacket);
    fn receive_gamestate(&self) -> GamestatePacket;
}

trait Server {
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
        return self.gamestate;
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
        return self.player_inputs;
    }
    fn send_gamestate(&mut self, gamestate: &GamestatePacket) {
        self.gamestate = *gamestate;
    }
}

pub fn handle_local(client1: &mut LocalClient, client2: &mut LocalClient, server: &mut LocalServer) {
    client1.gamestate = server.gamestate;
    client2.gamestate = server.gamestate;
    server.player_inputs = [client1.input, client2.input];
}
