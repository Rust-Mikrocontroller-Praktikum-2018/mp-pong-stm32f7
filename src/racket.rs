use BGCOLOR;
use core::cmp::max;
use core::cmp::min;
use graphics;
use lcd;

// general Racket Properties
pub const RACKET_WIDTH: u16 = 10;
pub const RACKET_HEIGHT: u16 = 30;
const RACKET_COLOR: u8 = 150;

// Racket Positions
pub struct Racket {
    xpos_centre: u16,
    ypos_centre: u16,
    ypos_centre_old: u16,
}
impl Racket {
    // Create new Racket

    pub fn new(player_id: u8) -> Racket {
        if player_id == 0 {
            Racket {
                xpos_centre: RACKET_WIDTH,
                ypos_centre: 135,
                ypos_centre_old: 135,
            }
        } else {
            Racket {
                xpos_centre: 479 - RACKET_WIDTH,
                ypos_centre: 135,
                ypos_centre_old: 135,
            }
        }
    }
    // set Centre Point Coordinates
    pub fn set_ypos_centre(&mut self, ypos_centre_set: u16) {
        self.ypos_centre = ypos_centre_set;
    }
    pub fn set_ypos_centre_old(&mut self, ypos_centre_set: u16) {
        self.ypos_centre_old = ypos_centre_set;
    }

    // get Centre Point Coordinates
    pub fn get_xpos_centre(&self) -> u16 {
        self.xpos_centre
    }
    pub fn get_ypos_centre(&self) -> u16 {
        self.ypos_centre
    }
    pub fn get_ypos_centre_old(&self) -> u16 {
        self.ypos_centre_old
    }

    pub fn draw_racket(&self, buffer: &mut lcd::FramebufferL8) {
        graphics::draw_rectangle(
            buffer,
            self.xpos_centre - RACKET_WIDTH,
            self.xpos_centre + RACKET_WIDTH,
            self.ypos_centre - RACKET_HEIGHT,
            self.ypos_centre + RACKET_HEIGHT,
            RACKET_COLOR,
        );
    }

    pub fn draw_moved_racket(
        &self,
        buffer: &mut lcd::FramebufferL8,
        y_top_erase: u16,
        y_bottom_erase: u16,
        y_top_draw: u16,
        y_bottom_draw: u16,
    ) {
        // erase old racket
        graphics::draw_rectangle(
            buffer,
            self.xpos_centre - RACKET_WIDTH,
            self.xpos_centre + RACKET_WIDTH,
            y_top_erase,
            y_bottom_erase,
            BGCOLOR,
        );
        // draw new racket
        graphics::draw_rectangle(
            buffer,
            self.xpos_centre - RACKET_WIDTH,
            self.xpos_centre + RACKET_WIDTH,
            y_top_draw,
            y_bottom_draw,
            RACKET_COLOR,
        );
    }
    // TODO Update self from Server Gamestate
    // pub fn update_racket_pos(&self, gamestate){
    //
    // remember old position
    // self.ypos_centre_old = self.ypos_centre;
    // TODO
    // update self position
    // self.ypos_centre =gamestate[1]}
    //
    pub fn update_racket_pos(
        &mut self,
        framebuffer: &mut lcd::FramebufferL8,
        new_ypos_centre: u16,
    ) {
        // Remember old state
        self.ypos_centre_old = self.ypos_centre;
        // Copy Position from Gamestate to self
        self.ypos_centre = new_ypos_centre;
        // draw Racket in new Position
        // if racket moved down
        if self.ypos_centre > self.ypos_centre_old {
            self.draw_moved_racket(
                framebuffer,
                self.get_ypos_centre_old() - RACKET_HEIGHT,
                min(
                    self.get_ypos_centre() - RACKET_HEIGHT - 1,
                    self.get_ypos_centre_old() + RACKET_HEIGHT,
                ),
                max(
                    self.get_ypos_centre_old() + RACKET_HEIGHT,
                    self.get_ypos_centre() - RACKET_HEIGHT,
                ),
                self.get_ypos_centre() + RACKET_HEIGHT,
            );
        }
        // if racket moved up
        if self.get_ypos_centre() < self.get_ypos_centre_old() {
            // TODO CREATE FN MOVE RACKET
            self.draw_moved_racket(
                framebuffer,
                max(
                    self.get_ypos_centre() + RACKET_HEIGHT + 1,
                    self.get_ypos_centre_old() - RACKET_HEIGHT,
                ),
                self.get_ypos_centre_old() + RACKET_HEIGHT,
                self.get_ypos_centre() - RACKET_HEIGHT,
                min(
                    self.get_ypos_centre_old() - RACKET_HEIGHT,
                    self.get_ypos_centre() + RACKET_HEIGHT,
                ),
            );
        }
    }
}
