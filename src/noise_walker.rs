// Ex 1.5

use ggez::{graphics, Context, GameResult};
use noise::{NoiseFn, Perlin};

use crate::SCREEN_SIZE;

pub struct Noise_walker {
    pub X: f32,
    pub Y: f32,
    XOFF: [f64; 2],
    YOFF: [f64; 2],
}

impl Noise_walker {
    pub fn new() -> Self {
        let x = 0.0;
        let y = 0.0;
        let xoff = [0.0; 2];
        let yoff = [0.0; 2];

        Self {
            X: x,
            Y: y,
            XOFF: xoff,
            YOFF: yoff,
        }
    }

    pub fn step(&mut self, noice: &Perlin) {
        let tmpX = noice.get(self.XOFF) as f32 + 1.0;
        let tmpY = noice.get(self.YOFF) as f32 + 1.0;
        self.X = tmpX * (SCREEN_SIZE).0 / 2.0;
        self.Y = tmpY * (SCREEN_SIZE).1 / 2.0;
        self.XOFF[0] += 0.01;
        self.YOFF[1] += 0.01;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            ggez::mint::Point2 {
                x: self.X,
                y: self.Y,
            },
            100.0,
            0.1,
            graphics::Color::BLACK,
        )?;

        canvas.draw(&circle, graphics::DrawParam::default());
        canvas.finish(ctx)?;
        Ok(())
    }
}
