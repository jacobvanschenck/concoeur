use std::{
    cell::{Ref, RefCell},
    char,
    rc::Rc,
    usize,
};

use rand::Rng;

use crate::components::Position;

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
            tiles: vec![
                vec![
                    Tile {
                        display: ' ',
                        is_solid: true
                    };
                    width
                ];
                height
            ],
        }
    }

    pub fn generate_bsp_map(&mut self) {
        let mut rng = rand::thread_rng();
        let mut bsp_tree = TreeNode::new(Dimensions {
            start: Position { x: 0, y: 0 },
            height: self.tiles.len(),
            width: self.tiles[0].len(),
        });
        split_bsp_tree_node(&mut bsp_tree, rng.gen_bool(1.0 / 2.0));
        draw_rooms(&bsp_tree, &mut self.tiles)
    }

    pub fn generate_random_map(&mut self) {
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

#[derive(Debug)]
struct Dimensions {
    pub start: Position,
    pub width: usize,
    pub height: usize,
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug)]
struct TreeNode {
    pub space: Dimensions,
    pub room: Option<Dimensions>,
    pub left: Option<TreeNodeRef>,
    pub right: Option<TreeNodeRef>,
}

impl TreeNode {
    pub fn new(dims: Dimensions) -> Self {
        return TreeNode {
            space: dims,
            room: None,
            left: None,
            right: None,
        };
    }

    fn check_node(&self, node: &TreeNode) -> Result<(), &'static str> {
        if node.space.start.x < self.space.start.x || node.space.start.y < self.space.start.y {
            return Err("Invalid start position, before bounding box");
        }
        if node.space.start.x > self.space.start.x + self.space.height
            || node.space.start.y > self.space.start.y + self.space.width
        {
            return Err("Invalid start position, past bounding box");
        }
        if node.space.width > self.space.width {
            return Err("Node width is too large");
        }
        if node.space.height > self.space.height {
            return Err("Node height is too large");
        }
        if node.space.start.y + node.space.width > self.space.start.y + self.space.width {
            return Err("Node extends past parent width");
        }
        if node.space.start.x + node.space.height > self.space.start.x + self.space.height {
            return Err("Node extends past parent height");
        }
        Ok(())
    }

    pub fn insert_left(&mut self, node: TreeNode) -> Result<(), &'static str> {
        self.check_node(&node)?;
        self.left = Some(Rc::new(RefCell::new(node)));

        Ok(())
    }

    pub fn insert_right(&mut self, node: TreeNode) -> Result<(), &'static str> {
        self.check_node(&node)?;
        self.right = Some(Rc::new(RefCell::new(node)));

        Ok(())
    }

    pub fn get_children(&self) -> (Option<Ref<TreeNode>>, Option<Ref<TreeNode>>) {
        let left: Option<Ref<TreeNode>>;
        let right: Option<Ref<TreeNode>>;
        if let Some(node) = &self.left {
            left = Some(node.borrow());
        } else {
            left = None;
        }

        if let Some(node) = &self.right {
            right = Some(node.borrow());
        } else {
            right = None;
        }
        (left, right)
    }

    pub fn add_room(&mut self) {
        let mut rng = rand::thread_rng();

        let min_width = self.space.width / 2;
        let min_height = self.space.height / 2;

        if self.space.width - 1 - min_width <= 0 || self.space.height - 1 - min_height <= 0 {
            return;
        }

        let new_width = rng.gen_range(min_width..self.space.width - 1);
        let new_height = rng.gen_range(min_height..self.space.height - 1);

        if new_width < 4 || new_height < 4 {
            return;
        }
        if self.space.height - new_height == 0 || self.space.width - new_width == 0 {
            return;
        }

        let delta_x = rng.gen_range(1..=self.space.height - new_height);
        let delta_y = rng.gen_range(1..=self.space.width - new_width);

        self.room = Some(Dimensions {
            start: Position {
                x: self.space.start.x + delta_x,
                y: self.space.start.y + delta_y,
            },
            height: new_height,
            width: new_width,
        });
    }
}

#[cfg(test)]
mod test {
    use super::TreeNode;
    use crate::{components::Position, map::Dimensions};

    #[test]
    fn create_new_tree() {
        let tree_root = TreeNode::new(Dimensions {
            start: Position { x: 0, y: 0 },
            width: 10,
            height: 10,
        });
        assert_eq!(tree_root.space.start.x, 0);
        assert_eq!(tree_root.space.start.y, 0);
        assert_eq!(tree_root.space.width, 10);
        assert_eq!(tree_root.space.height, 10);

        let (left, right) = &tree_root.get_children();
        assert!(left.is_none());
        assert!(right.is_none());
    }

    #[test]
    fn insert_into_root() -> Result<(), &'static str> {
        let mut tree_root = TreeNode::new(Dimensions {
            start: Position { x: 0, y: 0 },
            width: 100,
            height: 100,
        });

        tree_root.insert_left(TreeNode::new(Dimensions {
            start: Position { x: 0, y: 0 },
            width: 50,
            height: 100,
        }))?;

        tree_root.insert_right(TreeNode::new(Dimensions {
            start: Position { x: 0, y: 50 },
            width: 50,
            height: 100,
        }))?;

        let (left, right) = tree_root.get_children();

        let left_val = &left.unwrap().space;
        assert_eq!(left_val.start.x, 0);
        assert_eq!(left_val.start.y, 0);
        assert_eq!(left_val.width, 50);
        assert_eq!(left_val.height, 100);

        let right_val = &right.unwrap().space;
        assert_eq!(right_val.start.x, 0);
        assert_eq!(right_val.start.y, 50);
        assert_eq!(right_val.width, 50);
        assert_eq!(right_val.height, 100);

        Ok(())
    }
}

fn split_bsp_tree_node(node: &mut TreeNode, vertical: bool) -> &mut TreeNode {
    if node.space.width <= 10 || node.space.height <= 10 {
        node.add_room();
        return node;
    }
    let mut rng = rand::thread_rng();
    let split = rng.gen_range(0.4..0.6);

    let left_width: usize;
    let left_height: usize;
    let right_width: usize;
    let right_height: usize;
    let x_delta: usize;
    let y_delta: usize;

    if vertical {
        left_width = (node.space.width as f32 * split).floor() as usize;
        left_height = node.space.height;
        x_delta = 0;
        y_delta = left_width;
        right_width = node.space.width - left_width;
        right_height = node.space.height;
    } else {
        left_width = node.space.width;
        left_height = (node.space.height as f32 * split).floor() as usize;
        x_delta = left_height;
        y_delta = 0;
        right_width = node.space.width;
        right_height = node.space.height - left_height;
    }
    let mut left = TreeNode {
        space: Dimensions {
            start: Position {
                x: node.space.start.x,
                y: node.space.start.y,
            },
            width: left_width,
            height: left_height,
        },
        room: None,
        left: None,
        right: None,
    };

    let mut right = TreeNode {
        space: Dimensions {
            start: Position {
                x: node.space.start.x + x_delta,
                y: node.space.start.y + y_delta,
            },
            width: right_width,
            height: right_height,
        },
        room: None,
        left: None,
        right: None,
    };

    split_bsp_tree_node(&mut left, !vertical);
    split_bsp_tree_node(&mut right, !vertical);

    node.insert_left(left).unwrap();
    node.insert_right(right).unwrap();

    node
}

fn draw_rooms(node: &TreeNode, tiles: &mut Vec<Vec<Tile>>) {
    if let Some(room) = &node.room {
        let start_x = room.start.x;
        let start_y = room.start.y;
        let end_x = start_x + room.height;
        let end_y = start_y + room.width;
        tiles[start_x][start_y] = Tile {
            is_solid: true,
            display: '┌',
        };
        for tile_index in start_y + 1..end_y - 1 {
            tiles[start_x][tile_index] = Tile {
                is_solid: true,
                display: '─',
            }
        }
        tiles[start_x][end_y - 1] = Tile {
            is_solid: true,
            display: '┐',
        };
        for row_index in start_x + 1..end_x - 1 {
            tiles[row_index][start_y] = Tile {
                is_solid: true,
                display: '│',
            };
            for tile_index in start_y + 1..end_y - 1 {
                tiles[row_index][tile_index] = Tile {
                    is_solid: false,
                    display: '.',
                }
            }
            tiles[row_index][end_y - 1] = Tile {
                is_solid: true,
                display: '│',
            };
        }
        tiles[end_x - 1][start_y] = Tile {
            is_solid: true,
            display: '└',
        };
        for tile_index in start_y + 1..end_y - 1 {
            tiles[end_x - 1][tile_index] = Tile {
                is_solid: true,
                display: '─',
            }
        }
        tiles[end_x - 1][end_y - 1] = Tile {
            is_solid: true,
            display: '┘',
        };
        return;
    }
    let (left, right) = node.get_children();
    if left.is_some() {
        draw_rooms(&left.unwrap(), tiles);
    }
    if right.is_some() {
        draw_rooms(&right.unwrap(), tiles);
    }
}
