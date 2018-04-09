use lcd;
use lcd::Framebuffer;

pub fn draw_rectangle(
    buffer: &mut lcd::FramebufferL8,
    x_left: u16,
    x_right: u16,
    y_top: u16,
    y_bottom: u16,
    colour: lcd::Color,
) {
    for y in y_top..=y_bottom {
        for x in x_left..=x_right {
            buffer.set_pixel(x as usize, y as usize, colour);
        }
    }
}
pub fn draw_circle(buffer: &mut lcd::FramebufferL8) {}
