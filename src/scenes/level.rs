use crate::{Context, graphics, Scene, World, KeyCode, platform::Platform, player::Player, util};
use nalgebra::{Point2, Vector2};

#[allow(dead_code)]
pub struct LevelScene {
    platforms: Vec<Platform>,
    player: Player,

    is_done: bool,
}

impl LevelScene {
    pub fn new(
        _ctx: &mut Context, 
        world: &mut World
    ) -> Self {
        let platforms = vec!{
            Platform::new(
                Vector2::new(800., 1.), 
                Vector2::new(0., 300.), 
                Point2::new(0., 0.), 
                world
            ),
            Platform::new(
                Vector2::new(20., 50.), 
                Vector2::new(100., 300.), 
                Point2::new(100., 10.), 
                world
            ),
            Platform::new(
                Vector2::new(20., 40.), 
                Vector2::new(300., 300.), 
                Point2::new(100., 10.), 
                world
            ),
            Platform::new(
                Vector2::new(20., 60.), 
                Vector2::new(500., 300.), 
                Point2::new(100., 10.), 
                world
            ),
        };

        let player = Player::new(world);

        Self {
            platforms,
            player,
            is_done: false,
        }
    }

    fn draw_colliders(&mut self, ctx: &mut Context, world: &mut World) {
        for (_, collider) in world.physics.colliders.iter() {
            let shape = collider.shape().aabb(collider.position());
            let rect = graphics::Rect::new(
                shape.mins().x,
                shape.mins().y,
                shape.extents().x,
                shape.extents().y,
            );

            let circle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::Stroke(graphics::StrokeOptions::DEFAULT),
                rect,
                graphics::WHITE,
            )
            .unwrap();

            graphics::draw(
                ctx, 
                &circle, 
                graphics::DrawParam::new()
                    .dest(util::point_to_old(Point2::new(100., 100.))),
            )
            .unwrap();
        }
    }
}

impl Scene<World> for LevelScene {
    fn update(
        &mut self, 
        ctx: &mut Context, 
        world: &mut World
    ) -> Option<Box<dyn Scene<World>>> {
        if self.is_done {
            // TODO: Switch to game over scene
        }

        world.physics.step();
        world.physics.ticks += 1;

        self.player.update(ctx, world);

        None
    }

    fn draw(
        &mut self, 
        ctx: &mut Context, 
        world: &mut World
    ) {
        self.draw_colliders(ctx, world);
    }
    
    fn input(
        &mut self, 
        _world: &mut World, 
        keycode: KeyCode, 
        pressed: bool, 
        repeat: bool
    ) {
        match keycode {
            KeyCode::A | KeyCode::Left => self.player.input.left = pressed,
            KeyCode::D | KeyCode::Right => self.player.input.right = pressed,
            KeyCode::Space => self.player.input.jump = pressed && !repeat,
            _ => (),
        };
    }
    
    fn name(&self) -> &str {
        "Level Scene"
    }
}