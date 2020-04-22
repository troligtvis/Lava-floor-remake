pub mod menu;
pub mod level;

use crate::{Context, KeyCode};

pub trait Scene<W> {
    fn update(&mut self, ctx: &mut Context, world: &mut W) -> Option<Box<dyn Scene<W>>>;
    fn draw(&mut self, ctx: &mut Context, world: &mut W);
    fn input(&mut self, world: &mut W, keycode: KeyCode, started: bool, repeat: bool);
    fn name(&self) -> &str;
}