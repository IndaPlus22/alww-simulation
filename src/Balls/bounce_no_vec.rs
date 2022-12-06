use crate::SCREEN_SIZE;
use ggez::{graphics, Context, GameResult};
pub struct Ball {
    x: f32,
    y: f32,
    xspeed: f32,
    yspeed: f32,
}

impl Ball {
    pub fn new() -> Self {
        let x = 100.0;
        let y = 100.0;
        let xspeed = 10.0;
        let yspeed = 13.0;

        Self {
            x: x,
            y: y,
            xspeed: xspeed,
            yspeed: yspeed,
        }
    }

    pub fn calc(&mut self) {
        self.x = self.x + self.xspeed;
        self.y = self.y + self.yspeed;

        if (self.x > SCREEN_SIZE.0 || self.x < 0.0) {
            self.xspeed = self.xspeed * -1.0;
        }
        if (self.y > SCREEN_SIZE.1 || self.y < 0.0) {
            self.yspeed = self.yspeed * -1.0;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            ggez::mint::Point2 {
                x: self.x,
                y: self.y,
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
