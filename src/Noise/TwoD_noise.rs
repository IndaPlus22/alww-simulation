use crate::{GRID_CELL_SIZE, GRID_HEIGHT, GRID_WIDTH, SCREEN_SIZE};
use ggez::{
    graphics::{self, Color},
    mint, Context, GameResult,
};
use noise::{NoiseFn, Perlin};

pub struct TwoD_Noise {
    XOFF: f64,
    YOFF: f64,
    counter: u8,
}

impl TwoD_Noise {
    pub fn new() -> Self {
        let xoff = 0.0;
        let yoff = 0.0;
        let counter = 0;

        Self {
            XOFF: xoff,
            YOFF: yoff,
            counter: counter,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, noice: &Perlin) -> GameResult {
        if self.counter < 1 {
            let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
            let mut c = 0;
            for x in 0..(SCREEN_SIZE.0 as usize) {
                self.YOFF = 0.0;
                for y in 0..(SCREEN_SIZE.1 as usize) {
                    let _color = ((noice.get([self.XOFF, self.YOFF]) + 1.0) / 2.0) as f32;
                    let color = Color::new(_color, _color, _color, 1.0);
                    let rect = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(x as f32, y as f32, 10.0, 10.0),
                        color,
                    )?;
                    canvas.draw(&rect, graphics::DrawParam::default());
                    self.YOFF += 0.01;
                }
                self.XOFF += 0.01;
                // println!("{}", c);
                c += 1;
            }

            canvas.finish(ctx)?;
            self.counter += 1;
        }
        Ok(())
    }
}
