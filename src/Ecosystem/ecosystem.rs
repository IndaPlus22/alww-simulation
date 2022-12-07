use std::sync::Arc;

use crate::{utilities::Vector2, Movers::mover::Mover, SCREEN_SIZE};
use ggez::{
    graphics::{self, Canvas},
    input, Context, GameResult,
};
use noise::{NoiseFn, Perlin};
use rand::{rngs::ThreadRng, Rng};

use super::{fish::Fish, fly::Fly};

#[derive(Debug)]
pub struct EcoSystem {
    flies: Vec<Fly>,
    fish: Vec<Fish>,
}
impl EcoSystem {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut _flies = Vec::new();
        for _ in 0..10 {
            let x = rng.gen_range(100.0..(SCREEN_SIZE.0 - 100.0));
            let y = rng.gen_range(100.0..(SCREEN_SIZE.1 - 100.0));
            let mover = Fly::new(x, y);
            _flies.push(mover);
        }

        let mut _fishies = Vec::new();
        for _ in 0..10 {
            let x = rng.gen_range(100.0..(SCREEN_SIZE.0 - 100.0));
            let y = rng.gen_range(100.0..(SCREEN_SIZE.1 - 100.0));
            let mover = Fish::new(x, y);
            _fishies.push(mover);
        }

        Self {
            flies: _flies,
            fish: _fishies,
        }
    }
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        let water = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: SCREEN_SIZE.0 / 2.0,
                y: SCREEN_SIZE.1 / 2.0,
                w: (SCREEN_SIZE.0 / 2.0),
                h: (SCREEN_SIZE.0 / 2.0),
            },
            graphics::Color::CYAN,
        )?;
        canvas.draw(&water, graphics::DrawParam::default());
        for Fly in &mut self.flies {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Fly.pos.param(),
                10.0,
                0.1,
                graphics::Color::BLACK,
            )?;

            canvas.draw(&circle, graphics::DrawParam::default());
        }
        for Fish in &mut self.fish {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Fish.pos.param(),
                10.0,
                0.1,
                graphics::Color::BLUE,
            )?;

            canvas.draw(&circle, graphics::DrawParam::default());
        }
        canvas.finish(ctx)?;
        Ok(())
    }
    pub fn update(&mut self, ctx: &mut Context, noice: &Perlin) {
        let mut rng = rand::thread_rng();
        for Mover in &mut self.flies {
            Mover.update(ctx, &mut rng, noice);
            Mover.check_edges();
        }
        for Mover in &mut self.fish {
            Mover.update(ctx, &mut rng, noice);
            Mover.check_edges();
        }
    }
}
