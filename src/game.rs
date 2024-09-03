use std::io::{self, Read};

use crate::{ecs::World, map::Map, terminal::clear_screen};

pub fn start_game() {
    let mut stdin = io::stdin().lock();

    clear_screen();

    let world = new_game();
    print_map(&world);

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

fn new_game() -> World {
    let mut world = World::new();
    let mut map = Map::new(40, 60);
    map.generate_map();
    world.add_resource(map);
    world
}

fn print_map(world: &World) {
    let map = world.get_resource::<Map>();
    if let Some(map) = map {
        for row in &map.tiles {
            for tile in row {
                print!("{} ", &tile.display)
            }
            println!();
        }
    }
}
