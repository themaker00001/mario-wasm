use crate::types::*;
use crate::palette::*;
use crate::font::*;
use crate::sprites::*;
use crate::game::Game;
use crate::entities::Particle;

pub struct Renderer {
    pub sprites: AllSprites,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            sprites: AllSprites::new(),
        }
    }

    pub fn render(&self, game: &Game, fb: &mut [u32]) {
        // 1. Clear background to NES Sky Blue
        fb.fill(COLOR_SKY);

        // 2. Draw Parallax Background (Clouds, Hills, Bushes)
        self.draw_scenery(game, fb);

        // 3. Draw Level Tiles
        self.draw_tiles(game, fb);

        // 4. Draw Flag
        self.draw_flag(game, fb);

        // 5. Draw Items
        self.draw_items(game, fb);

        // 6. Draw Enemies
        self.draw_enemies(game, fb);

        // 7. Draw Mario
        self.draw_mario(game, fb);

        // 8. Draw Particles & Popups
        self.draw_particles(game, fb);

        // 9. Draw HUD
        self.draw_hud(game, fb);

        // 10. Draw State Screens (Title, Game Over, Stage Clear)
        self.draw_overlays(game, fb);
    }

    fn draw_scenery(&self, game: &Game, fb: &mut [u32]) {
        let cam_x = game.camera_x;

        // Clouds (Parallax 0.5)
        for i in 0..15 {
            let cx = (i * 240) as f32 - (cam_x * 0.4);
            let cy = 30.0 + ((i % 3) * 18) as f32;
            self.draw_cloud(fb, cx as i32, cy as i32);
        }

        // Hills
        for i in 0..12 {
            let hx = (i * 320) as f32 - cam_x;
            self.draw_hill(fb, hx as i32, 208);
            self.draw_hill(fb, (hx + 140.0) as i32, 208);
        }

        // Bushes
        for i in 0..15 {
            let bx = (i * 220 + 80) as f32 - cam_x;
            self.draw_bush(fb, bx as i32, 208);
        }
    }

    fn draw_cloud(&self, fb: &mut [u32], x: i32, y: i32) {
        if x < -80 || x > SCREEN_WIDTH as i32 + 80 {
            return;
        }
        // Cloud puffs
        self.fill_rect(fb, x + 10, y + 6, 28, 12, COLOR_WHITE);
        self.fill_rect(fb, x + 4, y + 10, 40, 8, COLOR_WHITE);
        self.fill_rect(fb, x + 6, y + 16, 36, 4, COLOR_CLOUD_SHADOW);
        self.fill_rect(fb, x + 18, y + 2, 12, 6, COLOR_WHITE);
    }

    fn draw_hill(&self, fb: &mut [u32], x: i32, y: i32) {
        if x < -80 || x > SCREEN_WIDTH as i32 + 80 {
            return;
        }
        // Green pyramid hill with rounded top
        for row in 0..32 {
            let ry = y - 32 + row;
            let hw = row * 4 / 3;
            self.fill_rect(fb, x - hw, ry, hw * 2, 1, COLOR_PIPE_DARK);
            self.fill_rect(fb, x - hw + 2, ry, hw * 2 - 4, 1, COLOR_PIPE_LIGHT);
        }
    }

    fn draw_bush(&self, fb: &mut [u32], x: i32, y: i32) {
        if x < -60 || x > SCREEN_WIDTH as i32 + 60 {
            return;
        }
        self.fill_rect(fb, x + 4, y - 10, 24, 10, COLOR_PIPE_DARK);
        self.fill_rect(fb, x + 6, y - 8, 20, 8, COLOR_PIPE_LIGHT);
        self.fill_rect(fb, x, y - 6, 32, 6, COLOR_PIPE_DARK);
    }

    fn draw_tiles(&self, game: &Game, fb: &mut [u32]) {
        let cam_x = game.camera_x;
        let start_col = ((cam_x / 16.0).floor() as i32).max(0) as usize;
        let end_col = (((cam_x + SCREEN_WIDTH as f32) / 16.0).ceil() as usize).min(LEVEL_WIDTH);

        let q_anim_frame = ((game.anim_tick / 10) % 4) as usize;

        for ty in 0..LEVEL_HEIGHT {
            for tx in start_col..end_col {
                let tile = game.level.get_tile(tx, ty);
                if tile == TileType::Air {
                    continue;
                }

                let draw_x = (tx as i32) * 16 - (cam_x as i32);
                let mut draw_y = (ty as i32) * 16;

                // Check block bump
                for bump in &game.block_bumps {
                    if bump.tile_x == tx && bump.tile_y == ty {
                        draw_y += bump.offset_y as i32;
                    }
                }

                match tile {
                    TileType::Ground => {
                        self.sprites.tile_ground.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, draw_x, draw_y, false);
                    }
                    TileType::Brick => {
                        self.sprites.tile_brick.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, draw_x, draw_y, false);
                    }
                    TileType::QuestionCoin | TileType::QuestionMushroom | TileType::QuestionStar => {
                        self.sprites.tile_question[q_anim_frame].draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, draw_x, draw_y, false);
                    }
                    TileType::UsedBlock => {
                        self.sprites.tile_used.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, draw_x, draw_y, false);
                    }
                    TileType::PipeTopLeft => {
                        self.sprites.tile_pipe_top_left.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, draw_x, draw_y, false);
                    }
                    TileType::PipeTopRight => {
                        self.sprites.tile_pipe_top_right.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, draw_x, draw_y, false);
                    }
                    TileType::PipeBodyLeft => {
                        self.sprites.tile_pipe_body_left.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, draw_x, draw_y, false);
                    }
                    TileType::PipeBodyRight => {
                        self.sprites.tile_pipe_body_right.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, draw_x, draw_y, false);
                    }
                    TileType::Stone => {
                        self.sprites.tile_stone.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, draw_x, draw_y, false);
                    }
                    TileType::Flagpole => {
                        self.fill_rect(fb, draw_x + 7, draw_y, 2, 16, COLOR_DARK_BROWN);
                    }
                    TileType::FlagpoleTop => {
                        self.sprites.flag_top.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, draw_x, draw_y, false);
                    }
                    TileType::CastleWall => {
                        self.sprites.tile_stone.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, draw_x, draw_y, false);
                    }
                    TileType::CastleTop => {
                        self.sprites.tile_stone.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, draw_x, draw_y, false);
                    }
                    TileType::CastleDoor => {
                        self.fill_rect(fb, draw_x, draw_y, 16, 16, COLOR_BLACK);
                    }
                    _ => {}
                }
            }
        }
    }

    fn draw_flag(&self, game: &Game, fb: &mut [u32]) {
        let flag_x = (game.level.flagpole_x - game.camera_x) as i32 - 14;
        let flag_y = game.flag_y as i32;
        self.sprites.flag.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, flag_x, flag_y, false);
    }

    fn draw_items(&self, game: &Game, fb: &mut [u32]) {
        let cam_x = game.camera_x;
        for item in &game.items {
            if item.collected {
                continue;
            }
            let ix = (item.x - cam_x) as i32;
            let iy = item.y as i32;
            match item.kind {
                ItemType::Mushroom => {
                    self.sprites.mushroom.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, ix, iy, false);
                }
                ItemType::FireFlower => {
                    self.sprites.flower.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, ix, iy, false);
                }
                _ => {}
            }
        }
    }

    fn draw_enemies(&self, game: &Game, fb: &mut [u32]) {
        let cam_x = game.camera_x;
        for enemy in &game.enemies {
            if !enemy.alive {
                continue;
            }
            let ex = (enemy.x - cam_x) as i32;
            let ey = enemy.y as i32;

            match enemy.kind {
                EnemyType::Goomba => {
                    if enemy.squished {
                        self.sprites.goomba_squish.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, ex, ey, false);
                    } else {
                        let frame = (enemy.walk_frame as usize) % 2;
                        if frame == 0 {
                            self.sprites.goomba_walk1.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, ex, ey, false);
                        } else {
                            self.sprites.goomba_walk2.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, ex, ey, false);
                        }
                    }
                }
                EnemyType::Koopa => {
                    if enemy.is_shell {
                        self.sprites.koopa_shell.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, ex, ey, false);
                    } else {
                        let frame = (enemy.walk_frame as usize) % 2;
                        let flip = enemy.facing_right;
                        if frame == 0 {
                            self.sprites.koopa_walk1.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, ex, ey - 6, flip);
                        } else {
                            self.sprites.koopa_walk2.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, ex, ey - 6, flip);
                        }
                    }
                }
            }
        }
    }

    fn draw_mario(&self, game: &Game, fb: &mut [u32]) {
        let mario = &game.mario;

        // Flash when invulnerable
        if mario.invulnerable_timer > 0 && ((mario.invulnerable_timer / 4) % 2 == 1) {
            return;
        }

        let mx = (mario.x - 2.0 - game.camera_x) as i32;
        let my = (mario.y - mario.draw_offset_y()) as i32;
        let flip = !mario.facing_right;

        match mario.form {
            MarioForm::Small => {
                if game.state == GameState::Dying {
                    self.sprites.mario_small_die.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, false);
                } else if !mario.is_grounded {
                    self.sprites.mario_small_jump.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, flip);
                } else if mario.is_skidding {
                    self.sprites.mario_small_skid.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, flip);
                } else if mario.vx.abs() > 0.1 {
                    let frame = (mario.walk_frame as usize) % 3;
                    match frame {
                        0 => self.sprites.mario_small_walk1.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, flip),
                        1 => self.sprites.mario_small_walk2.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, flip),
                        _ => self.sprites.mario_small_walk3.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, flip),
                    }
                } else {
                    self.sprites.mario_small_idle.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, flip);
                }
            }
            MarioForm::Super | MarioForm::Fire => {
                if mario.is_ducking {
                    self.sprites.mario_super_duck.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, flip);
                } else if !mario.is_grounded {
                    self.sprites.mario_super_jump.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, flip);
                } else if mario.is_skidding {
                    self.sprites.mario_super_skid.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, flip);
                } else if mario.vx.abs() > 0.1 {
                    let frame = (mario.walk_frame as usize) % 3;
                    match frame {
                        0 => self.sprites.mario_super_walk1.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, flip),
                        1 => self.sprites.mario_super_walk2.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, flip),
                        _ => self.sprites.mario_super_walk3.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, flip),
                    }
                } else {
                    self.sprites.mario_super_idle.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, mx, my, flip);
                }
            }
        }
    }

    fn draw_particles(&self, game: &Game, fb: &mut [u32]) {
        let cam_x = game.camera_x;
        for p in &game.particles {
            match p {
                Particle::Debris { x, y, .. } => {
                    let px = (*x - cam_x) as i32;
                    let py = *y as i32;
                    self.sprites.brick_debris.draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, px, py, false);
                }
                Particle::CoinSparkle { x, y, frame, .. } => {
                    let px = (*x - cam_x) as i32;
                    let py = *y as i32;
                    let coin_idx = ((*frame as usize) / 3) % 4;
                    self.sprites.coin_frames[coin_idx].draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, px, py, false);
                }
                Particle::ScorePopup { x, y, text, .. } => {
                    let px = (*x - cam_x) as i32;
                    let py = *y as i32;
                    draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, px, py, text, COLOR_WHITE, true);
                }
            }
        }
    }

    fn draw_hud(&self, game: &Game, fb: &mut [u32]) {
        // MARIO SCORE
        draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 16, 8, "MARIO", COLOR_WHITE, true);
        let score_str = format!("{:06}", game.score);
        draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 16, 16, &score_str, COLOR_WHITE, true);

        // COINS
        let coin_frame = ((game.anim_tick / 8) % 4) as usize;
        self.sprites.coin_frames[coin_frame].draw(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 88, 12, false);
        let coins_str = format!("x{:02}", game.coins);
        draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 104, 16, &coins_str, COLOR_WHITE, true);

        // WORLD
        draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 160, 8, "WORLD", COLOR_WHITE, true);
        draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 168, 16, "1-1", COLOR_WHITE, true);

        // TIME
        draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 224, 8, "TIME", COLOR_WHITE, true);
        let time_str = format!("{:03}", game.time_remaining);
        draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 228, 16, &time_str, COLOR_WHITE, true);

        // LIVES
        let lives_str = format!("x{}", game.lives);
        draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 280, 8, "LIVES", COLOR_WHITE, true);
        draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 288, 16, &lives_str, COLOR_WHITE, true);
    }

    fn draw_overlays(&self, game: &Game, fb: &mut [u32]) {
        match game.state {
            GameState::Title => {
                // Title Box
                self.fill_rect(fb, 40, 45, 240, 150, rgba(0, 0, 0, 180));
                self.draw_box_border(fb, 40, 45, 240, 150, COLOR_GOLD);

                draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 76, 60, "SUPER WASM BROS", COLOR_RED, true);
                draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 78, 62, "SUPER WASM BROS", COLOR_GOLD, false);

                let blink = (game.anim_tick / 30) % 2 == 0;
                if blink {
                    draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 68, 92, "PRESS START / SPACE", COLOR_WHITE, true);
                }

                draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 56, 120, "CONTROLS:", COLOR_GOLD, true);
                draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 56, 132, "ARROWS / WASD : MOVE", COLOR_WHITE, true);
                draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 56, 144, "SPACE / Z     : JUMP", COLOR_WHITE, true);
                draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 56, 156, "SHIFT / X     : RUN", COLOR_WHITE, true);
                draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 56, 172, "100% RUST & WEBASSEMBLY", COLOR_YELLOW_LIGHT, true);
            }
            GameState::GameOver => {
                self.fill_rect(fb, 60, 70, 200, 80, rgba(0, 0, 0, 200));
                self.draw_box_border(fb, 60, 70, 200, 80, COLOR_RED);
                draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 116, 95, "GAME OVER", COLOR_RED, true);
                let blink = (game.anim_tick / 30) % 2 == 0;
                if blink {
                    draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 84, 120, "PRESS START TO RETRY", COLOR_WHITE, true);
                }
            }
            GameState::StageClear => {
                self.fill_rect(fb, 50, 60, 220, 100, rgba(0, 0, 0, 200));
                self.draw_box_border(fb, 50, 60, 220, 100, COLOR_GOLD);
                draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 104, 80, "STAGE CLEAR!", COLOR_GOLD, true);
                draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 78, 100, "WELL DONE, SUPER MARIO!", COLOR_WHITE, true);
                let final_score = format!("FINAL SCORE: {:06}", game.score);
                draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 88, 118, &final_score, COLOR_YELLOW_LIGHT, true);
                let blink = (game.anim_tick / 30) % 2 == 0;
                if blink {
                    draw_text(fb, SCREEN_WIDTH, SCREEN_HEIGHT, 84, 138, "PRESS START TO REPLAY", COLOR_WHITE, true);
                }
            }
            _ => {}
        }
    }

    fn fill_rect(&self, fb: &mut [u32], x: i32, y: i32, w: i32, h: i32, color: u32) {
        let alpha = (color >> 24) & 0xFF;
        if alpha == 0 {
            return;
        }

        let start_x = x.max(0);
        let end_x = (x + w).min(SCREEN_WIDTH as i32);
        let start_y = y.max(0);
        let end_y = (y + h).min(SCREEN_HEIGHT as i32);

        if alpha == 255 {
            for py in start_y..end_y {
                let row_idx = (py as usize) * SCREEN_WIDTH;
                for px in start_x..end_x {
                    fb[row_idx + (px as usize)] = color;
                }
            }
        } else {
            // Alpha blend
            let a = alpha as f32 / 255.0;
            let inv_a = 1.0 - a;
            let r_src = ((color) & 0xFF) as f32;
            let g_src = ((color >> 8) & 0xFF) as f32;
            let b_src = ((color >> 16) & 0xFF) as f32;

            for py in start_y..end_y {
                let row_idx = (py as usize) * SCREEN_WIDTH;
                for px in start_x..end_x {
                    let idx = row_idx + (px as usize);
                    let dst = fb[idx];
                    let r_dst = ((dst) & 0xFF) as f32;
                    let g_dst = ((dst >> 8) & 0xFF) as f32;
                    let b_dst = ((dst >> 16) & 0xFF) as f32;

                    let r_out = (r_src * a + r_dst * inv_a) as u32;
                    let g_out = (g_src * a + g_dst * inv_a) as u32;
                    let b_out = (b_src * a + b_dst * inv_a) as u32;

                    fb[idx] = (r_out) | (g_out << 8) | (b_out << 16) | (255 << 24);
                }
            }
        }
    }

    fn draw_box_border(&self, fb: &mut [u32], x: i32, y: i32, w: i32, h: i32, color: u32) {
        self.fill_rect(fb, x, y, w, 2, color);
        self.fill_rect(fb, x, y + h - 2, w, 2, color);
        self.fill_rect(fb, x, y, 2, h, color);
        self.fill_rect(fb, x + w - 2, y, 2, h, color);
    }
}
