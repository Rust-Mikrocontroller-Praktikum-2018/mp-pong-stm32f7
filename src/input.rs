use stm32f7::{touch, i2c};
struct Input {
    top_left: bool,
    bottom_left: bool,
    top_right: bool,
    bottom_right: bool,
}
impl Input {
    pub fn is_up_pressed(&self) -> bool {
        self.top_left
    }
    pub fn is_down_pressed(&self) -> bool {
        self.bottom_left
    }

    // for local player 2
    pub fn is_up_pressed2(&self) -> bool {
        self.top_right
    }
    pub fn is_down_pressed2(&self) -> bool {
        self.bottom_right
    }

    pub fn evaluate_touch(
        &mut self,
        i2c_3: &mut i2c::I2C,
        racket_1_ypos_centre: u16,
        racket_2_ypos_centre: u16,
    ) {
        //reset
        self.top_left = false;
        self.bottom_left = false;
        self.top_right = false;
        self.bottom_right = false;

        // poll for new touch data
        for touch in &touch::touches(i2c_3).unwrap() {
            //Player_1
            if touch.x <= 199 {
                //if touch above current racket_position
                if touch.y < racket_1_ypos_centre {
                    self.top_left = true;
                }
                //if touch below current racket position
                else if touch.y > racket_1_ypos_centre {
                    self.bottom_left = true;
                }
            }
            // Player_2
            if touch.x >= 280 {
                //if touch above current racket_position
                if touch.y < racket_2_ypos_centre {
                    self.top_right = true;
                }
                //if touch below current racket position
                else if touch.y > racket_2_ypos_centre {
                    self.bottom_right = true;
                }
            }
        }
    }
}
