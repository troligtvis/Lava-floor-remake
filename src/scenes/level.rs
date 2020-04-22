use crate::{gfx, Context, Scene, World, KeyCode};

pub struct LevelScene {

}

impl LevelScene {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

impl Scene<World> for LevelScene {
    fn update(
        &mut self, 
        ctx: &mut Context, 
        world: &mut World
    ) -> Option<Box<dyn Scene<World>>> {
        None
    }

    fn draw(
        &mut self, 
        ctx: &mut Context, 
        world: &mut World
    ) {

    }
    
    fn input(
        &mut self, 
        world: &mut World, 
        keycode: KeyCode, 
        started: bool, 
        repeat: bool
    ) {

    }
    
    fn name(&self) -> &str {
        ""
    }
}