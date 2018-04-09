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
