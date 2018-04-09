#![feature(const_fn)]

use lcd;
use lcd::Framebuffer;
use lcd::FramebufferL8;

  //general Racket Properties
    const RACKET_WIDTH :u16= 10;
    const RACKET_HEIGHT : u16=30;
    const RACKET_COLOR : lcd::Color=lcd::Color::rgb(150, 150, 30);

    
    //Racket Starting Positions
    let xpos_centre_p1 = 0 + 5 + RACKET_WIDTH;
    let xpos_centre_p2 = 479 - 5 - RACKET_WIDTH;
    let ypos_centre = 135;
    //draw racket(s) in starting position
    for racket in rackets.iter_mut() {
        racket.draw_rectangle(
            framebuffer,
            racket.get_xpos_centre() - RACKET_WIDTH,
            racket.get_xpos_centre() + RACKET_WIDTH,
            racket.get_ypos_centre() - RACKET_HEIGHT,
            racket.get_ypos_centre() + RACKET_HEIGHT,
            RACKET_COLOR,
        );
    }


//Racket Positions
pub struct Racket {
    xpos_centre: u16,
    ypos_centre: u16,
    ypos_centre_old: u16,
}
impl Racket {
    //Create new Racket
    pub fn new(player_id: u16) -> Racket {
        if player_id=0{
        Racket {
            xpos_centre: RACKET_WIDTH,
            ypos_centre: 135,      ypos_centre_old: 135,
        }}
        else if player_id=1{
        Racket {
            xpos_centre: 479-RACKET_WIDTH,
            ypos_centre: 135,      ypos_centre_old: 135,
        }}
    }
    //set Centre Point Coordinates
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
    
    
    pub fn draw_racket(&self,buffer: &mut lcd::FramebufferL8,)
    {geometry::draw_rectangle(
            buffer,
            self.xpos_centre-RACKET_WIDTH,
            self.xpos_centre+RACKET_WIDTH,
            self.ypos_centre-RACKET_HEIGHT,
            self.ypos_centre+RACKET_HEIGHT,
            RACKET_COLOR,
        );

    }
    pub fn move_racket(
        & self,
        buffer: &mut lcd::FramebufferL8,
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
        geometry::draw_rectangle(
            buffer,
            x_left,
            x_right,
            y_top_erase,
            y_bottom_erase,
            bgcolor,
        );
        //draw new racket
        geometry::draw_rectangle(
            buffer,
            x_left,
            x_right,
            y_top_draw,
            y_bottom_draw,
            racket_color,
        );
    }
}


