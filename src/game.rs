use std::io::{self, Read, Write};

use crate::{
    components::{Direction, Player, Position, Renderable},
    ecs::World,
    map::Map,
    terminal::clear_screen,
};

pub fn start_game() {
    let mut stdin = io::stdin().lock();

    clear_screen();

    let world = new_game();
    draw_world(&world);
    println!("Raw mode is on. Press 'q' to exit.");

    // Read input one byte at a time
    let mut buffer = [0; 1];
    while stdin.read(&mut buffer).unwrap() > 0 {
        match buffer[0] {
            b'q' => break,
            b'h' => move_player(Direction { x: 0, y: -1 }, &world),
            b'y' => move_player(Direction { x: -1, y: -1 }, &world),
            b'k' => move_player(Direction { x: -1, y: 0 }, &world),
            b'u' => move_player(Direction { x: -1, y: 1 }, &world),
            b'l' => move_player(Direction { x: 0, y: 1 }, &world),
            b'n' => move_player(Direction { x: 1, y: 1 }, &world),
            b'j' => move_player(Direction { x: 1, y: 0 }, &world),
            b'b' => move_player(Direction { x: 1, y: -1 }, &world),
            _ => {}
        };
        clear_screen();
        draw_world(&world);
        println!("Raw mode is on. Press 'q' to exit.");
    }
}

fn new_game() -> World {
    let mut world = World::new();
    let mut map = Map::new(21, 80);
    // map.generate_random_map();
    map.generate_bsp_map();
    world.add_resource(map);
    world.register_component::<Position>();
    world.register_component::<Renderable>();
    world.register_component::<Player>();
    world
        .create_entity()
        .with_component(Position { x: 19, y: 69 })
        .unwrap_or_else(|err| panic!("new_game, {}", err))
        .with_component(Renderable { display: '@' })
        .unwrap_or_else(|err| panic!("new_game, {}", err))
        .with_component(Player::default())
        .unwrap_or_else(|err| panic!("new_game, {}", err));

    world
}

fn draw_world(world: &World) {
    let mut query = world.query();
    let query_entities = query
        .with_component::<Renderable>()
        .unwrap_or_else(|err| panic!("draw_world, {}", err))
        .with_component::<Position>()
        .unwrap_or_else(|err| panic!("draw_world, {}", err))
        .run_query();

    let map = world.get_resource::<Map>();
    let mut buffer = String::from("");
    if let Some(map) = map {
        map.tiles.iter().enumerate().for_each(|(row_index, row)| {
            row.iter().enumerate().for_each(|(tile_index, tile)| {
                let found_entity = query_entities.iter().find(|&entity| {
                    let position = entity.get_component_mut::<Position>().unwrap();
                    return position.x == row_index && position.y == tile_index;
                });
                if let Some(entity) = found_entity {
                    buffer.push(entity.get_component::<Renderable>().unwrap().display);
                } else {
                    buffer.push(tile.display);
                }
            });
            buffer.push('\n');
        });

        let mut stdout = std::io::stdout().lock();
        stdout.write_all(buffer.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }
}

fn move_player(dir: Direction, world: &World) {
    let mut query = world.query();
    let query_entities = query
        .with_component::<Player>()
        .unwrap_or_else(|err| panic!("move_player, {}", err))
        .with_component::<Position>()
        .unwrap_or_else(|err| panic!("move_player, {}", err))
        .run_query();

    let mut position = query_entities[0].get_component_mut::<Position>().unwrap();

    let map = world.get_resource::<Map>();
    if let Some(map) = map {
        let new_pos = position.add_dir(&dir);
        if map.tiles[new_pos.x][new_pos.y].is_solid {
            ()
        } else {
            position.add_dir_mut(dir);
        }
    }
}
