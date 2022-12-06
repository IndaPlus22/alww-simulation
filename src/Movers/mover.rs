use std::sync::Arc;

use crate::{utilities::Vector2, SCREEN_SIZE};
use ggez::{
    graphics::{self, Canvas},
    input, Context, GameResult,
};
use noise::{NoiseFn, Perlin};
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug)]
pub struct Mover {
    pos: Vector2<f32>,
    speed: Vector2<f32>,
    acc: Vector2<f32>,
    offsetX: Vector2<f64>,
    offsetY: Vector2<f64>,
}

impl Mover {
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
        let mouse_pos = ctx.mouse.position();
        let mut mouse_vec = Vector2::new(mouse_pos.x, mouse_pos.y);

        // //Perlin random acc
        // let tmpX = noice.get(self.offsetX.param()) as f32 / 5.0;
        // let tmpY = noice.get(self.offsetY.param()) as f32 / 5.0;
        // self.acc.x = tmpX;
        // self.acc.y = tmpY;
        // self.offsetX.x += 0.01;
        // self.offsetY.y += 0.01;

        // // Rand random acc
        // self.acc.x = rng.gen_range(-10.0..10.0);
        // self.acc.y = rng.gen_range(-10.0..10.0);

        mouse_vec -= self.pos;
        let magnitude = mouse_vec.magnitude();
        mouse_vec.normalize();
        mouse_vec.multScalar(magnitude / 125.0);
        self.acc = mouse_vec;

        self.speed = self.speed + self.acc;
        self.speed.limit(25.0);
        self.pos = self.pos + self.speed;
    }

    pub fn check_edges(&mut self) {
        if (self.pos.x > SCREEN_SIZE.0) {
            self.pos.x = 0.0;
        } else if (self.pos.x < 0.0) {
            self.pos.x = SCREEN_SIZE.0;
        }
        if (self.pos.y > SCREEN_SIZE.1) {
            self.pos.y = 0.0;
        } else if (self.pos.y < 0.0) {
            self.pos.y = SCREEN_SIZE.1;
        }
    }

    pub fn check_keys(&mut self, ctx: &mut Context) {
        if ctx
            .keyboard
            .is_key_pressed(ggez::winit::event::VirtualKeyCode::Up)
        {
            self.acc = self.acc + Vector2::new(0.1, 0.1);
        }
        if ctx
            .keyboard
            .is_key_pressed(ggez::winit::event::VirtualKeyCode::Down)
        {
            self.acc = self.acc + Vector2::new(-0.2, -0.2);
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

pub struct ManyMovers {
    movers: Vec<Mover>,
}
impl ManyMovers {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut _movers = Vec::new();
        for _ in 0..20 {
            let x = rng.gen_range(100.0..(SCREEN_SIZE.0 - 100.0));
            let y = rng.gen_range(100.0..(SCREEN_SIZE.1 - 100.0));
            let mover = Mover::new(x, y);
            println!("{:?}", x);
            println!("{:?}", y);
            _movers.push(mover);
        }

        Self { movers: _movers }
    }
    pub fn drawMany(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        for Mover in &mut self.movers {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Mover.pos.param(),
                10.0,
                0.1,
                graphics::Color::BLACK,
            )?;

            canvas.draw(&circle, graphics::DrawParam::default());
        }
        canvas.finish(ctx)?;
        Ok(())
    }
    pub fn updateMany(&mut self, ctx: &mut Context, noice: &Perlin) {
        let mut rng = rand::thread_rng();
        for Mover in &mut self.movers {
            Mover.check_keys(ctx);
            Mover.update(ctx, &mut rng, noice);
            Mover.check_edges();
        }
    }
}
