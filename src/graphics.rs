use fps;
use lcd;
use lcd::Framebuffer;

pub fn draw_rectangle(
    buffer: &mut lcd::FramebufferL8,
    x_left: u16,
    x_right: u16,
    y_top: u16,
    y_bottom: u16,
    color: lcd::Color,
) {
    for y in y_top..=y_bottom {
        for x in x_left..=x_right {
            buffer.set_pixel(x as usize, y as usize, color);
        }
    }
}


pub fn draw_circle(buffer: &mut lcd::FramebufferL8, x_pos_centre: u16, y_pos_centre: u16, radius: u16, color:lcd::Color) {

    for y in y_pos_centre-radius..=y_pos_centre+radius {
        for x in x_pos_centre-radius..=x_pos_centre+radius {
            if (x-x_pos_centre) * (x-x_pos_centre) + (y-y_pos_centre) * (y-y_pos_centre) <= radius * radius {
                buffer.set_pixel(x as usize, y as usize, color);
            }
        }
    }
}

pub fn draw_fps(layer1: &mut lcd::FramebufferL8, fps: &fps::FpsCounter) {
    let mut number = fps.last_fps;
    if number > 99 {
        number = 99;
    }
    draw_number(layer1, 0, 0, number / 10);
    draw_number(layer1, 5, 0, number % 10);
}
fn draw_number(layer1: &mut lcd::FramebufferL8, x: usize, y: usize, number: usize) {
    if number == 0 {
        draw_seven_segment(layer1, x, y, true, true, true, false, true, true, true);
    } else if number == 1 {
        draw_seven_segment(layer1, x, y, false, false, true, false, false, true, false);
    } else if number == 2 {
        draw_seven_segment(layer1, x, y, true, false, true, true, true, false, true);
    } else if number == 3 {
        draw_seven_segment(layer1, x, y, true, false, true, true, false, true, true);
    } else if number == 4 {
        draw_seven_segment(layer1, x, y, false, true, true, true, false, true, false);
    } else if number == 5 {
        draw_seven_segment(layer1, x, y, true, true, false, true, false, true, true);
    } else if number == 6 {
        draw_seven_segment(layer1, x, y, true, true, false, true, true, true, true);
    } else if number == 7 {
        draw_seven_segment(layer1, x, y, true, false, true, false, false, true, false);
    } else if number == 8 {
        draw_seven_segment(layer1, x, y, true, true, true, true, true, true, true);
    } else if number == 9 {
        draw_seven_segment(layer1, x, y, true, true, true, true, false, true, true);
    }
}
fn draw_seven_segment(
    layer1: &mut lcd::FramebufferL8,
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
    layer1.set_pixel_direct(x + 0, y + 0, if top { white } else { black });
    layer1.set_pixel_direct(x + 1, y + 0, if top { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 0, if top { white } else { black });
    layer1.set_pixel_direct(x + 0, y + 1, if top_left { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 1, if top_right { white } else { black });
    layer1.set_pixel_direct(x + 0, y + 2, if top_left { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 2, if top_right { white } else { black });
    layer1.set_pixel_direct(x + 0, y + 3, if center { white } else { black });
    layer1.set_pixel_direct(x + 1, y + 3, if center { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 3, if center { white } else { black });
    layer1.set_pixel_direct(x + 0, y + 4, if bottom_left { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 4, if bottom_right { white } else { black });
    layer1.set_pixel_direct(x + 0, y + 5, if bottom_left { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 5, if bottom_right { white } else { black });
    layer1.set_pixel_direct(x + 0, y + 6, if bottom { white } else { black });
    layer1.set_pixel_direct(x + 1, y + 6, if bottom { white } else { black });
    layer1.set_pixel_direct(x + 2, y + 6, if bottom { white } else { black });
}

pub fn quad(x: usize, y: usize, size: usize, color: &u8, layer1: &mut lcd::FramebufferL8) {
    for y in y..y + size {
        for x in x..x + size {
            layer1.set_pixel_direct(x, y, *color);
        }
    }
}
fn update_graphics(buffer: &mut lcd::FramebufferL8, gamestate: &GamestatePacket,rackets:&mut [racket::Racket;2]) {
    // TODO: implement
    // send gamestate to racket to let racket move
    for id in 0..2{
        rackets[id].update_racket_position(gamestate.get_racket_ypos(id));
    }
    //TODO same for ball
    
}
