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

    // TODO: improve this by passing the current position of the rackets
    pub fn evaluate_input(&mut self, i2c_3: &mut i2c::I2C) {
        self.top_left = false;
        self.top_right = false;
        self.bottom_left = false;
        self.bottom_right = false;

        // poll for new touch data
        for touch in &touch::touches(i2c_3).unwrap() {
            // Player_1
            if touch.x <= 199 {
                if touch.y < 136 {
                    // up
                    self.top_left = true;
                } else {
                    // down
                    self.bottom_left = true;
                }
            }
            // Player_2
            if touch.x >= 280 {
                if touch.y < 136 {
                    self.top_right = true;
                } else {
                    self.bottom_right = true;
                }
            }
        }
    }
}
