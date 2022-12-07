use std::sync::Arc;

use crate::{utilities::Vector2, SCREEN_SIZE};
use ggez::{
    graphics::{self, Canvas},
    input, Context, GameResult,
};
use noise::{NoiseFn, Perlin};
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug)]
pub struct Fly {
    pub pos: Vector2<f32>,
    speed: Vector2<f32>,
    acc: Vector2<f32>,
    offsetX: Vector2<f64>,
    offsetY: Vector2<f64>,
}

impl Fly {
    pub fn new(x: f32, y: f32) -> Self {
        let _pos = Vector2::new(x, y);
        let _vec_pos = Vector2::new(0.0, 0.0);
        let _speed = Vector2::new(1.0, 1.0);
        let _acc = Vector2::new(0.0, 0.0);
        let _offset = Vector2::new(1.0, 1.0);

        Self {
            pos: _pos,
            speed: _speed,
            acc: _acc,
            offsetX: _offset,
            offsetY: _offset,
        }
    }

    pub fn update(&mut self, ctx: &mut Context, rng: &mut ThreadRng, noice: &Perlin) {
        // Perlin random acc
        let tmpNX = noice.get(self.offsetX.param()) as f32 / 2.0;
        let tmpNY = noice.get(self.offsetY.param()) as f32 / 2.0;
        let mut tmpNVec = Vector2::new(tmpNX, tmpNY);
        self.offsetX += Vector2::new(0.1, 0.1);

        // Rand random acc
        let tmpRX = rng.gen_range(-2.0..2.0);
        let tmpRY = rng.gen_range(-2.0..2.0);
        let mut tmpRVec = Vector2::new(tmpRX, tmpRY);

        tmpNVec -= tmpRVec;
        // mouse_vec -= tmpRVec;
        tmpNVec.normalize();
        tmpNVec.multScalar(10.0);
        self.acc = tmpNVec;

        self.speed = self.speed + self.acc;
        self.speed.limit(20.0);
        self.pos = self.pos + self.speed;
        self.check_edges();
    }

    pub fn check_edges(&mut self) {
        if (self.pos.x > SCREEN_SIZE.0) {
            self.pos.x = SCREEN_SIZE.0;
        } else if (self.pos.x < 0.0) {
            self.pos.x = 0.0;
        }
        if (self.pos.y > SCREEN_SIZE.1) {
            self.pos.y = SCREEN_SIZE.1;
        } else if (self.pos.y < 0.0) {
            self.pos.y = 0.0;
        }
        if (self.pos.x > (SCREEN_SIZE.0 / 2.0) && self.pos.y > (SCREEN_SIZE.1 / 2.0)) {
            if ((self.pos.x - (SCREEN_SIZE.0 / 2.0)) < (self.pos.y - (SCREEN_SIZE.1 / 2.0))) {
                self.pos.x -= (self.pos.x - (SCREEN_SIZE.0 / 2.0));
            } else {
                self.pos.y -= (self.pos.y - (SCREEN_SIZE.1 / 2.0));
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.pos.param(),
            10.0,
            0.1,
            graphics::Color::BLACK,
        )?;

        canvas.draw(&circle, graphics::DrawParam::default());
        canvas.finish(ctx)?;
        Ok(())
    }
}
