use concoeur::{game::start_game, terminal::enter_raw_mode};

fn main() {
    let restore_fn = enter_raw_mode();

    start_game();

    restore_fn();
}
