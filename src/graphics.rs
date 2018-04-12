use ball;
use fps;
use lcd;
use lcd::Framebuffer;
use lcd::TextWriter;
use network;
use racket;
use PADDING;

const SCORE_1_X: usize = 480 / 2 - 10 - 15;
const SCORE_1_Y: usize = 272 - 50;
const SCORE_2_X: usize = 480 / 2 + 10;
const SCORE_2_Y: usize = 272 - 50;
const SCORE_REDRAW_TIME: usize = 800;

pub fn draw_rectangle(
    buffer: &mut lcd::Framebuffer,
    x_left: u16,
    x_right: u16,
    y_top: u16,
    y_bottom: u16,
    color: u8,
) {
    for y in y_top..=y_bottom {
        for x in x_left..=x_right {
            buffer.set_pixel(x as usize, y as usize, color);
        }
    }
}

pub fn draw_circle(
    buffer: &mut lcd::Framebuffer,
    x_pos_centre: u32,
    y_pos_centre: u32,
    radius: u32,
    color: u8,
) {
    for y in y_pos_centre - radius..=y_pos_centre + radius {
        for x in x_pos_centre - radius..=x_pos_centre + radius {
            if x * x + x_pos_centre * x_pos_centre - 2 * x * x_pos_centre + y * y
                + y_pos_centre * y_pos_centre - 2 * y * y_pos_centre
                <= radius * radius
            {
                buffer.set_pixel(x as usize, y as usize, color);
            }
        }
    }
}
pub fn draw_partial_circle(
    buffer: &mut lcd::FramebufferL8,
    x_pos_centre: u32,
    y_pos_centre: u32,
    x_pos_centre_part: u32,
    y_pos_centre_part: u32,
    radius: u32,
    radius_part: u32,
    color: u8,
) {
    // all coordinates of square around circle
    for y in y_pos_centre - radius..=y_pos_centre + radius {
        for x in x_pos_centre - radius..=x_pos_centre + radius {
            // if coordinates not inside square around overlapping circle
            if (x < x_pos_centre_part - radius_part || x > x_pos_centre + radius_part)
                && (y < y_pos_centre_part - radius_part || y > y_pos_centre_part + radius_part)
            {
                // if coordinates fulfil circle equation
                if x * x + x_pos_centre * x_pos_centre - 2 * x * x_pos_centre + y * y
                    + y_pos_centre * y_pos_centre - 2 * y * y_pos_centre
                    <= radius * radius
                {
                    buffer.set_pixel(x as usize, y as usize, color);
                }
            }
            // if coordinates inside square around overlapping circle, check for each pixel individually
            else if (x * x + x_pos_centre_part * x_pos_centre_part - 2 * x * x_pos_centre_part
                + y * y + y_pos_centre_part * y_pos_centre_part
                - 2 * y * y_pos_centre_part > radius_part * radius_part)
                && (x * x + x_pos_centre * x_pos_centre - 2 * x * x_pos_centre + y * y
                    + y_pos_centre * y_pos_centre - 2 * y * y_pos_centre
                    <= radius * radius)
            {
                buffer.set_pixel(x as usize, y as usize, color);
            }
        }
    }
}

pub fn draw_fps(framebuffer: &mut lcd::FramebufferL8, fps: &fps::FpsCounter) {
    let mut number = fps.last_fps;
    if number > 99 {
        number = 99;
    }
    draw_number(framebuffer, 0, 0, number / 10);
    draw_number(framebuffer, 5, 0, number % 10);
}
fn draw_number(framebuffer: &mut lcd::FramebufferL8, x: usize, y: usize, number: usize) {
    if number == 0 {
        draw_seven_segment(framebuffer, x, y, true, true, true, false, true, true, true);
    } else if number == 1 {
        draw_seven_segment(
            framebuffer,
            x,
            y,
            false,
            false,
            true,
            false,
            false,
            true,
            false,
        );
    } else if number == 2 {
        draw_seven_segment(
            framebuffer,
            x,
            y,
            true,
            false,
            true,
            true,
            true,
            false,
            true,
        );
    } else if number == 3 {
        draw_seven_segment(
            framebuffer,
            x,
            y,
            true,
            false,
            true,
            true,
            false,
            true,
            true,
        );
    } else if number == 4 {
        draw_seven_segment(
            framebuffer,
            x,
            y,
            false,
            true,
            true,
            true,
            false,
            true,
            false,
        );
    } else if number == 5 {
        draw_seven_segment(
            framebuffer,
            x,
            y,
            true,
            true,
            false,
            true,
            false,
            true,
            true,
        );
    } else if number == 6 {
        draw_seven_segment(framebuffer, x, y, true, true, false, true, true, true, true);
    } else if number == 7 {
        draw_seven_segment(
            framebuffer,
            x,
            y,
            true,
            false,
            true,
            false,
            false,
            true,
            false,
        );
    } else if number == 8 {
        draw_seven_segment(framebuffer, x, y, true, true, true, true, true, true, true);
    } else if number == 9 {
        draw_seven_segment(framebuffer, x, y, true, true, true, true, false, true, true);
    }
}
fn draw_seven_segment(
    framebuffer: &mut lcd::FramebufferL8,
    x: usize,
    y: usize,
    top: bool,
    top_left: bool,
    top_right: bool,
    center: bool,
    bottom_left: bool,
    bottom_right: bool,
    bottom: bool,
) {
    let black = 0;
    let white = 255;
    framebuffer.set_pixel(x + 0, y + 0, if top { white } else { black });
    framebuffer.set_pixel(x + 1, y + 0, if top { white } else { black });
    framebuffer.set_pixel(x + 2, y + 0, if top { white } else { black });
    framebuffer.set_pixel(x + 0, y + 1, if top_left { white } else { black });
    framebuffer.set_pixel(x + 2, y + 1, if top_right { white } else { black });
    framebuffer.set_pixel(x + 0, y + 2, if top_left { white } else { black });
    framebuffer.set_pixel(x + 2, y + 2, if top_right { white } else { black });
    framebuffer.set_pixel(x + 0, y + 3, if center { white } else { black });
    framebuffer.set_pixel(x + 1, y + 3, if center { white } else { black });
    framebuffer.set_pixel(x + 2, y + 3, if center { white } else { black });
    framebuffer.set_pixel(x + 0, y + 4, if bottom_left { white } else { black });
    framebuffer.set_pixel(x + 2, y + 4, if bottom_right { white } else { black });
    framebuffer.set_pixel(x + 0, y + 5, if bottom_left { white } else { black });
    framebuffer.set_pixel(x + 2, y + 5, if bottom_right { white } else { black });
    framebuffer.set_pixel(x + 0, y + 6, if bottom { white } else { black });
    framebuffer.set_pixel(x + 1, y + 6, if bottom { white } else { black });
    framebuffer.set_pixel(x + 2, y + 6, if bottom { white } else { black });
}

pub fn quad(x: usize, y: usize, size: usize, color: &u8, framebuffer: &mut lcd::FramebufferL8) {
    for y in y..y + size {
        for x in x..x + size {
            framebuffer.set_pixel(x, y, *color);
        }
    }
}

pub fn draw_initial(
    framebuffer: &mut lcd::FramebufferL8,
    rackets: &[racket::Racket; 2],
    ball: &ball::Ball,
) {
    // Draw Racket Start Position
    for racket in rackets.iter() {
        racket.draw_racket(framebuffer);
    }
    ball.draw_ball(framebuffer);
}
pub fn update_graphics(
    framebuffer: &mut lcd::FramebufferL8,
    gamestate: &network::GamestatePacket,
    rackets: &mut [racket::Racket; 2],
    ball: &mut ball::Ball,
    menu_font: &mut TextWriter,
    cache: &mut GraphicsCache,
    total_time: usize,
    _delta_time: usize,
) {

    if gamestate.state != 0 {
        if cache.last_state != gamestate.state {
            if gamestate.state == 254 {
                menu_font.write_at(framebuffer, "Player 1 wins", PADDING, PADDING);
            } else if gamestate.state == 255 {
                menu_font.write_at(framebuffer, "Player 2 wins", PADDING, PADDING);
            }
            cache.last_state = gamestate.state;
        }
    } else if cache.last_state != 0 {
        cache.last_state = 0;
        framebuffer.clear();
        *cache = GraphicsCache::new();
        draw_initial(framebuffer, rackets, ball);
        // TODO: redraw
    }

    //send gamestate to ball
    ball.update_ball_pos(framebuffer, gamestate.ball);
    // send gamestate to racket to let racket move
    for id in 0..2 {
        rackets[id].update_racket_pos(framebuffer, gamestate.rackets[id].y as u16);
    }
    
    let redraw_score_1 =
        gamestate.score[0] != cache.score[0] || total_time > cache.last_score_redraw + SCORE_REDRAW_TIME;
    let redraw_score_2 =
        gamestate.score[1] != cache.score[1] || total_time > cache.last_score_redraw + SCORE_REDRAW_TIME;

    if redraw_score_1 || redraw_score_2 {
        cache.last_score_redraw = total_time;
    }

    if redraw_score_1 {
        if gamestate.score[0] == 0 && cache.score[0] != 0 {
            draw_fix_for_score_0(framebuffer, SCORE_1_X, SCORE_1_Y);
        }
        if gamestate.score[0] == 1 && cache.score[0] != 1 {
            draw_fix_for_score_1(framebuffer, SCORE_1_X, SCORE_1_Y);
        }

        cache.score[0] = gamestate.score[0];
        menu_font.write_at(
            framebuffer,
            &format!("{}", gamestate.score[0]),
            SCORE_1_X,
            SCORE_1_Y,
        );
    }
    if redraw_score_2 {
        if gamestate.score[1] == 0 && cache.score[1] != 0 {
            draw_fix_for_score_0(framebuffer, SCORE_2_X, SCORE_2_Y);
        }
        if gamestate.score[1] == 1 && cache.score[1] != 1 {
            draw_fix_for_score_1(framebuffer, SCORE_2_X, SCORE_2_Y);
        }

        cache.score[1] = gamestate.score[1];
        menu_font.write_at(
            framebuffer,
            &format!("{}", gamestate.score[1]),
            SCORE_2_X,
            SCORE_2_Y,
        );
    }
}

fn draw_fix_for_score_0(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let x = x as u16;
    let y = y as u16;
    draw_rectangle(framebuffer, x, x + 1, y + 10, y + 30, 0);
}
fn draw_fix_for_score_1(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let x = x as u16;
    let y = y as u16;
    draw_rectangle(framebuffer, x + 10, x + 16, y + 10, y + 30, 0);
}

pub fn draw_guidelines(framebuffer: &mut Framebuffer) {
    // center guidelines
    for y in 0..272 {
        framebuffer.set_pixel(480 / 4, y, 64);
        framebuffer.set_pixel(480 / 2, y, 128);
        framebuffer.set_pixel(480 / 4 * 3, y, 64);
    }
    for x in 0..480 {
        framebuffer.set_pixel(x, 272 / 2, 128);
    }
}

pub struct GraphicsCache {
    score: [u8; 2],
    last_score_redraw: usize,
    last_state: u8,
}

impl GraphicsCache {
    pub fn new() -> GraphicsCache {
        GraphicsCache {
            score: [99, 99],
            last_score_redraw: 0,
            last_state: 0,
        }
    }
}
