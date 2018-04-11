use game::GameState;
use input::Input;
use lcd;
use lcd::Framebuffer;
use lcd::TextWriter;

pub fn choose_local_network(
    just_entered: bool,
    framebuffer: &mut Framebuffer,
    text_writer: &mut TextWriter,
    input: &mut Input,
) -> GameState {
    if just_entered {
        framebuffer.clear();
        text_writer.write_at(framebuffer, "Local Multiplayer", 40, 125);
        // text_writer.write_at(framebuffer, "Play on one device", 200, 150);
        text_writer.write_at(framebuffer, "Network Multiplayer", 270, 125);
    }

    let touch = input.handle_menu();

    if touch.is_down && !touch.any_touch_last_frame {
        if touch.x < lcd::WIDTH as u16 / 2 {
            return GameState::GameRunningLocal;
        } else {
            return GameState::ChooseClientOrServer;
        }
    }

    GameState::ChooseLocalOrNetwork
}

pub fn choose_client_server(
    just_entered: bool,
    framebuffer: &mut Framebuffer,
    text_writer: &mut TextWriter,
    input: &mut Input,
    is_server: &mut bool,
) -> GameState {
    if just_entered {
        framebuffer.clear();
        text_writer.write_at(framebuffer, "Client", 95, 125);
        text_writer.write_at(framebuffer, "Server", 335, 125);
    }

    let touch = input.handle_menu();

    if touch.is_down && !touch.any_touch_last_frame {
        if touch.x < lcd::WIDTH as u16 / 2 {
            *is_server = false;
            return GameState::ConnectToNetwork;
        } else {
            *is_server = true;
            return GameState::ConnectToNetwork;
        }
    }

    GameState::ChooseClientOrServer
}

pub fn choose_only_local(
    just_entered: bool,
    framebuffer: &mut Framebuffer,
    text_writer: &mut TextWriter,
    input: &mut Input,
) -> GameState {
    if just_entered {
        // framebuffer.clear(); // Don't clear as we cleared and wrote the debug message before
        text_writer.write_at(framebuffer, "Local Multiplayer", 40, 125);
        // text_writer.write_at(framebuffer, "Play on one device", 200, 150);
        text_writer.write_at(framebuffer, "-------------------", 270, 125);
    }

    let touch = input.handle_menu();

    if touch.is_down && !touch.any_touch_last_frame {
        if touch.x < lcd::WIDTH as u16 / 2 {
            return GameState::GameRunningLocal;
        }
    }

    GameState::ChooseOnlyLocal
}
