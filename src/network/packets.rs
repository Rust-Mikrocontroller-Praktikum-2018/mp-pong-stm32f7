use alloc::Vec;

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

    pub fn get_racket_ypos(&self, id: usize) -> i16 {
        self.rackets[id].y
    }
    pub fn set_racket_ypos(&mut self, id: usize, new_racket_ypos: i16) {
        self.rackets[id].y = new_racket_ypos;
    }
}

impl InputPacket {
    pub fn new() -> InputPacket {
        InputPacket {
            up: false,
            down: false,
        }
    }
    pub fn says_move_up(&self) -> i8 {
        if self.up != self.down {
            if self.up {
                -1
            } else {
                1
            }
        } else {
            0
        }
    }
}

pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(input: &[u8]) -> Self;
    fn len() -> usize;
}

impl Serializable for GamestatePacket {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.rackets[0].serialize());
        result.extend(self.rackets[1].serialize());
        result.extend(self.ball.serialize());
        result.push(self.score[0]);
        result.push(self.score[1]);
        result
    }

    fn deserialize(input: &[u8]) -> GamestatePacket {
        let mut index = 0;
        let racket1 = RacketPacket::deserialize(&input[index..index + RacketPacket::len()]);
        index += RacketPacket::len();
        let racket2 = RacketPacket::deserialize(&input[index..index + RacketPacket::len()]);
        index += RacketPacket::len();
        let ball = BallPacket::deserialize(&input[index..index + BallPacket::len()]);
        index += BallPacket::len();
        let score_player1 = input[index];
        let score_player2 = input[index + 1];

        GamestatePacket {
            rackets: [racket1, racket2],
            ball,
            score: [score_player1, score_player2],
        }
    }

    fn len() -> usize {
        2 * RacketPacket::len() + BallPacket::len() + 2
    }
}

impl Serializable for RacketPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.push(upper_byte(self.x));
        result.push(lower_byte(self.x));
        result.push(upper_byte(self.y));
        result.push(lower_byte(self.y));
        result
    }

    fn deserialize(input: &[u8]) -> RacketPacket {
        RacketPacket {
            x: merge(input[0], input[1]),
            y: merge(input[2], input[3]),
        }
    }
    fn len() -> usize {
        4
    }
}

impl Serializable for BallPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.push(upper_byte(self.x));
        result.push(lower_byte(self.x));
        result.push(upper_byte(self.y));
        result.push(lower_byte(self.y));
        result.push(upper_byte(self.x_vel));
        result.push(lower_byte(self.x_vel));
        result.push(upper_byte(self.y_vel));
        result.push(lower_byte(self.y_vel));
        result
    }

    fn deserialize(input: &[u8]) -> BallPacket {
        BallPacket {
            x: merge(input[0], input[1]),
            y: merge(input[2], input[3]),
            x_vel: merge(input[4], input[5]),
            y_vel: merge(input[6], input[7]),
        }
    }
    fn len() -> usize {
        8
    }
}

impl Serializable for InputPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut result: u8 = 0;
        if self.up {
            result |= 0b0000_0001;
        }
        if self.down {
            result |= 0b0000_0010;
        }

        vec![result]
    }

    fn deserialize(input: &[u8]) -> InputPacket {
        InputPacket {
            up: input[0] & 0b0000_0001 != 0,
            down: input[0] & 0b0000_0010 != 0,
        }
    }

    fn len() -> usize {
        1
    }
}

fn upper_byte(input: i16) -> u8 {
    ((input >> 8) & 0xff) as u8
}
fn lower_byte(input: i16) -> u8 {
    (input & 0xff) as u8
}

fn merge(upper: u8, lower: u8) -> i16 {
    i16::from(upper) << 8 | i16::from(lower)
}
