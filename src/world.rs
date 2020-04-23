use crate::{physics};

pub struct World {
    pub physics: physics::Physics2D,
}

impl World {
    pub fn new() -> Self {
        Self {
            physics: physics::Physics2D::new(),
        }
    }
}