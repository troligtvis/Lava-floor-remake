#![allow(dead_code)]

use std::{env, path, time::Duration};

use ggez::*;
use ggez::{ContextBuilder, Context, GameResult};
use ggez::event::{EventHandler, KeyCode, KeyMods};

extern crate nalgebra as na;
use na::Point2;

mod gfx;
mod util;
mod physics;
mod platform;
mod player;

mod scenes;
use scenes::*;

mod world;
use world::*;

pub struct MainState {
    world: World,
    current_scene: Box<dyn Scene<World>>,
    ticks: usize,
    dt: Duration,
}

impl EventHandler for MainState {
    fn update(
        &mut self, 
        ctx: &mut Context
    ) -> GameResult<()> {
        self.dt = timer::delta(ctx);

        if let Some(next_scene) = self.current_scene.update(ctx, &mut self.world) {
            self.current_scene = next_scene
        } 

        Ok(())
    }

    fn draw(
        &mut self, 
        ctx: &mut Context
    ) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        self.current_scene.draw(ctx, &mut self.world);

        let (window_width, window_height) = graphics::drawable_size(ctx);
        let new_rect = graphics::Rect::new(0.0, 0.0, window_width, window_height);  
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
        graphics::apply_transformations(ctx).unwrap();

        // Draw scene name
        //draw_current_scene_text(ctx, self.current_scene.name());

        graphics::present(ctx)?;

        self.ticks += 1;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            event::quit(_ctx)
        }

        self.current_scene.input(&mut self.world, keycode, true, repeat)
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods
    ) {
        self.current_scene.input(&mut self.world, keycode, false, false);    
    }
}

// FIXME: Very slow
fn draw_current_scene_text(ctx: &mut Context, text: &str) {
    let fragment = graphics::TextFragment::new(text)
    .color(graphics::Color::from((192, 128, 64, 255)))
    .font(graphics::Font::new(ctx, "/DejaVuSerif.ttf").unwrap())
    .scale(graphics::Scale::uniform(16.0));
    let scene_text = graphics::Text::new(fragment);

    graphics::queue_text(
        ctx, &scene_text, 
        util::point_to_old(Point2::new(10., 10.)), 
        None
    );

    graphics::draw_queued_text(
        ctx,
        graphics::DrawParam::new(),
        None,
        graphics::FilterMode::Nearest,
    ).unwrap();
}

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("lava_floor", "troligtvis")
        .add_resource_path(resource_dir)
        .conf(c)
        .build()
        .unwrap();


    let initial_screen = Box::new(scenes::menu::MenuScene::new(ctx));
    
    let state = &mut MainState {
        world: World::new(),
        current_scene: initial_screen,
        dt: std::time::Duration::new(0, 0),
        ticks: 0usize,
    };

    event::run(ctx, event_loop, state).unwrap();
}
