pub struct Direction {
    pub x: i32,
    pub y: i32,
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn add_signed_to_usize(u: usize, i: i32) -> usize {
        if i.is_negative() {
            let option = u.checked_sub(i.wrapping_abs().try_into().unwrap());
            if let Some(x) = option {
                x
            } else {
                0
            }
        } else {
            let option = u.checked_add(i as usize);
            if let Some(x) = option {
                x
            } else {
                0
            }
        }
    }

    pub fn add_dir_mut(&mut self, dir: Direction) {
        self.x = Self::add_signed_to_usize(self.x, dir.x);
        self.y = Self::add_signed_to_usize(self.y, dir.y);
    }

    pub fn add_dir(&mut self, dir: &Direction) -> Self {
        Position {
            x: Self::add_signed_to_usize(self.x, dir.x),
            y: Self::add_signed_to_usize(self.y, dir.y),
        }
    }
}

pub struct Renderable {
    pub display: char,
}

#[derive(Default)]
pub struct Player {}
