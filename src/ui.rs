use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Text, DrawMode, Rect, MeshBuilder};
use ggez::graphics::TextFragment;
use std::time::Instant;

use crate::game_state::{MainState, ROUND_DURATION, WINDOW_WIDTH, WINDOW_HEIGHT, MAX_ROUNDS};

// Modern color palette
const COLOR_BACKGROUND: Color = Color::new(0.95, 0.97, 1.0, 1.0);  // Light blue-gray
const COLOR_PRIMARY: Color = Color::new(0.2, 0.4, 0.8, 1.0);       // Royal blue
const COLOR_SECONDARY: Color = Color::new(0.9, 0.4, 0.3, 1.0);     // Coral
const COLOR_ACCENT: Color = Color::new(0.3, 0.7, 0.4, 1.0);        // Forest green
const COLOR_DISABLED: Color = Color::new(0.7, 0.7, 0.75, 1.0);     // Slate gray
const COLOR_TEXT: Color = Color::new(0.2, 0.2, 0.25, 1.0);         // Dark slate
const COLOR_TEXT_LIGHT: Color = Color::new(1.0, 1.0, 1.0, 1.0);    // White
const COLOR_PANEL: Color = Color::new(1.0, 1.0, 1.0, 0.9);         // Slightly transparent white
const COLOR_GOLD: Color = Color::new(0.85, 0.65, 0.2, 1.0);        // Gold

// Helper function to create modern looking panels
fn draw_panel(
    ctx: &mut Context,
    rect: Rect,
    color: Color,
    shadow_size: f32,
) -> GameResult {
    // Draw shadow first
    if shadow_size > 0.0 {
        let shadow_rect = Rect::new(
            rect.x + shadow_size,
            rect.y + shadow_size,
            rect.w,
            rect.h,
        );
        
        let shadow = MeshBuilder::new()
            .rounded_rectangle(
                DrawMode::fill(),
                shadow_rect,
                8.0, // Corner radius
                Color::new(0.0, 0.0, 0.0, 0.2), // Semi-transparent black shadow
            )?
            .build(ctx)?;
        
        graphics::draw(ctx, &shadow, DrawParam::default())?;
    }
    
    // Draw main panel
    let panel = MeshBuilder::new()
        .rounded_rectangle(
            DrawMode::fill(),
            rect,
            8.0, // Corner radius
            color,
        )?
        .build(ctx)?;
    
    graphics::draw(ctx, &panel, DrawParam::default())?;
    
    // Add subtle highlight at top
    let highlight_rect = Rect::new(rect.x, rect.y, rect.w, 2.0);
    let highlight = MeshBuilder::new()
        .rounded_rectangle(
            DrawMode::fill(),
            highlight_rect,
            1.0,
            Color::new(1.0, 1.0, 1.0, 0.4), // Semi-transparent white
        )?
        .build(ctx)?;
    
    graphics::draw(ctx, &highlight, DrawParam::default())?;
    
    Ok(())
}

// Helper function to create a beautiful gradient button
fn draw_button(
    ctx: &mut Context,
    rect: Rect,
    color: Color,
    hover: bool,
) -> GameResult {
    // Create a shadow for the button
    let shadow_rect = Rect::new(
        rect.x + 2.0,
        rect.y + 2.0,
        rect.w,
        rect.h,
    );
    
    let shadow = MeshBuilder::new()
        .rounded_rectangle(
            DrawMode::fill(),
            shadow_rect,
            8.0, // Corner radius
            Color::new(0.0, 0.0, 0.0, 0.2), // Semi-transparent black shadow
        )?
        .build(ctx)?;
    
    graphics::draw(ctx, &shadow, DrawParam::default())?;
    
    // Button base
    let button_base = MeshBuilder::new()
        .rounded_rectangle(
            DrawMode::fill(),
            rect,
            8.0, // Corner radius
            color,
        )?
        .build(ctx)?;
    
    graphics::draw(ctx, &button_base, DrawParam::default())?;
    
    // Add highlight to make it look 3D
    let highlight_rect = Rect::new(rect.x, rect.y, rect.w, rect.h / 2.0);
    let highlight = MeshBuilder::new()
        .rounded_rectangle(
            DrawMode::fill(),
            highlight_rect,
            8.0,
            Color::new(1.0, 1.0, 1.0, if hover { 0.3 } else { 0.2 }), // Brighter highlight when "hovered"
        )?
        .build(ctx)?;
    
    graphics::draw(ctx, &highlight, DrawParam::default())?;
    
    Ok(())
}

// Helper function to create buttons with text
fn draw_button_with_text(
    ctx: &mut Context,
    rect: Rect,
    color: Color,
    text: &str,
    text_size: f32,
    hover: bool,
) -> GameResult {
    // Draw the button
    draw_button(ctx, rect, color, hover)?;
    
    // Draw text
    let text_color = if color.r + color.g + color.b > 1.8 {
        COLOR_TEXT // Dark text for light buttons
    } else {
        COLOR_TEXT_LIGHT // Light text for dark buttons
    };
    
    // Create text with proper scaling
    let button_text = Text::new(
        TextFragment::new(text)
            .scale(text_size)
            .color(text_color)
    );
    
    // Center text in button both horizontally and vertically
    let text_width = text.len() as f32 * (text_size * 0.5);
    let text_x = rect.x + (rect.w - text_width) / 2.0;
    let text_y = rect.y + (rect.h - text_size) / 2.0 - 2.0; // Slight adjustment for visual centering
    
    graphics::draw(
        ctx,
        &button_text,
        DrawParam::default().dest([text_x, text_y]),
    )?;
    
    Ok(())
}

// Function to create a better looking header text
fn draw_header_text(
    ctx: &mut Context,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
) -> GameResult {
    // Draw text with a subtle shadow for better visibility
    let shadow_text = Text::new(
        TextFragment::new(text)
            .scale(size)
            .color(Color::new(0.0, 0.0, 0.0, 0.3))
    );
    
    graphics::draw(
        ctx,
        &shadow_text,
        DrawParam::default().dest([x + 1.0, y + 1.0]),
    )?;
    
    let main_text = Text::new(
        TextFragment::new(text)
            .scale(size)
            .color(color)
    );
    
    graphics::draw(
        ctx,
        &main_text,
        DrawParam::default().dest([x, y]),
    )?;
    
    Ok(())
}

// Function to draw a game stat with label and value
fn draw_stat(
    ctx: &mut Context,
    label: &str,
    value: &str,
    x: f32,
    y: f32,
    value_color: Color,
) -> GameResult {
    // Label
    let label_text = Text::new(
        TextFragment::new(label)
            .scale(18.0)
            .color(COLOR_TEXT)
    );
    
    graphics::draw(
        ctx,
        &label_text,
        DrawParam::default().dest([x, y]),
    )?;
    
    // Value
    let value_text = Text::new(
        TextFragment::new(value)
            .scale(20.0)
            .color(value_color)
    );
    
    // Position value after the label
    let label_width = label.len() as f32 * 9.0; // Approximate width
    
    graphics::draw(
        ctx,
        &value_text,
        DrawParam::default().dest([x + label_width + 5.0, y - 1.0]), // Slight adjustment for alignment
    )?;
    
    Ok(())
}

// Draws a progress bar
fn draw_progress_bar(
    ctx: &mut Context,
    rect: Rect,
    progress: f32, // 0.0 to 1.0
    color: Color,
) -> GameResult {
    // Background
    let background = MeshBuilder::new()
        .rounded_rectangle(
            DrawMode::fill(),
            rect,
            4.0,
            COLOR_DISABLED,
        )?
        .build(ctx)?;
    
    graphics::draw(ctx, &background, DrawParam::default())?;
    
    // Progress
    let progress_width = rect.w * progress.min(1.0).max(0.0);
    if progress_width > 0.0 {
        let progress_rect = Rect::new(rect.x, rect.y, progress_width, rect.h);
        let progress_mesh = MeshBuilder::new()
            .rounded_rectangle(
                DrawMode::fill(),
                progress_rect,
                4.0,
                color,
            )?
            .build(ctx)?;
        
        graphics::draw(ctx, &progress_mesh, DrawParam::default())?;
    }
    
    Ok(())
}
pub fn draw_game_ui(state: &MainState, ctx: &mut Context) -> GameResult {
    // Clear with the background color
    graphics::clear(ctx, COLOR_BACKGROUND);
    
    // Calculate round timer progress
    let round_elapsed = Instant::now().duration_since(state.round_start_time);
    let time_left = if round_elapsed < ROUND_DURATION {
        ROUND_DURATION - round_elapsed
    } else {
        std::time::Duration::from_secs(0)
    };
    let timer_progress = 1.0 - (time_left.as_secs_f32() / ROUND_DURATION.as_secs_f32());

    // Top header panel
    let header_rect = Rect::new(10.0, 10.0, WINDOW_WIDTH - 20.0, 60.0);
    draw_panel(ctx, header_rect, COLOR_PANEL, 3.0)?;
    
    // Draw round info
    draw_header_text(
        ctx,
        &format!("Round {}/{}", state.current_round, MAX_ROUNDS),
        30.0,
        25.0,
        24.0,
        COLOR_PRIMARY
    )?;
    
    // Draw timer
    let timer_rect = Rect::new(200.0, 30.0, 300.0, 20.0);
    draw_progress_bar(ctx, timer_rect, timer_progress, COLOR_SECONDARY)?;
    
    // Draw time text
    let time_text = Text::new(
        TextFragment::new(format!("{}s", time_left.as_secs()))
            .scale(18.0)
            .color(COLOR_TEXT)
    );
    
    graphics::draw(
        ctx,
        &time_text,
        DrawParam::default().dest([510.0, 28.0]),
    )?;
    
    // Player stats panel
    let stats_rect = Rect::new(10.0, 80.0, 240.0, 90.0);
    draw_panel(ctx, stats_rect, COLOR_PANEL, 3.0)?;
    
    // Draw gold
    draw_stat(
        ctx,
        "Gold: ",
        &format!("{:.0}", state.player.gold),
        30.0,
        95.0,
        COLOR_GOLD
    )?;
    
    // Draw health
    let health_color = if state.player.health <= 3 {
        COLOR_SECONDARY
    } else if state.player.health <= 6 {
        Color::new(0.9, 0.6, 0.1, 1.0) // Orange
    } else {
        COLOR_ACCENT
    };
    
    draw_stat(
        ctx,
        "Health: ",
        &state.player.health.to_string(),
        30.0,
        130.0,
        health_color
    )?;

    // Draw upgrade options
    draw_upgrade_options(state, ctx)?;
    
    draw_game_activity_log(state, ctx)?;

    // Draw bot info
    draw_bot_info(state, ctx)?;

    // Draw contribute gold option
    draw_contribute_option(state, ctx)?;

    Ok(())
}

fn draw_game_activity_log(state: &MainState, ctx: &mut Context) -> GameResult {
    // Center panel for game activity
    let log_rect = Rect::new(260.0, 80.0, WINDOW_WIDTH - 530.0, 240.0);
    draw_panel(ctx, log_rect, COLOR_PANEL, 3.0)?;
    
    // Panel header
    draw_header_text(
        ctx,
        "Game Activity",
        280.0,
        90.0,
        22.0,
        COLOR_PRIMARY
    )?;
    
    // Draw a separator line
    let line_rect = Rect::new(
        log_rect.x + 20.0,
        log_rect.y + 40.0,
        log_rect.w - 40.0,
        2.0
    );
    
    let line = MeshBuilder::new()
        .rectangle(
            DrawMode::fill(),
            line_rect,
            Color::new(0.8, 0.8, 0.8, 0.8)
        )?
        .build(ctx)?;
    
    graphics::draw(ctx, &line, DrawParam::default())?;
    
    // Generate some sample activity entries
    // In a real implementation, you would track this in the game state
    let activities = [
        ("You upgraded your Pickaxe to Lv1.", COLOR_TEXT),
        ("Bot #3 contributed 58g of gold.", COLOR_TEXT),
        ("Bot #1 upgraded their Mine to Lv1.", COLOR_TEXT),
        ("You contributed 10g of gold.", COLOR_ACCENT),
        ("Round 3 ended - you ranked #2!", COLOR_PRIMARY),
    ];
    
    let mut y_offset = log_rect.y + 60.0;
    
    for (i, (message, color)) in activities.iter().enumerate() {
        // Row background - alternating colors
        let row_rect = Rect::new(
            log_rect.x + 10.0,
            y_offset - 5.0,
            log_rect.w - 20.0,
            30.0
        );
        
        let row_color = if i % 2 == 0 {
            Color::new(0.95, 0.95, 0.95, 0.7) // Slightly darker for even rows
        } else {
            Color::new(1.0, 1.0, 1.0, 0.5) // Slightly lighter for odd rows
        };
        
        let row = MeshBuilder::new()
            .rounded_rectangle(
                DrawMode::fill(),
                row_rect,
                4.0,
                row_color
            )?
            .build(ctx)?;
        
        graphics::draw(ctx, &row, DrawParam::default())?;
        
        // Activity text
        let activity_text = Text::new(
            TextFragment::new(*message)
                .scale(16.0)
                .color(*color)
        );
        
        graphics::draw(
            ctx,
            &activity_text,
            DrawParam::default().dest([log_rect.x + 20.0, y_offset]),
        )?;
        
        y_offset += 35.0;
    }
    
    Ok(())
}

fn draw_upgrade_options(state: &MainState, ctx: &mut Context) -> GameResult {
    // Upgrades panel
    let upgrades_rect = Rect::new(10.0, 180.0, 240.0, 140.0);
    draw_panel(ctx, upgrades_rect, COLOR_PANEL, 3.0)?;
    
    // Panel header
    draw_header_text(
        ctx,
        "Upgrades",
        30.0,
        190.0,
        22.0,
        COLOR_PRIMARY
    )?;
    
    // Pickaxe upgrade button
    let mut pickaxe_color = COLOR_SECONDARY;
    let pickaxe_hover = false; // In a real game, check if mouse is over button
    
    if state.player.pickaxe_level < 4 && state.player.gold >= state.player.pickaxe_upgrade_cost() {
        pickaxe_color = COLOR_ACCENT;
    } else if state.player.pickaxe_level >= 4 {
        pickaxe_color = COLOR_DISABLED;
    }
    
    let pickaxe_rect = Rect::new(30.0, 220.0, 200.0, 40.0);
    draw_button(ctx, pickaxe_rect, pickaxe_color, pickaxe_hover)?;
    
    // Pickaxe icon (simplified)
    let pick_handle = Rect::new(45.0, 230.0, 15.0, 20.0);
    let pick_handle_mesh = MeshBuilder::new()
        .rectangle(
            DrawMode::fill(),
            pick_handle,
            Color::new(0.6, 0.4, 0.2, 1.0) // Brown
        )?
        .build(ctx)?;
    
    graphics::draw(ctx, &pick_handle_mesh, DrawParam::default())?;
    
    // Text
    let text_color = if pickaxe_color.r + pickaxe_color.g + pickaxe_color.b > 1.8 {
        COLOR_TEXT // Dark text for light buttons
    } else {
        COLOR_TEXT_LIGHT // Light text for dark buttons
    };
    
    let pickaxe_text = Text::new(
        TextFragment::new(format!(
            "Pickaxe Lv{}/4: {:.0}g",
            state.player.pickaxe_level,
            state.player.pickaxe_upgrade_cost()
        ))
        .scale(18.0)
        .color(text_color)
    );
    
    graphics::draw(
        ctx,
        &pickaxe_text,
        DrawParam::default().dest([70.0, 230.0]),
    )?;
    
    // Mine upgrade button
    let mut mine_color = COLOR_PRIMARY;
    let mine_hover = false; // In a real game, check if mouse is over button
    
    if state.player.mine_level < 4 && state.player.gold >= state.player.mine_upgrade_cost() {
        mine_color = COLOR_ACCENT;
    } else if state.player.mine_level >= 4 {
        mine_color = COLOR_DISABLED;
    }
    
    let mine_rect = Rect::new(30.0, 270.0, 200.0, 40.0);
    draw_button(ctx, mine_rect, mine_color, mine_hover)?;
    
    // Mine icon (simplified)
    let mine_icon = MeshBuilder::new()
        .circle(
            DrawMode::fill(),
            [45.0 + 7.5, 280.0 + 7.5],
            7.5,
            0.1,
            Color::new(0.5, 0.5, 0.5, 1.0) // Gray
        )?
        .build(ctx)?;
    
    graphics::draw(ctx, &mine_icon, DrawParam::default())?;
    
    // Text
    let text_color = if mine_color.r + mine_color.g + mine_color.b > 1.8 {
        COLOR_TEXT // Dark text for light buttons
    } else {
        COLOR_TEXT_LIGHT // Light text for dark buttons
    };
    
    let mine_text = Text::new(
        TextFragment::new(format!(
            "Mine Lv{}/4: {:.0}g",
            state.player.mine_level,
            state.player.mine_upgrade_cost()
        ))
        .scale(18.0)
        .color(text_color)
    );
    
    graphics::draw(
        ctx,
        &mine_text,
        DrawParam::default().dest([70.0, 280.0]),
    )?;

    Ok(())
}

fn draw_bot_info(state: &MainState, ctx: &mut Context) -> GameResult {
    // Opponents panel
    let opponents_rect = Rect::new(10.0, 330.0, WINDOW_WIDTH - 280.0, 260.0);
    draw_panel(ctx, opponents_rect, COLOR_PANEL, 3.0)?;
    
    // Panel header
    draw_header_text(
        ctx,
        "Opponents",
        30.0,
        340.0,
        22.0,
        COLOR_PRIMARY
    )?;
    
    let mut y_offset = 380.0;
    
    for (i, bot) in state.bots.iter().enumerate() {
        if bot.alive {
            // Background for bot row
            let row_rect = Rect::new(20.0, y_offset - 5.0, opponents_rect.w - 20.0, 40.0);
            let row_color = if i % 2 == 0 {
                Color::new(0.95, 0.95, 0.95, 0.7) // Slightly darker for even rows
            } else {
                Color::new(1.0, 1.0, 1.0, 0.5) // Slightly lighter for odd rows
            };
            
            let row = MeshBuilder::new()
                .rounded_rectangle(
                    DrawMode::fill(),
                    row_rect,
                    4.0,
                    row_color
                )?
                .build(ctx)?;
            
            graphics::draw(ctx, &row, DrawParam::default())?;
            
            // Bot name with icon
            let bot_name = Text::new(
                TextFragment::new(format!("Bot #{}", i + 1))
                    .scale(18.0)
                    .color(COLOR_PRIMARY)
            );
            
            graphics::draw(
                ctx,
                &bot_name,
                DrawParam::default().dest([30.0, y_offset]),
            )?;
            
            // Health bar
            let health_rect = Rect::new(120.0, y_offset + 5.0, 100.0, 15.0);
            let health_progress = bot.health as f32 / 10.0; // Assuming max health is 10
            
            // Health color based on remaining health
            let health_color = if bot.health <= 3 {
                COLOR_SECONDARY // Red for low health
            } else if bot.health <= 6 {
                Color::new(0.9, 0.6, 0.1, 1.0) // Orange for medium health
            } else {
                COLOR_ACCENT // Green for high health
            };
            
            draw_progress_bar(ctx, health_rect, health_progress, health_color)?;
            
            // Health text
            let health_text = Text::new(
                TextFragment::new(format!("{}", bot.health))
                    .scale(16.0)
                    .color(COLOR_TEXT)
            );
            
            graphics::draw(
                ctx,
                &health_text,
                DrawParam::default().dest([230.0, y_offset]),
            )?;
            
            // Pickaxe level icon and text
            let pickaxe_icon_rect = Rect::new(280.0, y_offset + 2.0, 10.0, 15.0);
            let pickaxe_icon_mesh = MeshBuilder::new()
                .rectangle(
                    DrawMode::fill(),
                    pickaxe_icon_rect,
                    Color::new(0.6, 0.4, 0.2, 1.0) // Brown
                )?
                .build(ctx)?;
            
            graphics::draw(ctx, &pickaxe_icon_mesh, DrawParam::default())?;
            
            let pickaxe_text = Text::new(
                TextFragment::new(format!("Lv{}", bot.pickaxe_level))
                    .scale(16.0)
                    .color(COLOR_SECONDARY)
            );
            
            graphics::draw(
                ctx,
                &pickaxe_text,
                DrawParam::default().dest([300.0, y_offset]),
            )?;
            
            // Mine level icon and text
            let mine_icon = MeshBuilder::new()
                .circle(
                    DrawMode::fill(),
                    [370.0, y_offset + 10.0],
                    5.0,
                    0.1,
                    Color::new(0.5, 0.5, 0.5, 1.0) // Gray
                )?
                .build(ctx)?;
            
            graphics::draw(ctx, &mine_icon, DrawParam::default())?;
            
            let mine_text = Text::new(
                TextFragment::new(format!("Lv{}", bot.mine_level))
                    .scale(16.0)
                    .color(COLOR_PRIMARY)
            );
            
            graphics::draw(
                ctx,
                &mine_text,
                DrawParam::default().dest([385.0, y_offset]),
            )?;
            
            y_offset += 50.0; // Increased spacing between bot rows
        }
    }
    
    Ok(())
}
fn draw_win_loss_tracker(state: &MainState, ctx: &mut Context, x: f32, y: f32) -> GameResult {
    // Section header
    draw_header_text(
        ctx,
        "Round Results",
        x,
        y,
        22.0,
        COLOR_PRIMARY
    )?;
    
    // Draw a separator line
    let line_rect = Rect::new(
        x,
        y + 30.0,
        220.0,
        2.0
    );
    
    let line = MeshBuilder::new()
        .rectangle(
            DrawMode::fill(),
            line_rect,
            Color::new(0.8, 0.8, 0.8, 0.8)
        )?
        .build(ctx)?;
    
    graphics::draw(ctx, &line, DrawParam::default())?;
    
    // Draw past round results (win/loss streak)
    let mut y_offset = y + 50.0;
    
    // We'll use the current_round to simulate some past results
    // In the real implementation, you would track this in the game state
    for round in 1..state.current_round {
        // For demo purposes, alternate wins and losses
        let win = round % 2 == 0;
        
        let result_rect = Rect::new(
            x,
            y_offset - 5.0,
            220.0,
            30.0
        );
        
        let result_color = if win {
            Color::new(0.8, 1.0, 0.8, 0.6) // Light green for win
        } else {
            Color::new(1.0, 0.8, 0.8, 0.6) // Light red for loss
        };
        
        let result_bg = MeshBuilder::new()
            .rounded_rectangle(
                DrawMode::fill(),
                result_rect,
                4.0,
                result_color
            )?
            .build(ctx)?;
        
        graphics::draw(ctx, &result_bg, DrawParam::default())?;
        
        // Round number
        let round_text = Text::new(
            TextFragment::new(format!("Round {}", round))
                .scale(16.0)
                .color(COLOR_TEXT)
        );
        
        graphics::draw(
            ctx,
            &round_text,
            DrawParam::default().dest([x + 10.0, y_offset]),
        )?;
        
        // Result text
        let result_text = Text::new(
            TextFragment::new(if win { "WIN" } else { "LOSS" })
                .scale(16.0)
                .color(if win { COLOR_ACCENT } else { COLOR_SECONDARY })
        );
        
        graphics::draw(
            ctx,
            &result_text,
            DrawParam::default().dest([x + 150.0, y_offset]),
        )?;
        
        y_offset += 35.0;
    }
    
    Ok(())
}

fn draw_contribute_option(state: &MainState, ctx: &mut Context) -> GameResult {
    // Contribution panel - extend height to match the opponents panel
    let contribute_rect = Rect::new(WINDOW_WIDTH - 260.0, 80.0, 250.0, 510.0);
    draw_panel(ctx, contribute_rect, COLOR_PANEL, 3.0)?;
    
    // Panel header
    draw_header_text(
        ctx,
        "Donate Gold",
        WINDOW_WIDTH - 240.0,
        90.0,
        22.0,
        COLOR_PRIMARY
    )?;
    
    // Donation explanation
    let explanation_text = Text::new(
        TextFragment::new("Donate gold to avoid taking damage at the end of each round.")
            .scale(16.0)
            .color(COLOR_TEXT)
    );
    
    graphics::draw(
        ctx,
        &explanation_text,
        DrawParam::default().dest([WINDOW_WIDTH - 240.0, 120.0]),
    )?;
    
    // Draw current donation
    let donated_text = Text::new(
        TextFragment::new(format!("Current donation: {:.0}g", state.player.donated_gold))
            .scale(18.0)
            .color(COLOR_GOLD)
    );
    
    graphics::draw(
        ctx,
        &donated_text,
        DrawParam::default().dest([WINDOW_WIDTH - 240.0, 150.0]),
    )?;

    // Draw contribution amount buttons
    let contribution_amounts = [10.0, 50.0, 100.0, 500.0, 1000.0];
    let mut y_offset = 190.0;
    
    // Draw numeric contribution options
    for amount in &contribution_amounts {
        let button_rect = Rect::new(WINDOW_WIDTH - 240.0, y_offset, 220.0, 30.0);
        
        let button_color = if state.player.gold >= *amount {
            COLOR_ACCENT
        } else {
            COLOR_DISABLED
        };
        
        let button_hover = false; // In a real game, check if mouse is over button
        
        // Use helper function for button with text
        draw_button_with_text(
            ctx,
            button_rect,
            button_color,
            &format!("Donate {:.0}g", amount),
            16.0,
            button_hover
        )?;
        
        y_offset += 40.0;
    }
    
    // Draw "All" option
    let all_button_rect = Rect::new(WINDOW_WIDTH - 240.0, y_offset, 220.0, 30.0);
    let all_button_color = if state.player.gold > 0.0 { 
        COLOR_GOLD
    } else { 
        COLOR_DISABLED
    };
    
    let all_button_hover = false; // In a real game, check if mouse is over button
    
    // Use helper function for button with text
    draw_button_with_text(
        ctx,
        all_button_rect,
        all_button_color,
        &format!("Donate All ({:.0}g)", state.player.gold),
        16.0,
        all_button_hover
    )?;
    
    // Add win/loss tracker section
    draw_win_loss_tracker(state, ctx, WINDOW_WIDTH - 240.0, y_offset + 80.0)?;

    Ok(())
}


pub fn draw_round_end_ui(state: &MainState, ctx: &mut Context) -> GameResult {
    // Clear with the background color
    graphics::clear(ctx, COLOR_BACKGROUND);
    
    if let Some(results) = &state.round_results {
        // Main panel
        let panel_height = (results.len() as f32 * 40.0) + 120.0;
        let panel_rect = Rect::new(
            WINDOW_WIDTH / 2.0 - 250.0,
            WINDOW_HEIGHT / 2.0 - panel_height / 2.0,
            500.0,
            panel_height
        );
        
        draw_panel(ctx, panel_rect, COLOR_PANEL, 5.0)?;
        
        // Draw round results header
        draw_header_text(
            ctx,
            &format!("Round {} Results", state.current_round),
            WINDOW_WIDTH / 2.0 - 120.0,
            panel_rect.y + 20.0,
            28.0,
            COLOR_PRIMARY
        )?;
        
        let mut y_offset = panel_rect.y + 70.0;
        
        // Table headers
        let headers = [
            ("Rank", 50.0, COLOR_TEXT),
            ("Player", 150.0, COLOR_TEXT),
            ("Donated", 150.0, COLOR_GOLD),
            ("Damage", 120.0, COLOR_SECONDARY)
        ];
        
        let mut x_offset = panel_rect.x + 20.0;
        
        for (header, width, color) in headers {
            let header_text = Text::new(
                TextFragment::new(header)
                    .scale(18.0)
                    .color(color)
            );
            
            graphics::draw(
                ctx,
                &header_text,
                DrawParam::default().dest([x_offset, y_offset]),
            )?;
            
            x_offset += width;
        }
        
        y_offset += 30.0;
        
        // Draw results rows
        for (position, (miner_index, donated_gold)) in results.iter().enumerate() {
            // Row background - alternating colors
            let row_rect = Rect::new(
                panel_rect.x + 10.0,
                y_offset - 5.0,
                panel_rect.w - 20.0,
                30.0
            );
            
            let row_color = if position % 2 == 0 {
                Color::new(0.95, 0.95, 0.95, 0.7) // Slightly darker for even rows
            } else {
                Color::new(1.0, 1.0, 1.0, 0.5) // Slightly lighter for odd rows
            };
            
            let row = MeshBuilder::new()
                .rounded_rectangle(
                    DrawMode::fill(),
                    row_rect,
                    4.0,
                    row_color
                )?
                .build(ctx)?;
            
            graphics::draw(ctx, &row, DrawParam::default())?;
            
            // Position/rank
            let position_color = match position {
                0 => Color::new(0.9, 0.8, 0.0, 1.0), // Gold
                1 => Color::new(0.8, 0.8, 0.8, 1.0), // Silver
                2 => Color::new(0.8, 0.5, 0.2, 1.0), // Bronze
                _ => COLOR_TEXT,                      // Default
            };
            
            let position_text = Text::new(
                TextFragment::new(format!("#{}", position + 1))
                    .scale(18.0)
                    .color(position_color)
            );
            
            graphics::draw(
                ctx,
                &position_text,
                DrawParam::default().dest([panel_rect.x + 25.0, y_offset]),
            )?;
            
            // Player name
            let miner_name = if *miner_index == 0 {
                "You (Player)".to_string()
            } else {
                format!("Bot #{}", miner_index)
            };
            
            let name_text = Text::new(
                TextFragment::new(miner_name)
                    .scale(18.0)
                    .color(COLOR_TEXT)
            );
            
            graphics::draw(
                ctx,
                &name_text,
                DrawParam::default().dest([panel_rect.x + 70.0, y_offset]),
            )?;
            
            // Donated gold
            let gold_text = Text::new(
                TextFragment::new(format!("{:.0}g", donated_gold))
                    .scale(18.0)
                    .color(COLOR_GOLD)
            );
            
            graphics::draw(
                ctx,
                &gold_text,
                DrawParam::default().dest([panel_rect.x + 220.0, y_offset]),
            )?;
            
            // Damage taken
            let damage = position as i32;
            
            let damage_text = Text::new(
                TextFragment::new(format!("-{}", damage))
                    .scale(18.0)
                    .color(COLOR_SECONDARY)
            );
            
            graphics::draw(
                ctx,
                &damage_text,
                DrawParam::default().dest([panel_rect.x + 370.0, y_offset]),
            )?;
            
            y_offset += 40.0; // Increased spacing between rows
        }
        
        // Draw continue button
        let button_rect = Rect::new(
            WINDOW_WIDTH / 2.0 - 100.0,
            panel_rect.y + panel_height - 50.0,
            200.0,
            40.0
        );
        
        draw_button_with_text(
            ctx,
            button_rect,
            COLOR_ACCENT,
            "Continue to Next Round",
            18.0,
            false // Not hovered
        )?;
    }
    
    Ok(())
}

pub fn draw_game_over_ui(state: &MainState, ctx: &mut Context) -> GameResult {
    // Clear with the background color
    graphics::clear(ctx, COLOR_BACKGROUND);
    
    // Create a fancy game over panel
    let panel_rect = Rect::new(
        WINDOW_WIDTH / 2.0 - 250.0,
        WINDOW_HEIGHT / 2.0 - 200.0, // Make panel taller
        500.0,
        400.0 // Increased height
    );
    
    draw_panel(ctx, panel_rect, COLOR_PANEL, 8.0)?; // Larger shadow for emphasis
    
    // Add a decorative header bar
    let header_bar_rect = Rect::new(
        panel_rect.x,
        panel_rect.y,
        panel_rect.w,
        50.0
    );
    
    let header_bar_color = if state.player.alive {
        COLOR_ACCENT // Green for victory
    } else {
        COLOR_SECONDARY // Red for defeat
    };
    
    let header_bar = MeshBuilder::new()
        .rounded_rectangle(
            DrawMode::fill(),
            header_bar_rect,
            8.0,
            header_bar_color
        )?
        .build(ctx)?;
    
    graphics::draw(ctx, &header_bar, DrawParam::default())?;
    
    // Draw game over text
    let game_over_message = if state.player.alive {
        "Game Complete - You Survived!"
    } else {
        "Game Over - You Died!"
    };
    
    draw_header_text(
        ctx,
        game_over_message,
        WINDOW_WIDTH / 2.0 - 180.0,
        panel_rect.y + 10.0,
        28.0,
        COLOR_TEXT_LIGHT
    )?;
    
    // Draw a separator line
    let line_rect = Rect::new(
        panel_rect.x + 50.0,
        panel_rect.y + 70.0,
        panel_rect.w - 100.0,
        2.0
    );
    
    let line = MeshBuilder::new()
        .rectangle(
            DrawMode::fill(),
            line_rect,
            Color::new(0.8, 0.8, 0.8, 0.8)
        )?
        .build(ctx)?;
    
    graphics::draw(ctx, &line, DrawParam::default())?;
    
    // Game stats
    let stats_text = Text::new(
        TextFragment::new(format!("Rounds Completed: {}/{}", 
            if state.player.alive { state.current_round } else { state.current_round - 1 }, 
            MAX_ROUNDS
        ))
        .scale(20.0)
        .color(COLOR_PRIMARY)
    );
    
    graphics::draw(
        ctx,
        &stats_text,
        DrawParam::default().dest([panel_rect.x + 100.0, panel_rect.y + 90.0]),
    )?;
    
    // Add player health
    let health_label = Text::new(
        TextFragment::new("Final Health: ")
            .scale(20.0)
            .color(COLOR_TEXT)
    );
    
    graphics::draw(
        ctx,
        &health_label,
        DrawParam::default().dest([panel_rect.x + 100.0, panel_rect.y + 130.0]),
    )?;
    
    let health_value = Text::new(
        TextFragment::new(format!("{}", state.player.health))
            .scale(20.0)
            .color(if state.player.health > 5 { COLOR_ACCENT } else { COLOR_SECONDARY })
    );
    
    graphics::draw(
        ctx,
        &health_value,
        DrawParam::default().dest([panel_rect.x + 260.0, panel_rect.y + 130.0]),
    )?;
    
    // Gold collected stat
    let gold_label = Text::new(
        TextFragment::new("Gold Collected: ")
            .scale(20.0)
            .color(COLOR_TEXT)
    );
    
    graphics::draw(
        ctx,
        &gold_label,
        DrawParam::default().dest([panel_rect.x + 100.0, panel_rect.y + 170.0]),
    )?;
    
    let gold_value = Text::new(
        TextFragment::new(format!("{:.0}g", state.player.gold + state.player.donated_gold))
            .scale(20.0)
            .color(COLOR_GOLD)
    );
    
    graphics::draw(
        ctx,
        &gold_value,
        DrawParam::default().dest([panel_rect.x + 260.0, panel_rect.y + 170.0]),
    )?;
    
    // Add round wins count
    let wins_count = state.past_results.iter().filter(|&&win| win).count();
    
    let wins_label = Text::new(
        TextFragment::new("Rounds Won: ")
            .scale(20.0)
            .color(COLOR_TEXT)
    );
    
    graphics::draw(
        ctx,
        &wins_label,
        DrawParam::default().dest([panel_rect.x + 100.0, panel_rect.y + 210.0]),
    )?;
    
    let wins_value = Text::new(
        TextFragment::new(format!("{}/{}", wins_count, state.past_results.len()))
            .scale(20.0)
            .color(COLOR_ACCENT)
    );
    
    graphics::draw(
        ctx,
        &wins_value,
        DrawParam::default().dest([panel_rect.x + 260.0, panel_rect.y + 210.0]),
    )?;
    
    // Add win streak info
    let mut current_streak = 0;
    let mut best_streak = 0;
    
    for &win in state.past_results.iter().rev() {
        if win {
            current_streak += 1;
            best_streak = best_streak.max(current_streak);
        } else {
            break;
        }
    }
    
    let streak_label = Text::new(
        TextFragment::new("Best Win Streak: ")
            .scale(20.0)
            .color(COLOR_TEXT)
    );
    
    graphics::draw(
        ctx,
        &streak_label,
        DrawParam::default().dest([panel_rect.x + 100.0, panel_rect.y + 250.0]),
    )?;
    
    let streak_value = Text::new(
        TextFragment::new(format!("{}", best_streak))
            .scale(20.0)
            .color(COLOR_ACCENT)
    );
    
    graphics::draw(
        ctx,
        &streak_value,
        DrawParam::default().dest([panel_rect.x + 260.0, panel_rect.y + 250.0]),
    )?;
    
    // Draw restart button
    let restart_rect = Rect::new(
        WINDOW_WIDTH / 2.0 - 75.0,
        panel_rect.y + 330.0, // Adjusted y position
        150.0,
        40.0
    );
    
    draw_button_with_text(
        ctx,
        restart_rect,
        COLOR_PRIMARY,
        "Restart Game",
        20.0,
        false // Not hovered
    )?;

    Ok(())
}
