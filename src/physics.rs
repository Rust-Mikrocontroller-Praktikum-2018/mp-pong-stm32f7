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
    let x_pos_new = ball.x + ball.x_vel;
    let y_pos_new = ball.y + ball.y_vel;

    let mut touches_racket_face: bool = false;
    let mut touches_racket_side: bool = false;

    if server_gamestate.state != 0 {
        return;
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
        let rect_ball = Rectangle::new(x_pos_new, y_pos_new, ball_radius, 0);
        let rect_racket = Rectangle::new(
            server_gamestate.rackets[i].x,
            server_gamestate.rackets[i].y,
            racket_width,
            racket_height,
        );
        //ball touches face of left racket

        if overlap_test(rect_ball, rect_racket) {
            if ball.y+ball_radius<server_gamestate.rackets[i].y-racket_height||ball.y-ball_radius>server_gamestate.rackets[i].y+racket_height{touches_racket_side=true;}
            else{touches_racket_face = true;}

        }
    }
    //move Ball
    //if ball touches top or bottom wall orr racket side
    if y_pos_new < ball_radius || y_pos_new > 271 - ball_radius||touches_racket_side {
        ball.y_vel *= -1;
    } else {
        //new position=old position+velocity
        ball.y += ball.y_vel;
    }
    //check for goals
    if x_pos_new <= ball_radius || x_pos_new >= 479 - ball_radius {
        // if ball touches goal increase score and reset ball position
        if x_pos_new <= ball_radius {
            server_gamestate.score[1] += 1;
            if server_gamestate.score[1] >= 9 {
                server_gamestate.state = 254;
            }
        }
        if x_pos_new >= 479 - ball_radius {
            server_gamestate.score[0] += 1;
            if server_gamestate.score[0] >= 9 {
                server_gamestate.state = 255;
            }
        }
        ball.reset();
    }// if ball touches racket face invert x_vel
    else if touches_racket_face {
        ball.x_vel *= -1;
    }
    //new position=old position+velocity
    ball.x += ball.x_vel;
}
fn overlap_test(rectangle1: Rectangle, rectangle2: Rectangle) -> bool {
    !(rectangle2.right < rectangle1.left || rectangle2.left > rectangle1.right
        || rectangle2.top > rectangle1.bottom || rectangle2.bottom < rectangle1.top)
}

struct Rectangle {
    left: i16,
    right: i16,
    top: i16,
    bottom: i16,
}
impl Rectangle {
    pub fn new(centre_x: i16, centre_y: i16, width: i16, height: i16) -> Rectangle {
        Rectangle {
            left: centre_x - width,
            right: centre_x + width,
            top: centre_y - height,
            bottom: centre_y + height,
        }
    }
}