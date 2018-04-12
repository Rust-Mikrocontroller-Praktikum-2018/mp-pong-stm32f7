use alloc::Vec;
use lcd::{HEIGHT, WIDTH};
use racket::RACKET_WIDTH;

const BALL_MAX_SPEED: i16 = 20;
const BALL_MIN_SPEED: i16 = 10;

const STATE_RUNNING: u8 = 0;
const STATE_WON_PLAYER_1: u8 = 100;
const STATE_WON_PLAYER_2: u8 = 100;

#[derive(Debug, Copy, Clone)]
pub struct GamestatePacket {
    pub rackets: [RacketPacket; 2],
    pub ball: BallPacket,
    pub score: [u8; 2],
    pub state: u8,
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
    pub goal_y: i16,
}
#[derive(Debug, Copy, Clone)]
pub struct WhoamiPacket {
    pub is_server: bool,
}

impl GamestatePacket {
    pub fn new() -> GamestatePacket {
        GamestatePacket {
            rackets: [
                RacketPacket {
                    x: RACKET_WIDTH as i16,
                    y: (HEIGHT / 2) as i16,
                },
                RacketPacket {
                    x: WIDTH as i16 - RACKET_WIDTH as i16,
                    y: (HEIGHT / 2) as i16,
                },
            ],
            ball: BallPacket {
                x: (WIDTH / 2) as i16,
                y: (HEIGHT / 2) as i16,
                x_vel: random_vel()[0],
                y_vel: random_vel()[1],
            },
            score: [0, 0],
            state: 0,
        }
    }
}
impl BallPacket {
    pub fn reset(&mut self) {
        self.x = (WIDTH / 2) as i16;
        self.y = (HEIGHT / 2) as i16;
        self.x_vel = random_vel()[0];
        self.y_vel = random_vel()[1];
    }
}

impl InputPacket {
    pub fn new() -> InputPacket {
        InputPacket { goal_y: 272 / 2 }
    }
}

impl WhoamiPacket {
    pub fn new(is_server: bool) -> WhoamiPacket {
        WhoamiPacket {
            is_server: is_server,
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
        result.push(self.state);
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
        let state = input[index + 1];

        GamestatePacket {
            rackets: [racket1, racket2],
            ball,
            score: [score_player1, score_player2],
            state: state,
        }
    }

    fn len() -> usize {
        2 * RacketPacket::len() + BallPacket::len() + 2 + 1
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
        let mut result = Vec::new();
        result.push(upper_byte(self.goal_y));
        result.push(lower_byte(self.goal_y));
        result
    }

    fn deserialize(input: &[u8]) -> InputPacket {
        InputPacket {
            goal_y: merge(input[0], input[1]),
        }
    }

    fn len() -> usize {
        2
    }
}

impl Serializable for WhoamiPacket {
    fn serialize(&self) -> Vec<u8> {
        if self.is_server {
            vec![255]
        } else {
            vec![0]
        }
    }

    fn deserialize(input: &[u8]) -> WhoamiPacket {
        WhoamiPacket {
            is_server: input[0] == 255,
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
fn random_vel() -> [i16; 2] {
    let mut random_x_vel = 5;
    let mut random_y_vel = 5;
    // TODO generate random velocity between BALL_MIN_SPEED and BALL_MAX_SPEED

    [random_x_vel, random_y_vel]
}
