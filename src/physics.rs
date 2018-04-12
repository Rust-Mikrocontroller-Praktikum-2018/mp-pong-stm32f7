use ball::BALL_RADIUS;
use network;
use racket::RACKET_HEIGHT;
use racket::RACKET_WIDTH;

const RACKET_SPEED: i16 = 8;

pub fn calculate_physics(
    server_gamestate: &mut network::GamestatePacket,
    inputs: [network::InputPacket; 2],
) {
    let racket_width = RACKET_WIDTH as i16;
    let racket_height = RACKET_HEIGHT as i16;
    let ball_radius = BALL_RADIUS as i16;
    let ball: &mut network::BallPacket = &mut server_gamestate.ball;
    let mut touches_racket_face: bool = false;
    
    //move Ball
    //new position=old position+velocity
    //check for borders
    if ball.x + ball.x_vel > racket_width && ball.x + ball.x_vel < 479 - racket_width {
        ball.x += ball.x_vel;
    }
    //else if{}
    if ball.y + ball.y_vel > 0 && ball.y + ball.y_vel < 271 {
        ball.y += ball.y_vel;
    }
    // Racket Positions
    // for each player check whether to move up, down or not at all
    for i in 0..2 {
        let mut racket_pos = server_gamestate.rackets[i].y;

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
            server_gamestate.rackets[i].y = racket_pos;
        }
        //Ball touches racket
        // ball touches face of left racket
        if ball.x <= ball_radius + server_gamestate.rackets[i].x + racket_width
            && ball.y >= server_gamestate.rackets[i].y - racket_height - ball_radius / 2
            && ball.y <= server_gamestate.rackets[i].y + racket_height + ball_radius / 2
        {
            touches_racket_face = true;
        }
    }
    // TODO Ball Position
 
    // if ball touches racket face invert x_vel
    if touches_racket_face{
        ball.x_vel *= -1;
    }
    
    // if ball touches top or bottom wall invert y_vel
    if ball.y <= ball_radius || ball.y >= 271 - ball_radius {
        ball.y_vel *= -1;
    }
    // if ball touches goal increase score and reset ball position
    if ball.x <= ball_radius || ball.x >= 479 {
        if ball.x <= ball_radius {
            server_gamestate.score[0] += 1;
        }
        if ball.x >= 479 {
            server_gamestate.score[1] += 1;
        }
        ball.reset();
    }
    
}
/*pub fn abs(value:i16)->i16{
    if value<0{-value}
    else {value}
}*/
