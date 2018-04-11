use i2c;
use network::InputPacket;
use touch;

pub struct Input {
    i2c_3: i2c::I2C,
    touch_last_frame: bool,
}

pub struct Touch {
    pub is_down: bool,
    pub x: u16,
    pub y: u16,
    pub any_touch_last_frame: bool, // Was there a touch in the last frame?
}

impl Input {
    pub fn new(i2c_3: i2c::I2C) -> Input {
        Input { i2c_3: i2c_3, touch_last_frame: false }
    }

    pub fn evaluate_touch_two_players(
        &mut self,
        local_input_1: &mut InputPacket,
        local_input_2: &mut InputPacket,
    ) {
        // poll for new touch data
        for touch in &touch::touches(&mut self.i2c_3).unwrap() {
            // Player_1
            if touch.x <= 199 {
                local_input_1.goal_y = touch.y as i16;
            }
            // Player_2
            if touch.x >= 280 {
                local_input_2.goal_y = touch.y as i16;
            }
        }
    }

    pub fn evaluate_touch_one_player(&mut self, local_input_1: &mut InputPacket) {
        // poll for new touch data
        for touch in &touch::touches(&mut self.i2c_3).unwrap() {
            local_input_1.goal_y = touch.y as i16;
        }
    }

    pub fn handle_menu(&mut self) -> Touch {
        let mut result = Touch {
            is_down: false,
            x: 0,
            y: 0,
            any_touch_last_frame: self.touch_last_frame,
        };
        self.touch_last_frame = false;
        for touch in &touch::touches(&mut self.i2c_3).unwrap() {
            self.touch_last_frame = true;
            result.is_down = true;
            result.x = touch.x;
            result.y = touch.y;
            break; // only get the first touch
        }
        result
    }
}
