use network;
use racket;

pub fn calculate_physics(
    server_gamestate: &mut network::GamestatePacket,
    inputs: [network::InputPacket; 2],
) {
    //Racket Positions
    //for each player check whether to move up, down or not at all
    for i in 0..2 {
        //input_direction: -1->up, 0->no movement, 1->down
        let input_direction = inputs[i].says_move_up();
        let mut racket_pos = server_gamestate.get_racket_ypos(i);
        if input_direction != 0 {
            //update racket position
            racket_pos += (input_direction * 5) as i16;
            server_gamestate.set_racket_ypos(i, racket_pos);
        }
    }
    //TODO Ball Position
}
