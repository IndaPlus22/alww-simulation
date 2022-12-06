mod TwoD_noise;
mod noise_walker;
use ggez::{
    event::{self, EventHandler},
    graphics, mint, Context, GameResult,
};
use noise::{NoiseFn, Perlin, Seedable};
use noise_walker::*;
use rand::Rng;
use TwoD_noise::TwoD_Noise;

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 9;
const GRID_CELL_SIZE: usize = 20;

const SCREEN_SIZE: (f32, f32) = (
    GRID_WIDTH as f32 * GRID_CELL_SIZE as f32,
    GRID_HEIGHT as f32 * GRID_CELL_SIZE as f32,
);

fn main() -> GameResult {
    //Inspired™ by https://github.com/ggez/ggez/blob/master/examples/04_snake.rs
    let (mut ctx, events_loop) = ggez::ContextBuilder::new("Simulation", "ALWW")
        .window_setup(ggez::conf::WindowSetup::default().title("Simulation"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let app = AppState::new();
    // Run!
    event::run(ctx, events_loop, app);
}

struct AppState {
    // Your state here...
    fps: u32,
    noise_walker: Noise_walker,
    TwoD: TwoD_Noise,
    noice: Perlin,
}

impl AppState {
    /// Initialize new shuffled grid.
    pub fn new() -> AppState {
        let _fps = 60;
        let walker = Noise_walker::new();
        let twoD = TwoD_Noise::new();
        let perlin = Perlin::new(1);

        AppState {
            fps: _fps,
            noise_walker: walker,
            TwoD: twoD,
            noice: perlin,
        }
    }
}

impl EventHandler for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(self.fps) {
            self.noise_walker.step(&self.noice);
        }
        Ok(())
    }

    //Inspired™™ by https://github.com/ggez/ggez/blob/master/examples/04_snake.rs
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // self.noise_walker.draw(ctx);
        self.TwoD.draw(ctx, &self.noice);
        Ok(())
    }
}
