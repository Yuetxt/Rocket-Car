use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Text, Mesh, DrawMode, Rect};
use std::time::Instant;

use crate::game_state::{MainState, ROUND_DURATION, WINDOW_WIDTH, WINDOW_HEIGHT, MAX_ROUNDS};

pub fn draw_game_ui(state: &MainState, ctx: &mut Context) -> GameResult {
    let round_elapsed = Instant::now().duration_since(state.round_start_time);
    let time_left = if round_elapsed < ROUND_DURATION {
        ROUND_DURATION - round_elapsed
    } else {
        std::time::Duration::from_secs(0)
    };

    // Draw round and time info
    let round_text = Text::new(format!("Round: {}/{}", state.current_round, MAX_ROUNDS));
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
    let player_gold_text = Text::new(format!("Gold: {:.0}", state.player.gold));
    let player_health_text = Text::new(format!("Health: {}", state.player.health));
    
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
    draw_upgrade_options(state, ctx)?;

    // Draw bot info (only showing their upgrades as per requirements)
    draw_bot_info(state, ctx)?;

    // Draw contribute gold option
    draw_contribute_option(state, ctx)?;

    Ok(())
}

fn draw_upgrade_options(state: &MainState, ctx: &mut Context) -> GameResult {
    // Pickaxe upgrade button
    let mut pickaxe_color = Color::RED;
    if state.player.pickaxe_level < 4 && state.player.gold >= state.player.pickaxe_upgrade_cost() {
        pickaxe_color = Color::GREEN;
    }
    
    let pickaxe_rect = Rect::new(50.0, 150.0, 200.0, 50.0);
    let pickaxe_mesh = Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        pickaxe_rect,
        pickaxe_color,
    )?;
    
    graphics::draw(ctx, &pickaxe_mesh, DrawParam::default())?;
    
    let pickaxe_text = Text::new(format!(
        "Upgrade Pickaxe: {:.0}g\nLevel: {}/4",
        state.player.pickaxe_upgrade_cost(),
        state.player.pickaxe_level,
    ));
    
    graphics::draw(
        ctx,
        &pickaxe_text,
        DrawParam::default().dest([60.0, 160.0]),
    )?;

    // Mine upgrade button
    let mut mine_color = Color::BLUE;
    if state.player.mine_level < 4 && state.player.gold >= state.player.mine_upgrade_cost() {
        mine_color = Color::GREEN;
    }
    
    let mine_rect = Rect::new(50.0, 220.0, 200.0, 50.0);
    let mine_mesh = Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        mine_rect,
        mine_color,
    )?;
    
    graphics::draw(ctx, &mine_mesh, DrawParam::default())?;
    
    let mine_text = Text::new(format!(
        "Upgrade Mine: {:.0}g\nLevel: {}/4",
        state.player.mine_upgrade_cost(),
        state.player.mine_level,
    ));
    
    graphics::draw(
        ctx,
        &mine_text,
        DrawParam::default().dest([60.0, 230.0]),
    )?;

    Ok(())
}

fn draw_bot_info(state: &MainState, ctx: &mut Context) -> GameResult {
    let mut y_offset = 300.0;
    
    for (i, bot) in state.bots.iter().enumerate() {
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

fn draw_contribute_option(state: &MainState, ctx: &mut Context) -> GameResult {
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
        let button_rect = Rect::new(400.0, y_offset, 150.0, 30.0);
        
        let mut button_color = Color {
            r: 128.0,
            g: 128.0,
            b: 128.0,
            a: 0.0,
        };
        if state.player.gold >= *amount {
            button_color = Color::GREEN;
        }
        
        let button_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
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
    let all_button_rect = Rect::new(400.0, y_offset, 150.0, 30.0);
    let all_button_color = if state.player.gold > 0.0 { Color::GREEN } else { Color::BLACK };
    
    let all_button_mesh = Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        all_button_rect,
        all_button_color,
    )?;
    
    graphics::draw(ctx, &all_button_mesh, DrawParam::default())?;
    
    let all_text = Text::new(format!("All ({:.0}g)", state.player.gold));
    
    graphics::draw(
        ctx,
        &all_text,
        DrawParam::default().dest([410.0, y_offset + 5.0]),
    )?;

    Ok(())
}

pub fn draw_round_end_ui(state: &MainState, ctx: &mut Context) -> GameResult {
    if let Some(results) = &state.round_results {
        // Draw round results
        let round_text = Text::new(format!("Round {} Results", state.current_round));
        
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

        // Draw continue button - make it larger and more visible
        let continue_rect = Rect::new(
            WINDOW_WIDTH / 2.0 - 100.0, // Wider button
            y_offset + 30.0,
            200.0, // Wider button
            50.0,  // Taller button
        );
        
        let continue_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            continue_rect,
            Color::GREEN,
        )?;
        
        graphics::draw(ctx, &continue_mesh, DrawParam::default())?;
        
        // Make the button text bigger and more centered
        let continue_text = Text::new("Continue");
        
        graphics::draw(
            ctx,
            &continue_text,
            DrawParam::default().dest([WINDOW_WIDTH / 2.0 - 80.0, y_offset + 45.0]),
        )?;
    }

    Ok(())
}

pub fn draw_game_over_ui(state: &MainState, ctx: &mut Context) -> GameResult {
    let game_over_text = Text::new(if state.player.alive {
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
    let restart_rect = Rect::new(
        WINDOW_WIDTH / 2.0 - 75.0,
        WINDOW_HEIGHT / 2.0 + 30.0,
        150.0,
        40.0,
    );
    
    let restart_mesh = Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
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