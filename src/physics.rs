use crate::types::*;
use crate::level::Level;

pub fn is_solid_tile(t: TileType) -> bool {
    matches!(
        t,
        TileType::Ground
            | TileType::Brick
            | TileType::QuestionCoin
            | TileType::QuestionMushroom
            | TileType::QuestionStar
            | TileType::UsedBlock
            | TileType::PipeTopLeft
            | TileType::PipeTopRight
            | TileType::PipeBodyLeft
            | TileType::PipeBodyRight
            | TileType::Stone
            | TileType::CastleWall
    )
}

pub fn check_box_collision(
    level: &Level,
    box_x: f32,
    box_y: f32,
    box_w: f32,
    box_h: f32,
) -> bool {
    let start_tx = (box_x / 16.0).floor() as i32;
    let end_tx = ((box_x + box_w - 0.01) / 16.0).floor() as i32;
    let start_ty = (box_y / 16.0).floor() as i32;
    let end_ty = ((box_y + box_h - 0.01) / 16.0).floor() as i32;

    for ty in start_ty..=end_ty {
        for tx in start_tx..=end_tx {
            if tx < 0 || tx >= LEVEL_WIDTH as i32 || ty < 0 || ty >= LEVEL_HEIGHT as i32 {
                continue;
            }
            let tile = level.get_tile(tx as usize, ty as usize);
            if is_solid_tile(tile) {
                return true;
            }
        }
    }
    false
}

pub struct CollisionResult {
    pub hit_ground: bool,
    pub hit_ceiling: bool,
    pub hit_wall_left: bool,
    pub hit_wall_right: bool,
    pub ceiling_tile_x: usize,
    pub ceiling_tile_y: usize,
}

pub fn move_and_collide(
    level: &Level,
    x: &mut f32,
    y: &mut f32,
    vx: &mut f32,
    vy: &mut f32,
    w: f32,
    h: f32,
) -> CollisionResult {
    let mut res = CollisionResult {
        hit_ground: false,
        hit_ceiling: false,
        hit_wall_left: false,
        hit_wall_right: false,
        ceiling_tile_x: 0,
        ceiling_tile_y: 0,
    };

    // 1. Move horizontally
    *x += *vx;
    if *vx > 0.0 {
        if check_box_collision(level, *x, *y, w, h) {
            // align with left edge of tile
            let tile_x = ((*x + w) / 16.0).floor() as i32;
            *x = (tile_x as f32) * 16.0 - w;
            *vx = 0.0;
            res.hit_wall_right = true;
        }
    } else if *vx < 0.0 {
        if check_box_collision(level, *x, *y, w, h) {
            // align with right edge of tile
            let tile_x = (*x / 16.0).floor() as i32;
            *x = ((tile_x + 1) as f32) * 16.0;
            *vx = 0.0;
            res.hit_wall_left = true;
        }
    }

    // 2. Move vertically
    *y += *vy;
    if *vy > 0.0 {
        if check_box_collision(level, *x, *y, w, h) {
            let tile_y = ((*y + h) / 16.0).floor() as i32;
            *y = (tile_y as f32) * 16.0 - h;
            *vy = 0.0;
            res.hit_ground = true;
        }
    } else if *vy < 0.0 {
        if check_box_collision(level, *x, *y, w, h) {
            let tile_y = (*y / 16.0).floor() as i32;
            *y = ((tile_y + 1) as f32) * 16.0;
            *vy = 0.0;
            res.hit_ceiling = true;
            let center_x = *x + w * 0.5;
            res.ceiling_tile_x = (center_x / 16.0).floor() as usize;
            res.ceiling_tile_y = tile_y as usize;
        }
    }

    res
}

pub fn aabb_overlap(
    x1: f32,
    y1: f32,
    w1: f32,
    h1: f32,
    x2: f32,
    y2: f32,
    w2: f32,
    h2: f32,
) -> bool {
    x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
}
