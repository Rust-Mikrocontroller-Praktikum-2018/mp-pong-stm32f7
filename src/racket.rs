#![feature(const_fn)]

use lcd;
use lcd::Framebuffer;
use lcd::FramebufferL8;

  //general Racket Properties
    const RACKET_WIDTH :u16= 10;
    const RACKET_HEIGHT : u16=30;
    const RACKET_COLOR : lcd::Color=lcd::Color::rgb(150, 150, 30);

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
    
    
    pub fn draw_racket_start_pos(&self,layer: &mut lcd::FramebufferL8,id:bool){self.draw_rectangle(
            layer,
            xpos_centre-RACKET_WIDTH,
            xpos_centre+RACKET_WIDTH,
            ypos_centre-RACKET_HEIGHT,
            ypos_centre+RACKET_HEIGHT,
            RACKET_COLOR,
        );

    }
    pub fn move_racket(
        & self,
        layer: &mut lcd::FramebufferL8,
        x_left: u16,
        x_right: u16,
        y_top_erase: u16,
        y_bottom_erase: u16,
        y_top_draw: u16,
        y_bottom_draw: u16,
        bgcolor: lcd::Color,
        racket_color: lcd::Color,
    ) {
        //erase old racket
        self.draw_rectangle(
            layer,
            x_left,
            x_right,
            y_top_erase,
            y_bottom_erase,
            bgcolor,
        );
        //draw new racket
        self.draw_rectangle(
            layer,
            x_left,
            x_right,
            y_top_draw,
            y_bottom_draw,
            racket_color,
        );
    }
}

pub fn update_racket_pos(&self, &mut buffer, ){
    for racket in rackets.iter_mut() {
        //check if position changed
        if racket.get_ypos_centre() != racket.get_ypos_centre_old() {
            //if racket moved down
            if racket.get_ypos_centre() > racket.get_ypos_centre_old() {
                racket.move_racket(
                    framebuffer,
                    racket.get_xpos_centre() - RACKET_WIDTH,
                    racket.get_xpos_centre() + RACKET_WIDTH,
                    racket.get_ypos_centre_old() - RACKET_HEIGHT,
                    min(
                        racket.get_ypos_centre() - RACKET_HEIGHT - 1,
                        racket.get_ypos_centre_old() + RACKET_HEIGHT,
                    ),
                    max(
                        racket.get_ypos_centre_old() + RACKET_HEIGHT,
                        racket.get_ypos_centre() - RACKET_HEIGHT,
                    ),
                    racket.get_ypos_centre() + RACKET_HEIGHT,
                    BGCOLOR,
                    RACKET_COLOR,
                );
            }
            //if racket moved up
            if racket.get_ypos_centre() < racket.get_ypos_centre_old() {
                //TODO CREATE FN MOVE RACKET
                racket.move_racket(
                    framebuffer,
                    racket.get_xpos_centre() - RACKET_WIDTH,
                    racket.get_xpos_centre() + RACKET_WIDTH,
                    max(
                        racket.get_ypos_centre() + RACKET_HEIGHT + 1,
                        racket.get_ypos_centre_old() - RACKET_HEIGHT,
                    ),
                    racket.get_ypos_centre_old() + RACKET_HEIGHT,
                    racket.get_ypos_centre() - RACKET_HEIGHT,
                    min(
                        racket.get_ypos_centre_old() - RACKET_HEIGHT,
                        racket.get_ypos_centre() + RACKET_HEIGHT,
                    ),
                    BGCOLOR,
                    RACKET_COLOR,
                );
            }
            //remember old racket points (y)
            let mut ypos_centre_old = racket.get_ypos_centre();
            racket.set_ypos_centre_old(ypos_centre_old);
        }
    }}