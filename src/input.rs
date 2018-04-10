use i2c;
use network::InputPacket;
use touch;

pub fn evaluate_touch_two_players(
    i2c_3: &mut i2c::I2C,
    local_input_1: &mut InputPacket,
    local_input_2: &mut InputPacket,
) {
    // poll for new touch data
    for touch in &touch::touches(i2c_3).unwrap() {
        //Player_1
        if touch.x <= 199 {
            local_input_1.goal_y = touch.y as i16;
        }
        // Player_2
        if touch.x >= 280 {
            local_input_2.goal_y = touch.y as i16;
        }
    }
}

pub fn evaluate_touch_one_player(i2c_3: &mut i2c::I2C, local_input_1: &mut InputPacket) {
    // poll for new touch data
    for touch in &touch::touches(i2c_3).unwrap() {
        local_input_1.goal_y = touch.y as i16;
    }
}
