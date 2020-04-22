use crate::{graphics, Context, Scene, World, KeyCode, level, util};


pub struct MenuScene {
    title_text: graphics::Text,
    begin_text: graphics::Text,

    is_done: bool,
}

impl MenuScene {
    pub fn new(ctx: &mut Context) -> Self {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf").unwrap();
        let title_text_fragment = graphics::TextFragment::new("Lava floor")
            .color(graphics::Color::from((192, 128, 64, 255)))
            .font(font)
            .scale(graphics::Scale::uniform(56.0));

        let begin_text_fragment = graphics::TextFragment::new("Press space to begin")
            .color(graphics::WHITE)
            .font(font)
            .scale(graphics::Scale::uniform(36.0));

        Self { 
            title_text: graphics::Text::new(title_text_fragment),
            begin_text: graphics::Text::new(begin_text_fragment),
            is_done: false,
        }
    }
}

impl Scene<World> for MenuScene {
    fn update(
        &mut self, 
        _ctx: &mut Context, 
        _world: &mut World
    ) -> Option<Box<dyn Scene<World>>> {
        if self.is_done {
            let next_scene = level::LevelScene::new();
            Some(Box::new(next_scene))
        } else {
            None
        }
    }

    fn draw(
        &mut self, 
        ctx: &mut Context, 
        _world: &mut World
    ) {
        let (window_width, window_height) = graphics::drawable_size(ctx);

        let title_text_width = self.title_text.width(ctx) as f32;
        let title_text_height = self.title_text.height(ctx) as f32;

        
        let title_text_dest = ggez::nalgebra::Point2::new(
            (window_width as f32 / 2.0) - (title_text_width / 2.0),
            (window_height as f32 / 2.0) - (title_text_height + 20.0),
        );

        graphics::draw(
            ctx,
            &self.title_text,
            graphics::DrawParam::new()
                .dest(title_text_dest)
        )
       .unwrap();

       let begin_text_width = self.begin_text.width(ctx) as f32;
       let begin_text_height = self.begin_text.height(ctx) as f32;

       let begin_text_dest = ggez::nalgebra::Point2::new(
           (window_width as f32 / 2.0) - (begin_text_width / 2.0),
           (window_height as f32 / 2.0) + (title_text_height + 20.0) - (begin_text_height + 20.0),
       );

       graphics::draw(
            ctx,
            &self.begin_text,
            graphics::DrawParam::new()
                .dest(begin_text_dest)
       )
      .unwrap();
    }
    
    fn input(
        &mut self, 
        _world: &mut World, 
        keycode: KeyCode, 
        _started: bool, 
        _repeat: bool
    ) {
        if keycode == KeyCode::Space {
            self.is_done = true;
        }
    }
    
    fn name(&self) -> &str {
        "Menu Scene"
    }
}