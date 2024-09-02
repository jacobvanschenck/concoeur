use std::io::{self, Read};

use crate::terminal::clear_screen;

pub fn game_loop() {
    let mut stdin = io::stdin().lock();

    clear_screen();

    println!("Raw mode is on. Press 'q' to exit.");

    // Read input one byte at a time
    let mut buffer = [0; 1];
    while stdin.read(&mut buffer).unwrap() > 0 {
        match buffer[0] {
            b'q' => break,
            _ => {}
        }
    }
}
