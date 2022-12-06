use ggez::{
    event::{self, EventHandler},
    graphics,
    input::{self, mouse},
    mint, Context, GameResult,
};
use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;
use utilities::Vector2;
use Balls::{bounce_no_vec, bounce_vec};
use Movers::mover::{self, ManyMovers, Mover};
use Noise::{
    noise_walker::Noise_walker, noise_walker_vec::Noise_walker_vec, TwoD_noise::TwoD_Noise,
};
use Vector::vector_op::Vector_OP;

mod Balls;
mod Movers;
mod Noise;
mod Vector;
mod utilities;

pub const GRID_WIDTH: usize = 1600;
pub const GRID_HEIGHT: usize = 1200;
pub const GRID_CELL_SIZE: usize = 1;

pub const SCREEN_SIZE: (f32, f32) = (
    GRID_WIDTH as f32 * GRID_CELL_SIZE as f32,
    GRID_HEIGHT as f32 * GRID_CELL_SIZE as f32,
);

pub struct AppState {
    // Your state here...
    fps: u32,
    noise_walker: Noise_walker,
    noise_walker_vec: Noise_walker_vec,
    TwoD: TwoD_Noise,
    noice: Perlin,
    no_vec_ball: bounce_no_vec::Ball,
    vec_ball: bounce_vec::Ball,
    vector_OP: Vector_OP,
    mover: Mover,
    movers: ManyMovers,
}

impl AppState {
    /// Initialize new shuffled grid.
    pub fn new() -> AppState {
        let _fps = 60;
        let walker = Noise_walker::new();
        let walker_vec = Noise_walker_vec::new();
        let twoD = TwoD_Noise::new();

        let mut rng = rand::thread_rng();
        let seed = rng.gen::<u32>();
        let perlin = Perlin::new(seed);
        let no_vec_ball = bounce_no_vec::Ball::new();
        let vec_ball = bounce_vec::Ball::new();
        let vector_sub = Vector_OP::new();
        let mover = Mover::new(100.0, 100.0);
        let movers = ManyMovers::new();

        AppState {
            fps: _fps,
            noise_walker: walker,
            noise_walker_vec: walker_vec,
            TwoD: twoD,
            noice: perlin,
            no_vec_ball: no_vec_ball,
            vec_ball: vec_ball,
            vector_OP: vector_sub,
            mover: mover,
            movers: movers,
        }
    }
}

impl EventHandler for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut rng = rand::thread_rng();

        while ctx.time.check_update_time(self.fps) {
            // self.noise_walker.step(&self.noice);
            // self.noise_walker_vec.step(&self.noice);
            // self.no_vec_ball.calc();
            // self.vec_ball.calc();
            // self.vector_OP.update(ctx);
            // self.mover.check_keys(ctx);
            // self.mover.update(ctx, &mut rng, &self.noice);
            // self.mover.check_edges();

            self.movers.updateMany(ctx, &self.noice);
        }
        Ok(())
    }

    //Inspired™™ by https://github.com/ggez/ggez/blob/master/examples/04_snake.rs
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // self.noise_walker.draw(ctx);
        // self.noise_walker_vec.draw(ctx);
        // self.TwoD.draw(ctx, &self.noice);
        // self.no_vec_ball.draw(ctx);
        // self.vec_ball.draw(ctx);
        // self.vector_OP.draw(ctx);
        // self.mover.draw(ctx);
        self.movers.drawMany(ctx);
        Ok(())
    }
}
