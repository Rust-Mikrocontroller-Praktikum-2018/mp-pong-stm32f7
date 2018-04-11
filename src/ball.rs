use graphics;
use lcd;
use network;

use BGCOLOR;

const BALL_COLOR: u8 = 255;
const BALL_RADIUS:u16= 10;

pub struct Ball {
    xpos_centre: u16,
    ypos_centre: u16,
    xpos_centre_old:u16,
    ypos_centre_old:u16,
}

impl Ball {
    pub fn new() -> Ball {
        
            Ball {
                xpos_centre: 239,
                ypos_centre: 135,
                xpos_centre_old:239,
                ypos_centre_old: 135,
            }
        } 

    // set Centre Point Coordinates
    pub fn set_xpos_centre(&mut self, xpos_centre_set: u16) {
        self.xpos_centre = xpos_centre_set;
    }
    pub fn set_xpos_centre_old(&mut self, xpos_centre_set: u16) {
        self.xpos_centre_old = xpos_centre_set;
    }
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
        pub fn get_xpos_centre_old(&self) -> u16 {
        self.xpos_centre_old
    }
    pub fn get_ypos_centre(&self) -> u16 {
        self.ypos_centre
    }
    pub fn get_ypos_centre_old(&self) -> u16 {
        self.ypos_centre_old
    }

    pub fn draw_ball(&self, buffer: &mut lcd::FramebufferL8) {
        graphics::draw_circle(
            buffer,
            self.xpos_centre.into(),
            self.ypos_centre.into(),
            BALL_RADIUS.into(),
            BALL_COLOR,
        );
    }

    
    pub fn update_ball_pos(
        &mut self,
        framebuffer: &mut lcd::FramebufferL8,
        new_ball: network::BallPacket,
    )
    {
        // Remember old state
        self.xpos_centre_old=self.xpos_centre;
    self.ypos_centre_old = self.ypos_centre;
        // Copy Position from Gamestate to self
        self.xpos_centre= new_ball.x as u16;
        self.ypos_centre = new_ball.y as u16;

        // draw Ball in new Position
        
        // erase old ball
        graphics::draw_partial_circle(
            framebuffer,
            self.xpos_centre_old.into(),
            self.ypos_centre_old.into(),
            self.xpos_centre.into(),
            self.ypos_centre.into(),
            BALL_RADIUS.into(),
            BALL_RADIUS.into(),
            BGCOLOR,
        );
        // draw new ball
            graphics::draw_partial_circle(
            framebuffer,
            self.xpos_centre.into(),
            self.ypos_centre.into(),
            self.xpos_centre_old.into(),
            self.ypos_centre_old.into(),
            BALL_RADIUS.into(),
            BALL_RADIUS.into(),
            BALL_COLOR,
        );  
    }
}
