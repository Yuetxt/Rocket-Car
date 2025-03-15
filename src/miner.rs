use ggez::Context;
use std::time::{Duration, Instant};

// Constants moved to this module
pub const STARTING_HEALTH: i32 = 10;

#[derive(Debug, Clone, Copy)]
pub enum MinerType {
    Player,
    Bot,
}

#[derive(Debug, Clone, Copy)]
pub struct Miner {
    pub miner_type: MinerType,
    pub gold: f32,
    pub donated_gold: f32,
    pub pickaxe_level: usize,
    pub mine_level: usize,
    pub last_mine_time: Instant,
    pub health: i32,
    pub alive: bool,
}

impl Miner {
    pub fn new(miner_type: MinerType) -> Self {
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

    pub fn mine_rate(&self) -> Duration {
        match self.pickaxe_level {
            0 => Duration::from_secs_f32(1.0),    // 1 sec (base)
            1 => Duration::from_secs_f32(0.75),   // 0.75 sec
            2 => Duration::from_secs_f32(0.5),    // 0.5 sec
            3 => Duration::from_secs_f32(0.25),   // 0.25 sec
            4 => Duration::from_secs_f32(0.1),    // 0.1 sec
            _ => Duration::from_secs_f32(1.0),    // Default to base in case
        }
    }

    pub fn gold_per_mine(&self) -> f32 {
        match self.mine_level {
            0 => 2.0,  // 2g (base)
            1 => 3.0,  // 3g
            2 => 5.0,  // 5g
            3 => 8.0,  // 8g
            4 => 15.0, // 15g
            _ => 2.0,  // Default to base in case
        }
    }

    pub fn pickaxe_upgrade_cost(&self) -> f32 {
        match self.pickaxe_level {
            0 => 200.0,  // Level 1: 200g
            1 => 400.0,  // Level 2: 400g
            2 => 800.0,  // Level 3: 800g
            3 => 1600.0, // Level 4: 1600g
            _ => f32::MAX, // Can't upgrade further
        }
    }

    pub fn mine_upgrade_cost(&self) -> f32 {
        match self.mine_level {
            0 => 100.0,  // Level 1: 100g
            1 => 300.0,  // Level 2: 300g
            2 => 600.0,  // Level 3: 600g
            3 => 1000.0, // Level 4: 1000g
            _ => f32::MAX, // Can't upgrade further
        }
    }

    pub fn update(&mut self, _ctx: &Context) {
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

    pub fn upgrade_pickaxe(&mut self) -> bool {
        if self.pickaxe_level >= 4 || self.gold < self.pickaxe_upgrade_cost() {
            return false;
        }

        self.gold -= self.pickaxe_upgrade_cost();
        self.pickaxe_level += 1;
        true
    }

    pub fn upgrade_mine(&mut self) -> bool {
        if self.mine_level >= 4 || self.gold < self.mine_upgrade_cost() {
            return false;
        }

        self.gold -= self.mine_upgrade_cost();
        self.mine_level += 1;
        true
    }

    pub fn contribute_gold(&mut self, amount: f32) {
        if amount <= self.gold {
            self.gold -= amount;
            self.donated_gold += amount;
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health <= 0 {
            self.alive = false;
            self.health = 0;
        }
    }
}