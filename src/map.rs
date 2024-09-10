use std::{
    cell::{Ref, RefCell},
    char,
    rc::Rc,
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
        let mut bsp_tree = TreeNode::new(Dimensions {
            start: Position { x: 0, y: 0 },
            height: self.tiles.len(),
            width: self.tiles[0].len(),
        });
        split_bsp_tree_node(&mut bsp_tree);
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
    pub value: Dimensions,
    pub left: Option<TreeNodeRef>,
    pub right: Option<TreeNodeRef>,
}

impl TreeNode {
    pub fn new(dims: Dimensions) -> Self {
        return TreeNode {
            value: dims,
            left: None,
            right: None,
        };
    }

    fn check_node(&self, node: &TreeNode) -> Result<(), &'static str> {
        if node.value.start.x < self.value.start.x || node.value.start.y < self.value.start.y {
            return Err("Invalid start position, before bounding box");
        }
        if node.value.start.x > self.value.start.x + self.value.height
            || node.value.start.y > self.value.start.y + self.value.width
        {
            return Err("Invalid start position, past bounding box");
        }
        if node.value.width > self.value.width {
            return Err("Node width is too large");
        }
        if node.value.height > self.value.height {
            return Err("Node height is too large");
        }
        if node.value.start.y + node.value.width > self.value.start.y + self.value.width {
            return Err("Node extends past parent width");
        }
        if node.value.start.x + node.value.height > self.value.start.x + self.value.height {
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
        assert_eq!(tree_root.value.start.x, 0);
        assert_eq!(tree_root.value.start.y, 0);
        assert_eq!(tree_root.value.width, 10);
        assert_eq!(tree_root.value.height, 10);

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

        let left_val = &left.unwrap().value;
        assert_eq!(left_val.start.x, 0);
        assert_eq!(left_val.start.y, 0);
        assert_eq!(left_val.width, 50);
        assert_eq!(left_val.height, 100);

        let right_val = &right.unwrap().value;
        assert_eq!(right_val.start.x, 0);
        assert_eq!(right_val.start.y, 50);
        assert_eq!(right_val.width, 50);
        assert_eq!(right_val.height, 100);

        Ok(())
    }
}

fn split_bsp_tree_node(node: &mut TreeNode) -> &mut TreeNode {
    if node.value.width <= 5 || node.value.height <= 5 {
        return node;
    }
    // let mut rng = rand::thread_rng();
    let split_width = node.value.width / 2;
    let split_height = node.value.height / 2;
    let mut left = TreeNode {
        value: Dimensions {
            start: Position {
                x: node.value.start.x,
                y: node.value.start.y,
            },
            width: split_width,
            height: split_height,
        },
        left: None,
        right: None,
    };

    let mut right = TreeNode {
        value: Dimensions {
            start: Position {
                x: node.value.start.x + split_height,
                y: node.value.start.y + split_width,
            },
            width: split_width,
            height: split_height,
        },
        left: None,
        right: None,
    };

    split_bsp_tree_node(&mut left);
    split_bsp_tree_node(&mut right);

    node.insert_left(left).unwrap();
    node.insert_right(right).unwrap();

    node
}

fn draw_rooms(node: &TreeNode, tiles: &mut Vec<Vec<Tile>>) {
    if node.left.is_none() || node.right.is_none() {
        for row_index in node.value.start.x..node.value.start.x + node.value.height {
            for tile_index in node.value.start.y..node.value.start.y + node.value.width {
                tiles[row_index][tile_index] = Tile {
                    is_solid: false,
                    display: '.',
                }
            }
        }
        return;
    }
    let (left, right) = node.get_children();
    draw_rooms(&left.unwrap(), tiles);
    draw_rooms(&right.unwrap(), tiles);
}
