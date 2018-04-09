struct Input{


}
impl Input{
pub fn is_up_pressed() -> bool{
    false
}
pub fn is_down_pressed() -> bool {
    false
}

// for local player 2
pub fn is_up_pressed2() -> bool {
    false
}
pub fn is_down_pressed2() -> bool {
    false
}
pub fn evaluate_input(){ //MOVE TO INPUT.RS
    // poll for new touch data
    for touch in &touch::touches(i2c_3).unwrap() {
        //Player_1
        if touch.x <= 199 {
            //if racket not completly inside the field position at edge
            if touch.y <= 0 + RACKET_HEIGHT {
                rackets[0].set_ypos_centre(0 + RACKET_HEIGHT);
            } else if touch.y >= 271 - RACKET_HEIGHT {
                rackets[0].set_ypos_centre(271 - RACKET_HEIGHT);
            }
            //if racket completly inside the field (if touch.y > 0 + RACKET_HEIGHT && touch.x < 271 - RACKET_HEIGHT)
            else {
                //set new racket centre point (y)
                rackets[0].set_ypos_centre(touch.y);
            }
        }
        //Player_2
        if touch.x >= 280 {
            //if racket not completly inside the field position at edge
            if touch.y <= 0 + RACKET_HEIGHT {
                rackets[1].set_ypos_centre(0 + RACKET_HEIGHT);
            } else if touch.y >= 271 - RACKET_HEIGHT {
                rackets[1].set_ypos_centre(271 - RACKET_HEIGHT);
            }
            //if racket completly inside the field (if touch.y > 0 + RACKET_HEIGHT && touch.x < 271 - RACKET_HEIGHT)
            else {
                //set new racket centre point (y)
                rackets[1].set_ypos_centre(touch.y);
            }
        }
    }};}

