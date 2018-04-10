use network;
use racket::RACKET_HEIGHT;

const RACKET_SPEED: i16 = 5;
pub fn calculate_physics(
    server_gamestate: &mut network::GamestatePacket,
    inputs: [network::InputPacket; 2],
) {
    let racket_height = RACKET_HEIGHT as i16;
    //Racket Positions
    //for each player check whether to move up, down or not at all
    for i in 0..2 {
        let mut racket_pos = server_gamestate.get_racket_ypos(i);

        let mut input_direction = inputs[i].goal_y - racket_pos;

        if input_direction != 0 {
            // update racket position
            if input_direction > RACKET_SPEED {
                input_direction = RACKET_SPEED;
            } else if input_direction < -RACKET_SPEED {
                input_direction = -RACKET_SPEED;
            }

            racket_pos += input_direction;
            // keep racket in bounds
            if racket_pos < racket_height {
                racket_pos = racket_height;
            } else if racket_pos > 271 - racket_height {
                racket_pos = 271 - racket_height;
            }
            server_gamestate.set_racket_ypos(i, racket_pos);
        }
    }
    //TODO Ball Position
}
