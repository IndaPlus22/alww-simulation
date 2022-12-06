// Ex 1.5

use ggez::{graphics, Context, GameResult};
use noise::{NoiseFn, Perlin};

use crate::{utilities::Vector2, SCREEN_SIZE};

pub struct Noise_walker_vec {
    pos: Vector2<f32>,
    offsetX: Vector2<f64>,
    offsetY: Vector2<f64>,
}

impl Noise_walker_vec {
    pub fn new() -> Self {
        let _pos = Vector2::new(1.0, 1.0);
        let _offset = Vector2::new(1.0, 1.0);

        Self {
            pos: _pos,
            offsetX: _offset,
            offsetY: _offset,
        }
    }

    pub fn step(&mut self, noice: &Perlin) {
        let tmpX = noice.get(self.offsetX.param()) as f32 + 1.0;
        let tmpY = noice.get(self.offsetY.param()) as f32 + 1.0;
        self.pos.x = tmpX * (SCREEN_SIZE).0 / 2.0;
        self.pos.y = tmpY * (SCREEN_SIZE).1 / 2.0;
        self.offsetX.x += 0.01;
        self.offsetY.y += 0.01;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.pos.param(),
            100.0,
            0.1,
            graphics::Color::BLACK,
        )?;

        canvas.draw(&circle, graphics::DrawParam::default());
        canvas.finish(ctx)?;
        Ok(())
    }
}
