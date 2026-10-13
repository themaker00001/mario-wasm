use crate::types::*;
use crate::entities::Enemy;

pub struct Level {
    pub tiles: [TileType; LEVEL_WIDTH * LEVEL_HEIGHT],
    pub initial_enemies: Vec<Enemy>,
    pub flagpole_x: f32,
    pub castle_door_x: f32,
}

impl Level {
    pub fn new_world_1_1() -> Self {
        let mut tiles = [TileType::Air; LEVEL_WIDTH * LEVEL_HEIGHT];

        let set_tile = |tiles: &mut [TileType], tx: usize, ty: usize, t: TileType| {
            if tx < LEVEL_WIDTH && ty < LEVEL_HEIGHT {
                tiles[ty * LEVEL_WIDTH + tx] = t;
            }
        };

        // 1. Default Ground across the level (rows 13 & 14)
        for x in 0..LEVEL_WIDTH {
            // Pits at certain locations
            let is_pit = (x >= 57 && x <= 59) || (x >= 86 && x <= 88) || (x >= 130 && x <= 132);
            if !is_pit {
                set_tile(&mut tiles, x, 13, TileType::Ground);
                set_tile(&mut tiles, x, 14, TileType::Ground);
            }
        }

        // 2. Question Blocks and Bricks in opening area
        // Floating single '?' block
        set_tile(&mut tiles, 16, 9, TileType::QuestionCoin);

        // Brick - '?' (Mushroom) - Brick - '?' (Coin) - Brick
        set_tile(&mut tiles, 20, 9, TileType::Brick);
        set_tile(&mut tiles, 21, 9, TileType::QuestionMushroom);
        set_tile(&mut tiles, 22, 9, TileType::Brick);
        set_tile(&mut tiles, 23, 9, TileType::QuestionCoin);
        set_tile(&mut tiles, 24, 9, TileType::Brick);

        // High '?' block with coin
        set_tile(&mut tiles, 22, 5, TileType::QuestionCoin);

        // 3. Pipes
        let make_pipe = |tiles: &mut [TileType], tx: usize, top_y: usize| {
            set_tile(tiles, tx, top_y, TileType::PipeTopLeft);
            set_tile(tiles, tx + 1, top_y, TileType::PipeTopRight);
            for y in (top_y + 1)..13 {
                set_tile(tiles, tx, y, TileType::PipeBodyLeft);
                set_tile(tiles, tx + 1, y, TileType::PipeBodyRight);
            }
        };

        make_pipe(&mut tiles, 28, 11); // Short Pipe (height 2)
        make_pipe(&mut tiles, 38, 10); // Medium Pipe (height 3)
        make_pipe(&mut tiles, 46, 9);  // Tall Pipe (height 4)
        make_pipe(&mut tiles, 54, 9);  // Tall Pipe before pit

        // 4. Floating platforms & bricks after first pit
        set_tile(&mut tiles, 64, 9, TileType::Brick);
        set_tile(&mut tiles, 65, 9, TileType::QuestionMushroom);
        set_tile(&mut tiles, 66, 9, TileType::Brick);
        set_tile(&mut tiles, 67, 9, TileType::QuestionCoin);
        set_tile(&mut tiles, 68, 9, TileType::Brick);

        // High Brick platform
        for x in 76..82 {
            set_tile(&mut tiles, x, 5, TileType::Brick);
        }
        set_tile(&mut tiles, 78, 9, TileType::QuestionCoin);
        set_tile(&mut tiles, 80, 9, TileType::QuestionCoin);

        // Stepping stone across pit 2
        set_tile(&mut tiles, 87, 10, TileType::Stone);

        // Multi-level Bricks & '?'
        set_tile(&mut tiles, 93, 9, TileType::Brick);
        set_tile(&mut tiles, 94, 9, TileType::QuestionCoin);
        set_tile(&mut tiles, 95, 9, TileType::Brick);

        for x in 98..102 {
            set_tile(&mut tiles, x, 5, TileType::Brick);
        }

        make_pipe(&mut tiles, 106, 11);

        // 5. Pyramid / Staircase (Stone steps)
        let make_stairs_up = |tiles: &mut [TileType], start_x: usize, height: usize| {
            for step in 0..height {
                let x = start_x + step;
                for h in 0..=step {
                    set_tile(tiles, x, 12 - h, TileType::Stone);
                }
            }
        };

        let make_stairs_down = |tiles: &mut [TileType], start_x: usize, height: usize| {
            for step in 0..height {
                let x = start_x + step;
                let step_h = height - step;
                for h in 0..step_h {
                    set_tile(tiles, x, 12 - h, TileType::Stone);
                }
            }
        };

        make_stairs_up(&mut tiles, 115, 4);
        make_stairs_down(&mut tiles, 120, 4);

        // Stepping stone for pit 3
        set_tile(&mut tiles, 131, 10, TileType::Stone);

        // Pipe after pit 3
        make_pipe(&mut tiles, 137, 10);

        // Brick bridge
        for x in 142..146 {
            set_tile(&mut tiles, x, 9, TileType::Brick);
        }
        set_tile(&mut tiles, 144, 9, TileType::QuestionCoin);

        // 6. Huge Staircase leading up to Flagpole (8 tiles high!)
        make_stairs_up(&mut tiles, 151, 8);
        set_tile(&mut tiles, 159, 12, TileType::Stone);

        // 7. Flagpole at x = 168
        let flagpole_tx = 168;
        set_tile(&mut tiles, flagpole_tx, 12, TileType::Stone);
        set_tile(&mut tiles, flagpole_tx, 3, TileType::FlagpoleTop);
        for y in 4..12 {
            set_tile(&mut tiles, flagpole_tx, y, TileType::Flagpole);
        }

        // 8. Castle at x = 175..181
        for cx in 175..182 {
            for cy in 9..13 {
                set_tile(&mut tiles, cx, cy, TileType::CastleWall);
            }
        }
        // Castle battlements on top
        for cx in [175, 177, 179, 181] {
            set_tile(&mut tiles, cx, 8, TileType::CastleTop);
        }
        // Castle door
        set_tile(&mut tiles, 178, 12, TileType::CastleDoor);
        set_tile(&mut tiles, 178, 11, TileType::CastleDoor);

        // Enemies setup
        let mut initial_enemies = Vec::new();
        // Goombas
        initial_enemies.push(Enemy::new_goomba(22.0 * 16.0, 12.0 * 16.0));
        initial_enemies.push(Enemy::new_goomba(34.0 * 16.0, 12.0 * 16.0));
        initial_enemies.push(Enemy::new_goomba(42.0 * 16.0, 12.0 * 16.0));
        initial_enemies.push(Enemy::new_goomba(50.0 * 16.0, 12.0 * 16.0));
        initial_enemies.push(Enemy::new_goomba(52.0 * 16.0, 12.0 * 16.0));

        // Koopa Troopa patrolling
        initial_enemies.push(Enemy::new_koopa(75.0 * 16.0, 12.0 * 16.0));
        initial_enemies.push(Enemy::new_goomba(79.0 * 16.0, 4.0 * 16.0));

        // More Goombas & Koopas
        initial_enemies.push(Enemy::new_goomba(96.0 * 16.0, 12.0 * 16.0));
        initial_enemies.push(Enemy::new_koopa(102.0 * 16.0, 12.0 * 16.0));
        initial_enemies.push(Enemy::new_goomba(122.0 * 16.0, 12.0 * 16.0));
        initial_enemies.push(Enemy::new_goomba(124.0 * 16.0, 12.0 * 16.0));
        initial_enemies.push(Enemy::new_goomba(143.0 * 16.0, 12.0 * 16.0));

        Self {
            tiles,
            initial_enemies,
            flagpole_x: (flagpole_tx as f32) * 16.0,
            castle_door_x: 178.0 * 16.0 + 8.0,
        }
    }

    pub fn get_tile(&self, tx: usize, ty: usize) -> TileType {
        if tx < LEVEL_WIDTH && ty < LEVEL_HEIGHT {
            self.tiles[ty * LEVEL_WIDTH + tx]
        } else {
            TileType::Air
        }
    }

    pub fn set_tile(&mut self, tx: usize, ty: usize, tile: TileType) {
        if tx < LEVEL_WIDTH && ty < LEVEL_HEIGHT {
            self.tiles[ty * LEVEL_WIDTH + tx] = tile;
        }
    }
}
