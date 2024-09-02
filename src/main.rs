use concoeur::{game::game_loop, terminal::enter_raw_mode};

fn main() {
    let restore_fn = enter_raw_mode();

    game_loop();

    restore_fn();
}
