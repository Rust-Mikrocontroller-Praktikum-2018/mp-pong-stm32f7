pub fn update_racket_pos(&self, &mut buffer, ){
    for racket in rackets.iter_mut() {
        //check if position changed
        if racket.get_ypos_centre() != racket.get_ypos_centre_old() {
            //if racket moved down
            if racket.get_ypos_centre() > racket.get_ypos_centre_old() {
                racket.move_racket(
                    framebuffer,
                    racket.get_xpos_centre() - RACKET_WIDTH,
                    racket.get_xpos_centre() + RACKET_WIDTH,
                    racket.get_ypos_centre_old() - RACKET_HEIGHT,
                    min(
                        racket.get_ypos_centre() - RACKET_HEIGHT - 1,
                        racket.get_ypos_centre_old() + RACKET_HEIGHT,
                    ),
                    max(
                        racket.get_ypos_centre_old() + RACKET_HEIGHT,
                        racket.get_ypos_centre() - RACKET_HEIGHT,
                    ),
                    racket.get_ypos_centre() + RACKET_HEIGHT,
                    BGCOLOR,
                    RACKET_COLOR,
                );
            }
            //if racket moved up
            if racket.get_ypos_centre() < racket.get_ypos_centre_old() {
                //TODO CREATE FN MOVE RACKET
                racket.move_racket(
                    framebuffer,
                    racket.get_xpos_centre() - RACKET_WIDTH,
                    racket.get_xpos_centre() + RACKET_WIDTH,
                    max(
                        racket.get_ypos_centre() + RACKET_HEIGHT + 1,
                        racket.get_ypos_centre_old() - RACKET_HEIGHT,
                    ),
                    racket.get_ypos_centre_old() + RACKET_HEIGHT,
                    racket.get_ypos_centre() - RACKET_HEIGHT,
                    min(
                        racket.get_ypos_centre_old() - RACKET_HEIGHT,
                        racket.get_ypos_centre() + RACKET_HEIGHT,
                    ),
                    BGCOLOR,
                    RACKET_COLOR,
                );
            }
            //remember old racket points (y)
            let mut ypos_centre_old = racket.get_ypos_centre();
            racket.set_ypos_centre_old(ypos_centre_old);
        }
    }}