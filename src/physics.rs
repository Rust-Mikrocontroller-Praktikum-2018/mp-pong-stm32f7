mod network;
mod racket;

pub fn calculate_physics(server_gamestate:network::GamestatePacket, inputs:[network::InputPacket; 2]){
    //Racket Positions
    //for each player check whether to move up, down or not at all
    for i in 0..2{
        //input_direction: -1->up, 0->no movement, 1->down
        let input_direction=inputs[i].says_move_up()
        if input_direction!0{
            rackets[i].update_racket_position(input_direction)

        }
    }
    //TODO Ball Position
}


