use lcd;
use lcd::Framebuffer;
use lcd::FramebufferL8;

pub fn draw_rectangle(
        & self,
        layer: &mut lcd::FramebufferL8,
        x_left: u16,
        x_right: u16,
        y_top: u16,
        y_bottom: u16,
        colour: lcd::Color,
    ) {
        for y in y_top..=y_bottom {
            for x in x_left..=x_right {
                layer.set_pixel(x as usize, y as usize, colour);
            }
        }
    }
pub fn draw_circle();