use ggez::event;
use ggez::{ContextBuilder, GameResult};
use ggez_mario;
use ggez_mario::game::*;
use ggez_mario::*;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Mario", "siabard");
    let (ctx, event_loop) = &mut cb
        .add_resource_path("./resources")
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .window_setup(ggez::conf::WindowSetup::default().title("Mario CS50 GGEZ porting"))
        .build()?;

    ggez::graphics::set_default_filter(ctx, ggez::graphics::FilterMode::Linear);
    let state = &mut ggez_mario::game::Game::new(ctx)?;

    event::run(ctx, event_loop, state)
}
