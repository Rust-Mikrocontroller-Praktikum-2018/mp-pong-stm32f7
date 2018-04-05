/*
// high level
fn main() {
    println!("Hello, world!");

    let is_server = false;

    loop {
        if is_server {
            server_loop();
        }

        game_loop();
    }
}

fn server_loop() {
    receive_input_from_clients();
    game_update();
    send_state_to_clients();
}


fn game_loop() {
    receive_state_from_server();
    read_input();
    send_input_to_server();
    draw_stuff();
}
*/