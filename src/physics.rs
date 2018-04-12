use ball::BALL_RADIUS;
use network;
use racket::RACKET_HEIGHT;
use racket::RACKET_WIDTH;
use stm32f7::system_clock::ticks;

const RACKET_SPEED: i16 = 8;
const BALL_MAX_SPEED: i16 = 15;
pub fn calculate_physics(
    server_gamestate: &mut network::GamestatePacket,
    inputs: [network::InputPacket; 2],
) {
    let racket_width = RACKET_WIDTH as i16;
    let racket_height = RACKET_HEIGHT as i16;
    let ball_radius = BALL_RADIUS as i16;

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
    }
    // TODO Ball Position
    let ball: &mut network::BallPacket = &mut server_gamestate.ball;
    let mut touches_racket_1_face: bool = false;
    let mut touches_racket_2_face: bool = false;
    let mut random_x_vel;
    let mut random_y_vel;


    // TMP::
    if ball.x > 300 {
        server_gamestate.score[0] = 1;
        ball.x = 300;
    }
    // ::TMP

    // if ball touches top or bottom wall invert y_vel
    if ball.y <= ball_radius || ball.y >= 271 - ball_radius {
        ball.x_vel *= -1;
    }

    // ball touches face of left racket
    if ball.x <= ball_radius + server_gamestate.rackets[0].x + racket_width
        && ball.y >= server_gamestate.rackets[0].y - racket_height
        && ball.y <= server_gamestate.rackets[0].y + racket_height
    {
        touches_racket_1_face = true;
    }
    // ball touches face of right racket
    if ball.x >= 479 - ball_radius - server_gamestate.rackets[1].x - racket_width
        && ball.y >= server_gamestate.rackets[1].y - racket_height
        && ball.y <= server_gamestate.rackets[1].y + racket_height
    {
        touches_racket_2_face = true;
    }
    // if ball touches racket face invert x_vel
    if touches_racket_1_face || touches_racket_2_face {
        ball.y_vel *= -1;
    }
    // if ball touches racket corner calculate new direction
    // if ball touches goal increase score and reset ball position
    if ball.x <= ball_radius || ball.x >= 479 {
        while {
            random_x_vel = ticks() as i16;
            (random_x_vel > BALL_MAX_SPEED) || (random_x_vel < -BALL_MAX_SPEED)
        } {}
        while {
            random_y_vel = ticks() as i16;
            (random_y_vel > BALL_MAX_SPEED) || (random_y_vel < -BALL_MAX_SPEED)
        } {}
        if ball.x <= ball_radius {
            server_gamestate.score[0] += 1;
        }
        if ball.x >= 479 {
            server_gamestate.score[1] += 1;
        }
        ball.x_vel = random_x_vel;
        ball.y_vel = random_y_vel;
    }
    // new position=old position+velocity
    ball.x += ball.x_vel;
    ball.y += ball.y_vel;
}
