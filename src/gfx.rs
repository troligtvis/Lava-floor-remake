use ggez::graphics::{self, Drawable};
use ggez::Context;

use crate::{util};

use nalgebra::{Point2, Vector2};

const PAWN_RADIUS: f32 = 10.0; // TODO: move 

pub struct Sprite {
    image: graphics::Image,
    dimension: (u16, u16),

    // Todo: Add sprite batch logic :shrug:
    //sprite_batch: Vec<graphics::spritebatch::SpriteBatch>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum DrawPawnType {
    Player,
}

pub struct PawnDrawData {
    pub position: Point2<f32>,
    pub rotation: f32,
    pub pawn_type: DrawPawnType,
}

impl Sprite {
    pub fn new(ctx: &mut Context, name: String) -> Self {
        let image = graphics::Image::new(ctx, name).unwrap();
        let dimension = (image.width(), image.height());
        
        Self {
            image,
            dimension,
        }
    }
}

pub struct GfxUtil {
    pub player_sprite: Sprite,
}

impl GfxUtil {
    pub fn new(ctx: &mut Context) -> Self {
        Self{
            player_sprite: Sprite::new(ctx, "/sprites/player.png".to_string()),
        }
    }

    pub fn draw_pawns<I: Iterator<Item = PawnDrawData>>(&mut self, ctx: &mut Context, pawns: I) {
        for pawn in pawns {
            let image = if pawn.pawn_type == DrawPawnType::Player {
                &self.player_sprite.image
            } else {
                unimplemented!()
            };

            let (image_width, image_height) = self.player_sprite.dimension;
            // let half_w = image_width as f32 / PAWN_RADIUS;
            // let half_h = image_height as f32 / PAWN_RADIUS;

            let position = pawn.position;
            let dest = Point2::new(position.x, position.y);

            let dest = util::point_to_old(Point2::new(
                dest.x + ((image_width as f32 + PAWN_RADIUS) / 2.), 
                dest.y + ((image_height as f32 + PAWN_RADIUS) / 2.),
            ));
            let scale = util::vector_to_old(Vector2::new(
                (PAWN_RADIUS / image_width as f32) * 2.,
                (PAWN_RADIUS / image_height as f32) * 2.,
            ));
            let offset = util::point_to_old(Point2::new(-0.5, -0.5));

            let param = graphics::DrawParam::default()
            .dest(dest)
            .scale(scale)
            .offset(offset);

            image.draw(ctx, param)
                .unwrap();
        }
    }
}