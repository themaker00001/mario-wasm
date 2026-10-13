use crate::types::*;
use crate::level::Level;
use crate::entities::*;
use crate::physics::*;

pub struct Game {
    pub mario: Mario,
    pub level: Level,
    pub enemies: Vec<Enemy>,
    pub items: Vec<Item>,
    pub particles: Vec<Particle>,
    pub block_bumps: Vec<BlockBump>,
    pub camera_x: f32,
    pub score: u32,
    pub coins: u32,
    pub lives: u32,
    pub time_remaining: u32,
    pub time_tick: u32,
    pub state: GameState,
    pub state_timer: u32,
    pub flag_y: f32,
    pub sound_events: u32,
    pub anim_tick: u32,
    pub prev_input: u32,
}

impl Game {
    pub fn new() -> Self {
        let level = Level::new_world_1_1();
        let enemies = level.initial_enemies.iter().map(|e| Enemy {
            kind: e.kind,
            x: e.x,
            y: e.y,
            vx: e.vx,
            vy: e.vy,
            alive: true,
            is_shell: false,
            shell_sliding: false,
            squished: false,
            squish_timer: 0,
            walk_frame: 0.0,
            facing_right: false,
        }).collect();

        Self {
            mario: Mario::new(40.0, 11.0 * 16.0),
            level,
            enemies,
            items: Vec::new(),
            particles: Vec::new(),
            block_bumps: Vec::new(),
            camera_x: 0.0,
            score: 0,
            coins: 0,
            lives: 3,
            time_remaining: 400,
            time_tick: 0,
            state: GameState::Title,
            state_timer: 0,
            flag_y: 4.0 * 16.0,
            sound_events: 0,
            anim_tick: 0,
            prev_input: 0,
        }
    }

    pub fn start_game(&mut self) {
        self.state = GameState::Playing;
        self.score = 0;
        self.coins = 0;
        self.lives = 3;
        self.reset_level();
    }

    pub fn reset_level(&mut self) {
        self.level = Level::new_world_1_1();
        self.enemies = self.level.initial_enemies.iter().map(|e| Enemy {
            kind: e.kind,
            x: e.x,
            y: e.y,
            vx: e.vx,
            vy: e.vy,
            alive: true,
            is_shell: false,
            shell_sliding: false,
            squished: false,
            squish_timer: 0,
            walk_frame: 0.0,
            facing_right: false,
        }).collect();
        self.items.clear();
        self.particles.clear();
        self.block_bumps.clear();
        self.mario = Mario::new(40.0, 11.0 * 16.0);
        self.camera_x = 0.0;
        self.time_remaining = 400;
        self.time_tick = 0;
        self.flag_y = 4.0 * 16.0;
        self.state_timer = 0;
    }

    pub fn update(&mut self, input: u32) {
        self.sound_events = 0;
        self.anim_tick = self.anim_tick.wrapping_add(1);

        match self.state {
            GameState::Title => {
                if (input & (INPUT_START | INPUT_JUMP)) != 0 && (self.prev_input & (INPUT_START | INPUT_JUMP)) == 0 {
                    self.start_game();
                    self.sound_events |= 1 << SOUND_COIN;
                }
            }
            GameState::GameOver => {
                self.state_timer += 1;
                if self.state_timer > 60 && (input & (INPUT_START | INPUT_JUMP)) != 0 && (self.prev_input & (INPUT_START | INPUT_JUMP)) == 0 {
                    self.start_game();
                    self.sound_events |= 1 << SOUND_COIN;
                }
            }
            GameState::StageClear => {
                self.state_timer += 1;
                if self.state_timer > 180 && (input & (INPUT_START | INPUT_JUMP)) != 0 && (self.prev_input & (INPUT_START | INPUT_JUMP)) == 0 {
                    self.reset_level();
                    self.state = GameState::Playing;
                }
            }
            GameState::Dying => {
                self.update_dying();
            }
            GameState::FlagpoleSlide => {
                self.update_flagpole_slide();
            }
            GameState::CastleWalk => {
                self.update_castle_walk();
            }
            GameState::Playing => {
                self.update_playing(input);
            }
        }

        self.prev_input = input;
    }

    fn update_playing(&mut self, input: u32) {
        // 1. Level Timer countdown
        self.time_tick += 1;
        if self.time_tick >= 40 {
            self.time_tick = 0;
            if self.time_remaining > 0 {
                self.time_remaining -= 1;
            } else {
                self.kill_mario();
                return;
            }
        }

        let is_left = (input & INPUT_LEFT) != 0;
        let is_right = (input & INPUT_RIGHT) != 0;
        let is_down = (input & INPUT_DOWN) != 0;
        let is_jump = (input & INPUT_JUMP) != 0;
        let is_run = (input & INPUT_RUN) != 0;
        let jump_pressed = is_jump && ((self.prev_input & INPUT_JUMP) == 0);

        // Mario invulnerability flashing countdown
        if self.mario.invulnerable_timer > 0 {
            self.mario.invulnerable_timer -= 1;
        }

        // Mario ducking (only for Super form)
        if is_down && self.mario.is_grounded && self.mario.form != MarioForm::Small {
            self.mario.is_ducking = true;
        } else {
            self.mario.is_ducking = false;
        }

        // Mario horizontal acceleration & speed
        let max_speed = if is_run { 3.2 } else { 1.8 };
        let accel = if self.mario.is_grounded { 0.12 } else { 0.08 };
        let friction = 0.88;

        self.mario.is_skidding = false;
        if !self.mario.is_ducking {
            if is_right {
                self.mario.facing_right = true;
                if self.mario.vx < -0.5 && self.mario.is_grounded {
                    self.mario.is_skidding = true;
                    self.mario.vx += accel * 2.2;
                } else if self.mario.vx < max_speed {
                    self.mario.vx += accel;
                }
            } else if is_left {
                self.mario.facing_right = false;
                if self.mario.vx > 0.5 && self.mario.is_grounded {
                    self.mario.is_skidding = true;
                    self.mario.vx -= accel * 2.2;
                } else if self.mario.vx > -max_speed {
                    self.mario.vx -= accel;
                }
            } else {
                if self.mario.is_grounded {
                    self.mario.vx *= friction;
                    if self.mario.vx.abs() < 0.08 {
                        self.mario.vx = 0.0;
                    }
                }
            }
        } else {
            // Sliding while ducking
            self.mario.vx *= 0.94;
            if self.mario.vx.abs() < 0.08 {
                self.mario.vx = 0.0;
            }
        }

        // Mario Jump mechanics
        if jump_pressed && self.mario.is_grounded && !self.mario.is_ducking {
            let jump_power = if is_run && self.mario.vx.abs() > 2.0 {
                -5.3
            } else {
                -4.7
            };
            self.mario.vy = jump_power;
            self.mario.is_grounded = false;
            let snd = if self.mario.form == MarioForm::Small {
                SOUND_JUMP_SMALL
            } else {
                SOUND_JUMP_SUPER
            };
            self.sound_events |= 1 << snd;
        }

        // Variable height jump gravity
        let gravity = if is_jump && self.mario.vy < 0.0 {
            0.22
        } else {
            0.50
        };
        self.mario.vy = (self.mario.vy + gravity).min(6.5);

        // Walk animation counter
        if self.mario.is_grounded && self.mario.vx.abs() > 0.1 {
            self.mario.walk_frame += self.mario.vx.abs() * 0.15;
            if self.mario.walk_frame >= 3.0 {
                self.mario.walk_frame -= 3.0;
            }
        } else {
            self.mario.walk_frame = 0.0;
        }

        // Collide Mario with tiles
        let mw = self.mario.width();
        let mh = self.mario.height();
        let col = move_and_collide(
            &self.level,
            &mut self.mario.x,
            &mut self.mario.y,
            &mut self.mario.vx,
            &mut self.mario.vy,
            mw,
            mh,
        );

        self.mario.is_grounded = col.hit_ground;

        // Block hitting from underneath
        if col.hit_ceiling {
            self.hit_block(col.ceiling_tile_x, col.ceiling_tile_y);
        }

        // Prevent moving behind camera left edge
        if self.mario.x < self.camera_x {
            self.mario.x = self.camera_x;
            if self.mario.vx < 0.0 {
                self.mario.vx = 0.0;
            }
        }

        // Mario pit fall check
        if self.mario.y > (SCREEN_HEIGHT as f32 + 16.0) {
            self.kill_mario();
            return;
        }

        // Flagpole check
        if self.mario.x + mw >= self.level.flagpole_x {
            self.start_flagpole_sequence();
            return;
        }

        // Update Camera (smooth forward tracking)
        let target_cam = self.mario.x - 100.0;
        if target_cam > self.camera_x {
            self.camera_x = target_cam;
        }
        let max_cam = ((LEVEL_WIDTH * 16) - SCREEN_WIDTH) as f32;
        self.camera_x = self.camera_x.clamp(0.0, max_cam);

        // Update Block Bump animations
        self.update_block_bumps();

        // Update Items
        self.update_items();

        // Update Enemies
        self.update_enemies();

        // Update Particles
        self.update_particles();
    }

    fn hit_block(&mut self, tx: usize, ty: usize) {
        let tile = self.level.get_tile(tx, ty);
        match tile {
            TileType::QuestionCoin => {
                self.level.set_tile(tx, ty, TileType::UsedBlock);
                self.add_block_bump(tx, ty);
                self.coins += 1;
                self.score += 200;
                self.sound_events |= 1 << SOUND_COIN;
                // Pop coin particle
                self.particles.push(Particle::CoinSparkle {
                    x: (tx as f32) * 16.0 + 4.0,
                    y: (ty as f32) * 16.0 - 8.0,
                    vy: -4.0,
                    frame: 0,
                });
                self.particles.push(Particle::ScorePopup {
                    x: (tx as f32) * 16.0,
                    y: (ty as f32) * 16.0 - 12.0,
                    vy: -0.6,
                    text: "200",
                    life: 40,
                });
            }
            TileType::QuestionMushroom => {
                self.level.set_tile(tx, ty, TileType::UsedBlock);
                self.add_block_bump(tx, ty);
                self.sound_events |= 1 << SOUND_POWERUP_SPAWN;
                let item_type = if self.mario.form == MarioForm::Small {
                    ItemType::Mushroom
                } else {
                    ItemType::FireFlower
                };
                self.items.push(Item::new(
                    item_type,
                    (tx as f32) * 16.0,
                    (ty as f32) * 16.0,
                ));
            }
            TileType::Brick => {
                if self.mario.form != MarioForm::Small {
                    // Smash brick!
                    self.level.set_tile(tx, ty, TileType::Air);
                    self.sound_events |= 1 << SOUND_BRICK_BREAK;
                    self.score += 50;
                    let bx = (tx as f32) * 16.0;
                    let by = (ty as f32) * 16.0;
                    self.particles.push(Particle::Debris { x: bx, y: by, vx: -1.5, vy: -4.5, life: 60 });
                    self.particles.push(Particle::Debris { x: bx + 8.0, y: by, vx: 1.5, vy: -4.5, life: 60 });
                    self.particles.push(Particle::Debris { x: bx, y: by + 8.0, vx: -1.2, vy: -2.5, life: 60 });
                    self.particles.push(Particle::Debris { x: bx + 8.0, y: by + 8.0, vx: 1.2, vy: -2.5, life: 60 });
                } else {
                    self.add_block_bump(tx, ty);
                    self.sound_events |= 1 << SOUND_BRICK_BUMP;
                }
            }
            _ => {
                self.sound_events |= 1 << SOUND_BRICK_BUMP;
            }
        }
    }

    fn add_block_bump(&mut self, tx: usize, ty: usize) {
        self.block_bumps.push(BlockBump {
            tile_x: tx,
            tile_y: ty,
            offset_y: 0.0,
            vy: -2.0,
            active: true,
        });
    }

    fn update_block_bumps(&mut self) {
        for bump in &mut self.block_bumps {
            if bump.active {
                bump.offset_y += bump.vy;
                bump.vy += 0.4;
                if bump.offset_y >= 0.0 {
                    bump.offset_y = 0.0;
                    bump.active = false;
                }
            }
        }
        self.block_bumps.retain(|b| b.active);
    }

    fn update_items(&mut self) {
        let mx = self.mario.x;
        let my = self.mario.y;
        let mw = self.mario.width();
        let mh = self.mario.height();

        for item in &mut self.items {
            if item.collected {
                continue;
            }

            if item.spawning {
                item.y -= 0.6;
                if item.y <= item.spawn_y_target {
                    item.y = item.spawn_y_target;
                    item.spawning = false;
                }
                continue;
            }

            // Normal item physics
            item.vy = (item.vy + 0.35).min(5.0);
            let mut ivx = item.vx;
            let mut ivy = item.vy;
            let col = move_and_collide(
                &self.level,
                &mut item.x,
                &mut item.y,
                &mut ivx,
                &mut ivy,
                14.0,
                14.0,
            );
            item.vy = ivy;
            if col.hit_wall_left || col.hit_wall_right {
                item.vx = -item.vx;
            }

            // Collision with Mario
            if aabb_overlap(mx, my, mw, mh, item.x, item.y, 14.0, 14.0) {
                item.collected = true;
                self.sound_events |= 1 << SOUND_POWERUP;
                self.score += 1000;
                match item.kind {
                    ItemType::Mushroom => {
                        if self.mario.form == MarioForm::Small {
                            self.mario.form = MarioForm::Super;
                            self.mario.y -= 14.0; // adjust height smoothly
                        }
                    }
                    ItemType::FireFlower => {
                        self.mario.form = MarioForm::Fire;
                    }
                    _ => {}
                }
                self.particles.push(Particle::ScorePopup {
                    x: item.x,
                    y: item.y - 8.0,
                    vy: -0.6,
                    text: "1000",
                    life: 40,
                });
            }
        }
        self.items.retain(|i| !i.collected);
    }

    fn update_enemies(&mut self) {
        let mx = self.mario.x;
        let my = self.mario.y;
        let mw = self.mario.width();
        let mh = self.mario.height();

        // 1. Enemy AI and movement
        for enemy in &mut self.enemies {
            if !enemy.alive {
                continue;
            }

            // Squished Goomba timer
            if enemy.squished {
                enemy.squish_timer += 1;
                if enemy.squish_timer > 25 {
                    enemy.alive = false;
                }
                continue;
            }

            // Only update enemies within 1.5 screens of camera
            let cam_left = self.camera_x - 32.0;
            let cam_right = self.camera_x + (SCREEN_WIDTH as f32) + 64.0;
            if enemy.x < cam_left || enemy.x > cam_right {
                continue;
            }

            // Gravity
            enemy.vy = (enemy.vy + 0.35).min(5.0);

            let ew = enemy.width();
            let eh = enemy.height();
            let col = move_and_collide(
                &self.level,
                &mut enemy.x,
                &mut enemy.y,
                &mut enemy.vx,
                &mut enemy.vy,
                ew,
                eh,
            );

            if col.hit_wall_left || col.hit_wall_right {
                enemy.vx = -enemy.vx;
                enemy.facing_right = enemy.vx > 0.0;
            }

            enemy.walk_frame += enemy.vx.abs() * 0.1;
            if enemy.walk_frame >= 2.0 {
                enemy.walk_frame -= 2.0;
            }

            // Enemy fell into pit
            if enemy.y > (SCREEN_HEIGHT as f32 + 32.0) {
                enemy.alive = false;
            }
        }

        // 2. Sliding Shell vs other enemies
        let mut shell_hits: Vec<usize> = Vec::new();
        for i in 0..self.enemies.len() {
            if !self.enemies[i].alive || !self.enemies[i].shell_sliding {
                continue;
            }
            let sx = self.enemies[i].x;
            let sy = self.enemies[i].y;
            for j in 0..self.enemies.len() {
                if i == j || !self.enemies[j].alive || self.enemies[j].squished {
                    continue;
                }
                let jx = self.enemies[j].x;
                let jy = self.enemies[j].y;
                let jw = self.enemies[j].width();
                let jh = self.enemies[j].height();
                if aabb_overlap(sx, sy, 14.0, 14.0, jx, jy, jw, jh) {
                    shell_hits.push(j);
                }
            }
        }

        for idx in shell_hits {
            if idx < self.enemies.len() && self.enemies[idx].alive {
                self.enemies[idx].alive = false;
                self.sound_events |= 1 << SOUND_KICK;
                self.score += 200;
                let ex = self.enemies[idx].x;
                let ey = self.enemies[idx].y;
                self.particles.push(Particle::ScorePopup {
                    x: ex,
                    y: ey - 8.0,
                    vy: -0.6,
                    text: "200",
                    life: 40,
                });
            }
        }

        // 3. Mario vs Enemy interaction
        let mario_invul = self.mario.invulnerable_timer > 0;
        for enemy in &mut self.enemies {
            if !enemy.alive || enemy.squished {
                continue;
            }

            let ew = enemy.width();
            let eh = enemy.height();

            if aabb_overlap(mx, my, mw, mh, enemy.x, enemy.y, ew, eh) {
                // Check if Mario stomps on top of enemy:
                // Mario's bottom is within top 8px of enemy and Mario is falling down
                let mario_bottom = my + mh;
                let enemy_top = enemy.y;
                let is_stomp = (mario_bottom - enemy_top) <= 10.0 && self.mario.vy >= 0.0;

                if is_stomp {
                    match enemy.kind {
                        EnemyType::Goomba => {
                            enemy.squished = true;
                            enemy.squish_timer = 0;
                            self.mario.vy = -4.0;
                            self.sound_events |= 1 << SOUND_STOMP;
                            self.score += 100;
                            self.particles.push(Particle::ScorePopup {
                                x: enemy.x,
                                y: enemy.y - 8.0,
                                vy: -0.6,
                                text: "100",
                                life: 40,
                            });
                        }
                        EnemyType::Koopa => {
                            if !enemy.is_shell {
                                // Turn into stationary shell
                                enemy.is_shell = true;
                                enemy.vx = 0.0;
                                enemy.y += 8.0; // shell is shorter
                                self.mario.vy = -4.0;
                                self.sound_events |= 1 << SOUND_STOMP;
                                self.score += 100;
                            } else if enemy.shell_sliding {
                                // Stop sliding shell
                                enemy.shell_sliding = false;
                                enemy.vx = 0.0;
                                self.mario.vy = -4.0;
                                self.sound_events |= 1 << SOUND_STOMP;
                            } else {
                                // Kick stationary shell
                                enemy.shell_sliding = true;
                                enemy.vx = if mx < enemy.x { 4.2 } else { -4.2 };
                                self.sound_events |= 1 << SOUND_KICK;
                                self.score += 400;
                            }
                        }
                    }
                } else {
                    // Mario touches enemy from the side
                    if enemy.is_shell && !enemy.shell_sliding {
                        // Kick stationary shell
                        enemy.shell_sliding = true;
                        enemy.vx = if mx < enemy.x { 4.2 } else { -4.2 };
                        self.sound_events |= 1 << SOUND_KICK;
                        self.score += 400;
                    } else if !mario_invul {
                        // Mario takes damage!
                        if self.mario.form != MarioForm::Small {
                            self.mario.form = MarioForm::Small;
                            self.mario.invulnerable_timer = 120; // 2 seconds flashing
                            self.sound_events |= 1 << SOUND_PIPE;
                        } else {
                            self.kill_mario();
                            return;
                        }
                    }
                }
            }
        }

        self.enemies.retain(|e| e.alive);
    }

    fn update_particles(&mut self) {
        for p in &mut self.particles {
            match p {
                Particle::Debris { x, y, vx, vy, life } => {
                    *x += *vx;
                    *y += *vy;
                    *vy += 0.3;
                    *life = life.saturating_sub(1);
                }
                Particle::CoinSparkle { y, vy, frame, .. } => {
                    *y += *vy;
                    *vy += 0.25;
                    *frame += 1;
                }
                Particle::ScorePopup { y, vy, life, .. } => {
                    *y += *vy;
                    *life = life.saturating_sub(1);
                }
            }
        }
        self.particles.retain(|p| match p {
            Particle::Debris { life, .. } => *life > 0,
            Particle::CoinSparkle { frame, .. } => *frame < 24,
            Particle::ScorePopup { life, .. } => *life > 0,
        });
    }

    fn kill_mario(&mut self) {
        self.state = GameState::Dying;
        self.mario.death_timer = 0;
        self.mario.vx = 0.0;
        self.mario.vy = 0.0;
        self.sound_events |= 1 << SOUND_DIE;
    }

    fn update_dying(&mut self) {
        self.mario.death_timer += 1;
        if self.mario.death_timer == 20 {
            self.mario.vy = -6.0;
        } else if self.mario.death_timer > 20 {
            self.mario.vy = (self.mario.vy + 0.35).min(7.0);
            self.mario.y += self.mario.vy;
        }

        if self.mario.death_timer > 140 {
            if self.lives > 1 {
                self.lives -= 1;
                self.reset_level();
                self.state = GameState::Playing;
            } else {
                self.lives = 0;
                self.state = GameState::GameOver;
                self.state_timer = 0;
                self.sound_events |= 1 << SOUND_GAME_OVER;
            }
        }
    }

    fn start_flagpole_sequence(&mut self) {
        self.state = GameState::FlagpoleSlide;
        self.mario.vx = 0.0;
        self.mario.vy = 0.0;
        self.mario.x = self.level.flagpole_x - 6.0;
        self.sound_events |= 1 << SOUND_FLAGPOLE;

        // Calculate bonus points based on catch height
        let bonus = if self.mario.y < 80.0 {
            5000
        } else if self.mario.y < 130.0 {
            2000
        } else {
            400
        };
        self.score += bonus;
        self.particles.push(Particle::ScorePopup {
            x: self.mario.x + 12.0,
            y: self.mario.y,
            vy: -0.6,
            text: if bonus == 5000 { "5000" } else if bonus == 2000 { "2000" } else { "400" },
            life: 60,
        });
    }

    fn update_flagpole_slide(&mut self) {
        let bottom_y = 12.0 * 16.0 - self.mario.height();
        if self.mario.y < bottom_y {
            self.mario.y += 2.0;
            self.flag_y = (self.flag_y + 2.0).min(11.0 * 16.0);
        } else {
            self.mario.y = bottom_y;
            self.mario.flagpole_timer += 1;
            if self.mario.flagpole_timer > 25 {
                // Hop off pole and walk into castle
                self.mario.x = self.level.flagpole_x + 10.0;
                self.mario.facing_right = true;
                self.state = GameState::CastleWalk;
                self.sound_events |= 1 << SOUND_STAGE_CLEAR;
            }
        }
    }

    fn update_castle_walk(&mut self) {
        if self.mario.x < self.level.castle_door_x {
            self.mario.x += 1.2;
            self.mario.walk_frame += 0.2;
            if self.mario.walk_frame >= 3.0 {
                self.mario.walk_frame -= 3.0;
            }
        } else {
            // Entered castle door!
            self.state = GameState::StageClear;
            self.state_timer = 0;
            // Add remaining time bonus to score
            self.score += self.time_remaining * 50;
            self.time_remaining = 0;
        }
    }
}
