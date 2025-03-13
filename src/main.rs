use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, DrawParam, Text};
use ggez::event::{self, EventHandler};
use ggez::input::mouse::MouseButton;
use ggez::conf::{WindowSetup, WindowMode};
use rand::Rng;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const MAX_ROUNDS: usize = 15;
const ROUND_DURATION: Duration = Duration::from_secs(60); // 1 minute
const STARTING_HEALTH: i32 = 10;

#[derive(Debug, Clone, Copy)]
enum MinerType {
    Player,
    Bot,
}

#[derive(Debug, Clone, Copy)]
struct Miner {
    miner_type: MinerType,
    gold: f32,
    donated_gold: f32,
    pickaxe_level: usize,
    mine_level: usize,
    last_mine_time: Instant,
    health: i32,
    alive: bool,
}

impl Miner {
    fn new(miner_type: MinerType) -> Self {
        Miner {
            miner_type,
            gold: 0.0,
            donated_gold: 0.0,
            pickaxe_level: 0,
            mine_level: 0,
            last_mine_time: Instant::now(),
            health: STARTING_HEALTH,
            alive: true,
        }
    }

    fn mine_rate(&self) -> Duration {
        match self.pickaxe_level {
            0 => Duration::from_secs_f32(1.0),    // 1 sec (base)
            1 => Duration::from_secs_f32(0.75),   // 0.75 sec
            2 => Duration::from_secs_f32(0.5),    // 0.5 sec
            3 => Duration::from_secs_f32(0.25),   // 0.25 sec
            4 => Duration::from_secs_f32(0.1),    // 0.1 sec
            _ => Duration::from_secs_f32(1.0),    // Default to base in case
        }
    }

    fn gold_per_mine(&self) -> f32 {
        match self.mine_level {
            0 => 2.0,  // 2g (base)
            1 => 3.0,  // 3g
            2 => 5.0,  // 5g
            3 => 8.0,  // 8g
            4 => 15.0, // 15g
            _ => 2.0,  // Default to base in case
        }
    }

    fn pickaxe_upgrade_cost(&self) -> f32 {
        match self.pickaxe_level {
            0 => 200.0,  // Level 1: 200g
            1 => 400.0,  // Level 2: 400g
            2 => 800.0,  // Level 3: 800g
            3 => 1600.0, // Level 4: 1600g
            _ => f32::MAX, // Can't upgrade further
        }
    }

    fn mine_upgrade_cost(&self) -> f32 {
        match self.mine_level {
            0 => 100.0,  // Level 1: 100g
            1 => 300.0,  // Level 2: 300g
            2 => 600.0,  // Level 3: 600g
            3 => 1000.0, // Level 4: 1000g
            _ => f32::MAX, // Can't upgrade further
        }
    }

    fn update(&mut self, ctx: &Context) {
        if !self.alive {
            return;
        }

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_mine_time);
        
        if elapsed >= self.mine_rate() {
            // Mine gold
            self.gold += self.gold_per_mine();
            self.last_mine_time = now;
        }
    }

    fn upgrade_pickaxe(&mut self) -> bool {
        if self.pickaxe_level >= 4 || self.gold < self.pickaxe_upgrade_cost() {
            return false;
        }

        self.gold -= self.pickaxe_upgrade_cost();
        self.pickaxe_level += 1;
        true
    }

    fn upgrade_mine(&mut self) -> bool {
        if self.mine_level >= 4 || self.gold < self.mine_upgrade_cost() {
            return false;
        }

        self.gold -= self.mine_upgrade_cost();
        self.mine_level += 1;
        true
    }

    fn contribute_gold(&mut self, amount: f32) {
        if amount <= self.gold {
            self.gold -= amount;
            self.donated_gold += amount;
        }
    }

    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health <= 0 {
            self.alive = false;
            self.health = 0;
        }
    }
}

enum GameState {
    Playing,
    RoundEnd,
    GameOver,
}

struct MainState {
    player: Miner,
    bots: Vec<Miner>,
    current_round: usize,
    round_start_time: Instant,
    game_state: GameState,
    round_results: Option<Vec<(usize, f32)>>, // (miner_index, donated_gold)
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
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

    fn bot_make_decision(&mut self, bot_index: usize) {
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

    fn end_round(&mut self) {
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

    fn start_next_round(&mut self) {
        self.current_round += 1;
        self.round_start_time = Instant::now();
        self.game_state = GameState::Playing;
        self.round_results = None;
    }

    fn restart_game(&mut self) {
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

    fn draw_game_ui(&self, ctx: &mut Context) -> GameResult {
        let round_elapsed = Instant::now().duration_since(self.round_start_time);
        let time_left = if round_elapsed < ROUND_DURATION {
            ROUND_DURATION - round_elapsed
        } else {
            Duration::from_secs(0)
        };

        // Draw round and time info
        let round_text = Text::new(format!("Round: {}/{}", self.current_round, MAX_ROUNDS));
        let time_text = Text::new(format!("Time left: {}s", time_left.as_secs()));
        
        graphics::draw(
            ctx,
            &round_text,
            DrawParam::default().dest([20.0, 20.0]),
        )?;
        
        graphics::draw(
            ctx,
            &time_text,
            DrawParam::default().dest([20.0, 50.0]),
        )?;

        // Draw player info
        let player_gold_text = Text::new(format!("Gold: {:.0}", self.player.gold));
        let player_health_text = Text::new(format!("Health: {}", self.player.health));
        
        graphics::draw(
            ctx,
            &player_gold_text,
            DrawParam::default().dest([20.0, 80.0]),
        )?;
        
        graphics::draw(
            ctx,
            &player_health_text,
            DrawParam::default().dest([20.0, 110.0]),
        )?;

        // Draw upgrade options
        self.draw_upgrade_options(ctx)?;

        // Draw bot info (only showing their upgrades as per requirements)
        self.draw_bot_info(ctx)?;

        // Draw contribute gold option
        self.draw_contribute_option(ctx)?;

        Ok(())
    }

    fn draw_upgrade_options(&self, ctx: &mut Context) -> GameResult {
        // Pickaxe upgrade button
        let mut pickaxe_color = Color::RED;
        if self.player.pickaxe_level < 4 && self.player.gold >= self.player.pickaxe_upgrade_cost() {
            pickaxe_color = Color::GREEN;
        }
        
        let pickaxe_rect = graphics::Rect::new(50.0, 150.0, 200.0, 50.0);
        let pickaxe_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            pickaxe_rect,
            pickaxe_color,
        )?;
        
        graphics::draw(ctx, &pickaxe_mesh, DrawParam::default())?;
        
        let pickaxe_text = Text::new(format!(
            "Upgrade Pickaxe: {:.0}g\nLevel: {}/4",
            self.player.pickaxe_upgrade_cost(),
            self.player.pickaxe_level,
        ));
        
        graphics::draw(
            ctx,
            &pickaxe_text,
            DrawParam::default().dest([60.0, 160.0]),
        )?;

        // Mine upgrade button
        let mut mine_color = Color::BLUE;
        if self.player.mine_level < 4 && self.player.gold >= self.player.mine_upgrade_cost() {
            mine_color = Color::GREEN;
        }
        
        let mine_rect = graphics::Rect::new(50.0, 220.0, 200.0, 50.0);
        let mine_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            mine_rect,
            mine_color,
        )?;
        
        graphics::draw(ctx, &mine_mesh, DrawParam::default())?;
        
        let mine_text = Text::new(format!(
            "Upgrade Mine: {:.0}g\nLevel: {}/4",
            self.player.mine_upgrade_cost(),
            self.player.mine_level,
        ));
        
        graphics::draw(
            ctx,
            &mine_text,
            DrawParam::default().dest([60.0, 230.0]),
        )?;

        Ok(())
    }

    fn draw_bot_info(&self, ctx: &mut Context) -> GameResult {
        let mut y_offset = 300.0;
        
        for (i, bot) in self.bots.iter().enumerate() {
            if bot.alive {
                let bot_text = Text::new(format!(
                    "Bot #{}: Pickaxe Lv{}, Mine Lv{}, Health: {}",
                    i + 1,
                    bot.pickaxe_level,
                    bot.mine_level,
                    bot.health,
                ));
                
                graphics::draw(
                    ctx,
                    &bot_text,
                    DrawParam::default().dest([50.0, y_offset]),
                )?;
                
                y_offset += 30.0;
            }
        }
        
        Ok(())
    }

    fn draw_contribute_option(&self, ctx: &mut Context) -> GameResult {
        // Contribution options
        let contribute_text = Text::new("Contribute Gold:");
        
        graphics::draw(
            ctx,
            &contribute_text,
            DrawParam::default().dest([400.0, 150.0]),
        )?;

        // Draw contribution amount buttons
        let contribution_amounts = [10.0, 50.0, 100.0, 500.0, 1000.0];
        let mut y_offset = 180.0;
        
        // Draw numeric contribution options
        for amount in &contribution_amounts {
            let button_text = format!("{:.0}g", amount);
            let button_rect = graphics::Rect::new(400.0, y_offset, 150.0, 30.0);
            
            let mut button_color = Color {
                r: 128.0,
                g: 128.0,
                b: 128.0,
                a: 0.0,
            };
            if self.player.gold >= *amount {
                button_color = Color::GREEN;
            }
            
            let button_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                button_rect,
                button_color,
            )?;
            
            graphics::draw(ctx, &button_mesh, DrawParam::default())?;
            
            let text = Text::new(button_text);
            
            graphics::draw(
                ctx,
                &text,
                DrawParam::default().dest([410.0, y_offset + 5.0]),
            )?;
            
            y_offset += 40.0;
        }
        
        // Draw "All" option
        let all_button_rect = graphics::Rect::new(400.0, y_offset, 150.0, 30.0);
        let all_button_color = if self.player.gold > 0.0 { Color::GREEN } else { Color::BLACK };
        
        let all_button_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            all_button_rect,
            all_button_color,
        )?;
        
        graphics::draw(ctx, &all_button_mesh, DrawParam::default())?;
        
        let all_text = Text::new(format!("All ({:.0}g)", self.player.gold));
        
        graphics::draw(
            ctx,
            &all_text,
            DrawParam::default().dest([410.0, y_offset + 5.0]),
        )?;

        Ok(())
    }

    fn draw_round_end_ui(&self, ctx: &mut Context) -> GameResult {
        if let Some(results) = &self.round_results {
            // Draw round results
            let round_text = Text::new(format!("Round {} Results", self.current_round));
            
            graphics::draw(
                ctx,
                &round_text,
                DrawParam::default().dest([WINDOW_WIDTH / 2.0 - 80.0, 100.0]),
            )?;

            let mut y_offset = 150.0;
            
            for (position, (miner_index, donated_gold)) in results.iter().enumerate() {
                let miner_name = if *miner_index == 0 {
                    "You (Player)".to_string()
                } else {
                    format!("Bot #{}", miner_index)
                };
                
                let damage = position as i32;
                
                let result_text = Text::new(format!(
                    "{}. {}: {:.0}g donated, {} damage taken",
                    position + 1,
                    miner_name,
                    donated_gold,
                    damage,
                ));
                
                graphics::draw(
                    ctx,
                    &result_text,
                    DrawParam::default().dest([WINDOW_WIDTH / 2.0 - 150.0, y_offset]),
                )?;
                
                y_offset += 30.0;
            }

            // Draw continue button
            let continue_rect = graphics::Rect::new(
                WINDOW_WIDTH / 2.0 - 75.0,
                y_offset + 30.0,
                150.0,
                40.0,
            );
            
            let continue_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                continue_rect,
                Color::GREEN,
            )?;
            
            graphics::draw(ctx, &continue_mesh, DrawParam::default())?;
            
            let continue_text = Text::new("Continue");
            
            graphics::draw(
                ctx,
                &continue_text,
                DrawParam::default().dest([WINDOW_WIDTH / 2.0 - 30.0, y_offset + 40.0]),
            )?;
        }

        Ok(())
    }

    fn draw_game_over_ui(&self, ctx: &mut Context) -> GameResult {
        let game_over_text = Text::new(if self.player.alive {
            "Game Over - You Survived!"
        } else {
            "Game Over - You Died!"
        });
        
        graphics::draw(
            ctx,
            &game_over_text,
            DrawParam::default().dest([WINDOW_WIDTH / 2.0 - 100.0, WINDOW_HEIGHT / 2.0 - 50.0]),
        )?;

        // Draw restart button
        let restart_rect = graphics::Rect::new(
            WINDOW_WIDTH / 2.0 - 75.0,
            WINDOW_HEIGHT / 2.0 + 30.0,
            150.0,
            40.0,
        );
        
        let restart_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            restart_rect,
            Color::GREEN,
        )?;
        
        graphics::draw(ctx, &restart_mesh, DrawParam::default())?;
        
        let restart_text = Text::new("Restart Game");
        
        graphics::draw(
            ctx,
            &restart_text,
            DrawParam::default().dest([WINDOW_WIDTH / 2.0 - 45.0, WINDOW_HEIGHT / 2.0 + 40.0]),
        )?;

        Ok(())
    }

    fn handle_game_ui_click(&mut self, x: f32, y: f32) {
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

    fn handle_round_end_ui_click(&mut self, x: f32, y: f32) {
        if let Some(results) = &self.round_results {
            let mut y_offset = 150.0;
            
            // Count number of results
            y_offset += 30.0 * results.len() as f32 + 30.0;
            
            // Check continue button
            if x >= WINDOW_WIDTH / 2.0 - 75.0 && x <= WINDOW_WIDTH / 2.0 + 75.0 &&
               y >= y_offset + 30.0 && y <= y_offset + 70.0 {
                self.start_next_round();
            }
        }
    }

    fn handle_game_over_ui_click(&mut self, x: f32, y: f32) {
        // Check restart button
        if x >= WINDOW_WIDTH / 2.0 - 75.0 && x <= WINDOW_WIDTH / 2.0 + 75.0 &&
           y >= WINDOW_HEIGHT / 2.0 + 30.0 && y <= WINDOW_HEIGHT / 2.0 + 70.0 {
            self.restart_game();
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update player and bots
        self.player.update(ctx);
        for bot in &mut self.bots {
            bot.update(ctx);
        }

        match self.game_state {
            GameState::Playing => {
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
                // Wait for player to continue
            },
            GameState::GameOver => {
                // Wait for player to restart
            },
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        // Draw UI based on game state
        match self.game_state {
            GameState::Playing => {
                self.draw_game_ui(ctx)?;
            },
            GameState::RoundEnd => {
                self.draw_round_end_ui(ctx)?;
            },
            GameState::GameOver => {
                self.draw_game_over_ui(ctx)?;
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

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("placeholder_title", "Daniel Zheng")
        .window_setup(WindowSetup::default().title("Placeholder Title"))
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}