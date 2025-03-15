use ggez::{ContextBuilder, GameResult};
use ggez::event;
use ggez::conf::{WindowSetup, WindowMode};

mod miner;
mod game_state;
mod ui;

use game_state::MainState;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("placeholder_title", "Daniel Zheng")
        .window_setup(WindowSetup::default().title("Placeholder Title"))
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}