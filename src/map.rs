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

#[derive(Debug)]
struct Dimensions {
    pub start: Position,
    pub width: i32,
    pub height: i32,
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

    pub fn insert_left(&mut self, node: TreeNode) {
        self.left = Some(Rc::new(RefCell::new(node)));
    }

    pub fn insert_right(&mut self, node: TreeNode) {
        self.right = Some(Rc::new(RefCell::new(node)));
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
    fn insert_into_root() {
        let mut tree_root = TreeNode::new(Dimensions {
            start: Position { x: 0, y: 0 },
            width: 100,
            height: 100,
        });

        tree_root.insert_left(TreeNode::new(Dimensions {
            start: Position { x: 0, y: 0 },
            width: 50,
            height: 100,
        }));

        tree_root.insert_right(TreeNode::new(Dimensions {
            start: Position { x: 0, y: 51 },
            width: 50,
            height: 100,
        }));

        let (left, right) = tree_root.get_children();

        let left_val = &left.unwrap().value;
        assert_eq!(left_val.start.x, 0);
        assert_eq!(left_val.start.y, 0);
        assert_eq!(left_val.width, 50);
        assert_eq!(left_val.height, 100);

        let right_val = &right.unwrap().value;
        assert_eq!(right_val.start.x, 0);
        assert_eq!(right_val.start.y, 51);
        assert_eq!(right_val.width, 50);
        assert_eq!(right_val.height, 100);
    }
}
