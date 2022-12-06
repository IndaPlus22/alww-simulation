use crate::{utilities::Vector2, SCREEN_SIZE};
use ggez::{graphics, Context, GameResult};
pub struct Ball {
    pos: Vector2<f32>,
    speed: Vector2<f32>,
}

impl Ball {
    pub fn new() -> Self {
        let _pos = Vector2::new(100.0, 100.0);
        let _speed = Vector2::new(10.0, 12.0);

        Self {
            pos: _pos,
            speed: _speed,
        }
    }

    pub fn calc(&mut self) {
        self.pos = self.pos + self.speed;

        if (self.pos.x > SCREEN_SIZE.0 || self.pos.x < 0.0) {
            self.speed.x = self.speed.x * -1.0;
        }
        if (self.pos.y > SCREEN_SIZE.1 || self.pos.y < 0.0) {
            self.speed.y = self.speed.y * -1.0;
        }
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
