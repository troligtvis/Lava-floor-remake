use crate::{Context, Scene, World, KeyCode, platform::Platform};
use nalgebra::{Point2, Vector2};

#[allow(dead_code)]
pub struct LevelScene {
    platforms: Vec<Platform>,
}

impl LevelScene {
    pub fn new(
        _ctx: &mut Context, 
        world: &mut World
    ) -> Self {
        let platforms = vec!{
            Platform::new(
                Vector2::new(250., 1.), 
                Vector2::new(0., 50.), 
                Point2::new(0., 0.), 
                world
            ),
            Platform::new(
                Vector2::new(20., 50.), 
                Vector2::new(100., 10.), 
                Point2::new(100., 10.), 
                world
            ),
        };

        Self {
            platforms,
        }
    }
}

impl Scene<World> for LevelScene {
    fn update(
        &mut self, 
        _ctx: &mut Context, 
        _world: &mut World
    ) -> Option<Box<dyn Scene<World>>> {
        None
    }

    fn draw(
        &mut self, 
        _ctx: &mut Context, 
        _world: &mut World
    ) {

    }
    
    fn input(
        &mut self, 
        _world: &mut World, 
        _keycode: KeyCode, 
        _started: bool, 
        _repeat: bool
    ) {

    }
    
    fn name(&self) -> &str {
        "Level Scene"
    }
}