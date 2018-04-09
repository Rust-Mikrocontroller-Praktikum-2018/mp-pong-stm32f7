use stm32f7::lcd;
//Racket Positions
pub struct Racket {
    xpos_centre: u16,
    ypos_centre: u16,
    ypos_centre_old: u16,
}
impl Racket {
    //Create new Racket
    pub fn new(xpos_centre: u16, ypos_centre: u16, ypos_centre_old: u16) -> Racket {
        Racket {
            xpos_centre: xpos_centre,
            ypos_centre: ypos_centre,
            ypos_centre_old: ypos_centre_old,
        }
    }
    //set Centre Point Coordinates
    /*pub fn set_xpos_centre(&mut self, xpos_centre_set: u16) {
        self.xpos_centre = xpos_centre_set;
    }*/
    pub fn set_ypos_centre(&mut self, ypos_centre_set: u16) {
        self.ypos_centre = ypos_centre_set;
    }
    pub fn set_ypos_centre_old(&mut self, ypos_centre_set: u16) {
        self.ypos_centre_old = ypos_centre_set;
    }
    //get Centre Point Coordinates
    pub fn get_xpos_centre(& self) -> u16 {
        self.xpos_centre
    }
    pub fn get_ypos_centre(& self) -> u16 {
        self.ypos_centre
    }
    pub fn get_ypos_centre_old(& self) -> u16 {
        self.ypos_centre_old
    }
    //pub fn draw_racket()
    pub fn draw_rectangle(
        & self,
        layer: &mut lcd::Layer<lcd::FramebufferArgb8888>,
        x_left: u16,
        x_right: u16,
        y_top: u16,
        y_bottom: u16,
        colour: lcd::Color,
    ) {
        for y in y_top..=y_bottom {
            for x in x_left..=x_right {
                layer.print_point_color_at(x as usize, y as usize, colour);
            }
        }
    }
    pub fn move_racket(
        & self,
        layer: &mut lcd::Layer<lcd::FramebufferArgb8888>,
        x_left: u16,
        x_right: u16,
        y_top_erase: u16,
        y_bottom_erase: u16,
        y_top_draw: u16,
        y_bottom_draw: u16,
        bgcolour: lcd::Color,
        racket_colour: lcd::Color,
    ) {
        //erase old racket
        self.draw_rectangle(
            layer,
            x_left,
            x_right,
            y_top_erase,
            y_bottom_erase,
            bgcolour,
        );
        //draw new racket
        self.draw_rectangle(
            layer,
            x_left,
            x_right,
            y_top_draw,
            y_bottom_draw,
            racket_colour,
        );
    }
}
