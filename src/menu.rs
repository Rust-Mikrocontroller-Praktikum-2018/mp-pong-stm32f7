use game::GameState;
use lcd::Framebuffer;
use lcd::TextWriter;
use i2c;
use touch;
use lcd;

pub fn choose_local_network(
    just_entered: bool,
    framebuffer: &mut Framebuffer,
    text_writer: &mut TextWriter,
    i2c_3: &mut i2c::I2C,
) -> GameState {
    if just_entered {
        framebuffer.clear();
        text_writer.write_at(framebuffer, "Local Multiplayer", 40, 130);
        //text_writer.write_at(framebuffer, "Play on one device", 200, 150);
        text_writer.write_at(
            framebuffer,
            "Network Multiplayer",
            260,
            130,
        );
    }

    for touch in &touch::touches(i2c_3).unwrap() {
        if touch.x < lcd::WIDTH as u16 / 2 {
            return GameState::GameRunningLocal
        } else {
            return GameState::ChooseClientOrServer
        }
    }
    
    GameState::ChooseLocalOrNetwork
}
