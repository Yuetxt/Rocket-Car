use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::input::mouse::MouseButton;
use rand::Rng;
use std::time::{Duration, Instant};

use crate::miner::{Miner, MinerType};
use crate::ui;

// Game constants
pub const MAX_ROUNDS: usize = 15;
pub const ROUND_DURATION: Duration = Duration::from_secs(60); // 1 minute
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

pub enum GameState {
    Playing,
    RoundEnd,
    GameOver,
}

pub struct MainState {
    pub player: Miner,
    pub bots: Vec<Miner>,
    pub current_round: usize,
    pub round_start_time: Instant,
    pub game_state: GameState,
    pub round_results: Option<Vec<(usize, f32)>>, // (miner_index, donated_gold)
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let player = Miner::new(MinerType::Player);
        let mut bots = Vec::new();
        
        // Create 3 bot miners
        for _ in 0..3 {
            bots.push(Miner::new(MinerType::Bot));
        }

        Ok(MainState {
            player,
            bots,
            current_round: 1,
            round_start_time: Instant::now(),
            game_state: GameState::Playing,
            round_results: None,
        })
    }

    pub fn bot_make_decision(&mut self, bot_index: usize) {
        let bot = &mut self.bots[bot_index];
        if !bot.alive {
            return;
        }

        let mut rng = rand::thread_rng();
        let decision = rng.gen_range(0..3); // 0: Upgrade pickaxe, 1: Upgrade mine, 2: Contribute gold

        match decision {
            0 => {
                if bot.pickaxe_level < 4 && bot.gold >= bot.pickaxe_upgrade_cost() {
                    bot.upgrade_pickaxe();
                }
            },
            1 => {
                if bot.mine_level < 4 && bot.gold >= bot.mine_upgrade_cost() {
                    bot.upgrade_mine();
                }
            },
            2 => {
                // Contribute a random portion of gold
                let contribution_percentage = rng.gen_range(0.1..0.6); // 10% to 60% of current gold
                let contribution = bot.gold * contribution_percentage;
                bot.contribute_gold(contribution);
            },
            _ => {}
        }
    }

    pub fn end_round(&mut self) {
        // Collect all miners' donated gold amounts (including player)
        let mut results = Vec::new();
        
        // Add player
        results.push((0, self.player.donated_gold));
        
        // Add bots
        for (i, bot) in self.bots.iter().enumerate() {
            if bot.alive {
                results.push((i + 1, bot.donated_gold));
            }
        }
        
        // Sort by donated gold (highest first)
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Assign damage based on position
        for (position, (miner_index, _)) in results.iter().enumerate() {
            let damage = position as i32;
            
            if *miner_index == 0 {
                // Player
                self.player.take_damage(damage);
            } else {
                // Bot
                self.bots[*miner_index - 1].take_damage(damage);
            }
        }
        
        // Reset donated gold
        self.player.donated_gold = 0.0;
        for bot in &mut self.bots {
            bot.donated_gold = 0.0;
        }
        
        // Store results for display
        self.round_results = Some(results);
        
        // Check if player is dead
        if !self.player.alive {
            self.game_state = GameState::GameOver;
        } else if self.current_round >= MAX_ROUNDS {
            self.game_state = GameState::GameOver;
        } else {
            // Move to next round
            self.game_state = GameState::RoundEnd;
        }
    }

    pub fn start_next_round(&mut self) {
        self.current_round += 1;
        self.round_start_time = Instant::now();
        self.game_state = GameState::Playing;
        self.round_results = None;
    }

    pub fn restart_game(&mut self) {
        self.player = Miner::new(MinerType::Player);
        self.bots = Vec::new();
        for _ in 0..3 {
            self.bots.push(Miner::new(MinerType::Bot));
        }
        self.current_round = 1;
        self.round_start_time = Instant::now();
        self.game_state = GameState::Playing;
        self.round_results = None;
    }

    pub fn handle_game_ui_click(&mut self, x: f32, y: f32) {
        // Check pickaxe upgrade button
        if x >= 50.0 && x <= 250.0 && y >= 150.0 && y <= 200.0 {
            self.player.upgrade_pickaxe();
        }
        
        // Check mine upgrade button
        if x >= 50.0 && x <= 250.0 && y >= 220.0 && y <= 270.0 {
            self.player.upgrade_mine();
        }
        
        // Check contribute buttons
        if x >= 400.0 && x <= 550.0 {
            let contribution_amounts = [10.0, 50.0, 100.0, 500.0, 1000.0];
            
            // Check numeric contribution options
            for (i, amount) in contribution_amounts.iter().enumerate() {
                let y_pos = 180.0 + (i as f32 * 40.0);
                
                if y >= y_pos && y <= y_pos + 30.0 && *amount <= self.player.gold {
                    self.player.contribute_gold(*amount);
                    break;
                }
            }
            
            // Check "All" option
            let all_y_pos = 180.0 + (contribution_amounts.len() as f32 * 40.0);
            
            if y >= all_y_pos && y <= all_y_pos + 30.0 && self.player.gold > 0.0 {
                self.player.contribute_gold(self.player.gold);
            }
        }
    }

    pub fn handle_round_end_ui_click(&mut self, x: f32, y: f32) {
        if let Some(results) = &self.round_results {
            let mut y_offset = 150.0;
            
            // Count number of results
            y_offset += 30.0 * results.len() as f32 + 30.0;
            
            // Debugging output (useful for development)
            // println!("Click at ({}, {}), button at y: {} to {}", 
            //          x, y, y_offset + 30.0, y_offset + 70.0);
            
            // Make the continue button larger and more forgiving with a wider hit area
            // This helps fix the issue with the continue button not always responding
            let button_x_min = WINDOW_WIDTH / 2.0 - 100.0; // Wider x range
            let button_x_max = WINDOW_WIDTH / 2.0 + 100.0;
            let button_y_min = y_offset + 20.0; // Start a bit higher
            let button_y_max = y_offset + 80.0; // End a bit lower
            
            if x >= button_x_min && x <= button_x_max &&
               y >= button_y_min && y <= button_y_max {
                self.start_next_round();
            }
        }
    }

    pub fn handle_game_over_ui_click(&mut self, x: f32, y: f32) {
        // Check restart button
        if x >= WINDOW_WIDTH / 2.0 - 75.0 && x <= WINDOW_WIDTH / 2.0 + 75.0 &&
           y >= WINDOW_HEIGHT / 2.0 + 30.0 && y <= WINDOW_HEIGHT / 2.0 + 70.0 {
            self.restart_game();
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Only update player and bots when in Playing state
        // This fixes issue with gold accumulating during round end screen
        match self.game_state {
            GameState::Playing => {
                // Update player and bots
                self.player.update(ctx);
                for bot in &mut self.bots {
                    bot.update(ctx);
                }
                
                // Make random decisions for bots
                for i in 0..self.bots.len() {
                    self.bot_make_decision(i);
                }

                // Check if round is over
                let now = Instant::now();
                let round_elapsed = now.duration_since(self.round_start_time);
                if round_elapsed >= ROUND_DURATION {
                    self.end_round();
                }
            },
            GameState::RoundEnd => {
                // Wait for player to continue - no updates to miners
            },
            GameState::GameOver => {
                // Wait for player to restart - no updates to miners
            },
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        use ggez::graphics::{self, Color};
        graphics::clear(ctx, Color::BLACK);

        // Draw UI based on game state
        match self.game_state {
            GameState::Playing => {
                ui::draw_game_ui(self, ctx)?;
            },
            GameState::RoundEnd => {
                ui::draw_round_end_ui(self, ctx)?;
            },
            GameState::GameOver => {
                ui::draw_game_over_ui(self, ctx)?;
            },
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == MouseButton::Left {
            match self.game_state {
                GameState::Playing => {
                    // Handle UI clicks during gameplay
                    self.handle_game_ui_click(x, y);
                },
                GameState::RoundEnd => {
                    // Handle round end UI clicks
                    self.handle_round_end_ui_click(x, y);
                },
                GameState::GameOver => {
                    // Handle game over UI clicks
                    self.handle_game_over_ui_click(x, y);
                },
            }
        }
    }
}