use ggez::{
    event::{self},
    GameResult,
};

fn main() -> GameResult {
    //Inspired™ by https://github.com/ggez/ggez/blob/master/examples/04_snake.rs
    let (mut ctx, events_loop) = ggez::ContextBuilder::new("Simulation", "ALWW")
        .window_setup(ggez::conf::WindowSetup::default().title("Simulation"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            alww_simulation::SCREEN_SIZE.0,
            alww_simulation::SCREEN_SIZE.1,
        ))
        .build()?;

    let app = alww_simulation::AppState::new();
    // Run!
    event::run(ctx, events_loop, app);
}
