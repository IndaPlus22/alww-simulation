use ggez::{
    graphics::{self, DrawParam},
    mint::Point2,
    Context, GameResult,
};
use noise::{NoiseFn, Perlin};

use crate::{utilities::Vector2, SCREEN_SIZE};

pub struct Vector_OP {
    vec_pos: [[f32; 2]; 2],
    vec_pos_unit: [[f32; 2]; 2],
    magnitude: f32,
}

impl Vector_OP {
    pub fn new() -> Self {
        Self {
            vec_pos: [[0.0, 0.0]; 2],
            vec_pos_unit: [[0.0, 0.0]; 2],
            magnitude: 0.0,
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let mouse_pos = ctx.mouse.position();
        let mut mouse_vec = Vector2::new(mouse_pos.x, mouse_pos.y);
        let screen_center = Vector2::new(SCREEN_SIZE.0 / 2.0, SCREEN_SIZE.1 / 2.0);
        mouse_vec -= screen_center;
        // mouse_vec.multScalar(0.5);
        self.vec_pos = [[0.0, 0.0], mouse_vec.param()];
        self.magnitude = mouse_vec.magnitude();
        mouse_vec.normalize();
        mouse_vec.multScalar(50.0);
        self.vec_pos_unit = [[0.0, 0.0], mouse_vec.param()];
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        canvas.set_screen_coordinates(graphics::Rect {
            x: SCREEN_SIZE.0 / -2.0,
            y: SCREEN_SIZE.1 / -2.0,
            w: SCREEN_SIZE.0,
            h: SCREEN_SIZE.1,
        });
        let line = graphics::Mesh::new_line(ctx, &self.vec_pos, 10.0, graphics::Color::BLACK)?;
        let unit_line =
            graphics::Mesh::new_line(ctx, &self.vec_pos_unit, 10.0, graphics::Color::RED)?;
        let magnitude_bar = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: SCREEN_SIZE.0 / -2.0,
                y: SCREEN_SIZE.1 / -2.0,
                w: (self.magnitude),
                h: (10.0),
            },
            graphics::Color::CYAN,
        )?;

        canvas.draw(&line, graphics::DrawParam::default());
        canvas.draw(&unit_line, graphics::DrawParam::default());
        canvas.draw(&magnitude_bar, graphics::DrawParam::default());
        canvas.finish(ctx)?;
        Ok(())
    }
}
