use ball::BALL_RADIUS;
use lcd::HEIGHT;
use lcd::WIDTH;
use network;
use racket::RACKET_HEIGHT;
use racket::RACKET_WIDTH;
use network::packets::STATE_RUNNING;
use network::packets::STATE_WON_PLAYER_1;
use network::packets::STATE_WON_PLAYER_2;
use network::BallPacket;


const RACKET_SPEED: i16 = 8;

pub fn calculate_physics(
    server_gamestate: &mut network::GamestatePacket,
    inputs: [network::InputPacket; 2],
    total_time: usize,
) {
    let racket_width = RACKET_WIDTH as i16;
    let racket_height = RACKET_HEIGHT as i16;
    let ball_radius = BALL_RADIUS as i16;
    let height = HEIGHT as i16;
    let width = WIDTH as i16;
    let ball: &mut network::BallPacket = &mut server_gamestate.ball;
    let x_pos_new = ball.x + ball.x_vel;
    let y_pos_new = ball.y + ball.y_vel;

    let mut touches_racket_face: bool = false;

    let mut touches_racket_upper_side: bool = false;
    let mut touches_racket_under_side: bool = false;
    let mut in_goal: bool = false;

    if server_gamestate.state != STATE_RUNNING {
        return;
    }

    // Racket Positions
    // for each player check whether to move up, down or not at all
    for i in 0..2 {
        let mut racket_pos = server_gamestate.rackets[i].y;
        let racket_pos_old = racket_pos;
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
            } else if racket_pos > height - 1 - racket_height {
                racket_pos = height - 1 - racket_height;
            }
            server_gamestate.rackets[i].y = racket_pos;
        }
        // Ball touches racket
        let rect_ball = Rectangle::new(x_pos_new, y_pos_new, ball_radius, 0);
        let rect_racket = Rectangle::new(
            server_gamestate.rackets[i].x,
            server_gamestate.rackets[i].y,
            racket_width,
            racket_height,
        );

        if overlap_test(rect_ball, rect_racket) {
            // ball touches side of racket
            if ball.y + ball_radius < racket_pos_old - racket_height {
                touches_racket_under_side = true;
            } else if ball.y - ball_radius > racket_pos_old + racket_height {
                touches_racket_upper_side = true;
            } else {
                // ball touches face of racket
                touches_racket_face = true;
            }
        }
    }
    // move Ball
    // if ball touches top or bottom wall
    if y_pos_new < ball_radius || y_pos_new > height - 1 - ball_radius {
        ball.y_vel *= -1;
    }
    // if ball touches racket side
    if touches_racket_upper_side {
        ball.y_vel = -abs(ball.y_vel);
    } else if touches_racket_under_side {
        ball.y_vel = abs(ball.y_vel);
    }
    // new position=old position+velocity
    ball.y += ball.y_vel;

    // check for goals
    if x_pos_new <= ball_radius || x_pos_new >= width - 1 - ball_radius {
        in_goal = true;
    }
    // if ball touches racket face invert x_vel
    else if touches_racket_face {
        if x_pos_new > width as i16 / 2 {
            ball.x_vel = -abs(ball.x_vel);
        } else {
            ball.x_vel = abs(ball.x_vel);
        }
    }
    // new position=old position+velocity
    ball.x += ball.x_vel;
    // if ball touches goal increase score and reset ball position
    if in_goal {
        if x_pos_new <= ball_radius {
            server_gamestate.score[1] += 1;
            if server_gamestate.score[1] >= 9 {
                server_gamestate.state = STATE_WON_PLAYER_2;
            }
        }
        if x_pos_new >= width - 1 - ball_radius {
            server_gamestate.score[0] += 1;
            if server_gamestate.score[0] >= 9 {
                server_gamestate.state = STATE_WON_PLAYER_1;
            }
        }
        ball.reset(total_time);
    }
}
fn overlap_test(rectangle1: Rectangle, rectangle2: Rectangle) -> bool {
    !(rectangle2.right < rectangle1.left || rectangle2.left > rectangle1.right
        || rectangle2.top > rectangle1.bottom || rectangle2.bottom < rectangle1.top)
}
fn abs(value: i16) -> i16 {
    if value < 0 {
        -value
    } else {
        value
    }
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
