use std::char;

use rand::Rng;

#[derive(Default, Clone, Debug)]
pub struct Tile {
    pub display: char,
    pub is_solid: bool,
}

#[derive(Default, Debug)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            tiles: vec![vec![Tile::default(); width]; height],
        }
    }

    pub fn generate_map(&mut self) {
        let mut rng = rand::thread_rng();
        for row in self.tiles.iter_mut() {
            for tile in row.iter_mut() {
                let random_number: f32 = rng.gen();
                if random_number > 0.80 {
                    *tile = Tile {
                        is_solid: true,
                        display: '#',
                    }
                } else {
                    *tile = Tile {
                        is_solid: false,
                        display: '.',
                    }
                }
            }
        }
    }
}
