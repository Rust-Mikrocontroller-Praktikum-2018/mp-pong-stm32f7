use lcd;
use lcd::Framebuffer;
use stm32f7::lcd::FontRenderer;

pub struct TextWriter<'a> {
    font_renderer: FontRenderer<'a>,
    x_pos: i32,
    y_pos: i32,
}

impl<'a> TextWriter<'a> {
    pub fn new(font_data: &[u8], font_height: f32) -> TextWriter {
        TextWriter {
            font_renderer: FontRenderer::new(font_data, font_height),
            x_pos: 0,
            y_pos: 0,
        }
    }
    pub fn write(&mut self, framebuffer: &mut Framebuffer, text: &str) {
        let &mut TextWriter {
            ref mut font_renderer,
            ref mut x_pos,
            ref mut y_pos,
            ..
        } = self;

        let font_height = font_renderer.font_height() as i32;
        font_renderer.render(text, |x, y, v| {
            let x = x as i32;
            let y = y as i32;
            if *x_pos + x >= lcd::WIDTH as i32 {
                *x_pos = -x;
                *y_pos += font_height;
            }
            if *y_pos + font_height >= lcd::HEIGHT as i32 {
                *y_pos = 0;
                // TODO: no place for text D:
            }
            let alpha = (v * 255.0 + 0.5) as u8;
            framebuffer.set_pixel((*x_pos + x) as usize, (*y_pos + y) as usize, alpha);
        });
    }
    pub fn write_at(&mut self, framebuffer: &mut Framebuffer, text: &str, x: usize, y: usize) {
        self.x_pos = x as i32;
        self.y_pos = y as i32;
        self.write(framebuffer, text);
    }
}
