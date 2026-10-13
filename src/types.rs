pub const SCREEN_WIDTH: usize = 320;
pub const SCREEN_HEIGHT: usize = 240;
pub const TILE_SIZE: i32 = 16;
pub const LEVEL_WIDTH: usize = 220;
pub const LEVEL_HEIGHT: usize = 15; // 15 * 16 = 240

// Input bitflags
pub const INPUT_LEFT: u32 = 1 << 0;
pub const INPUT_RIGHT: u32 = 1 << 1;
pub const INPUT_UP: u32 = 1 << 2;
pub const INPUT_DOWN: u32 = 1 << 3;
pub const INPUT_JUMP: u32 = 1 << 4;
pub const INPUT_RUN: u32 = 1 << 5;
pub const INPUT_START: u32 = 1 << 6;

// Sound event IDs
pub const SOUND_NONE: u32 = 0;
pub const SOUND_JUMP_SMALL: u32 = 1;
pub const SOUND_JUMP_SUPER: u32 = 2;
pub const SOUND_COIN: u32 = 3;
pub const SOUND_STOMP: u32 = 4;
pub const SOUND_POWERUP_SPAWN: u32 = 5;
pub const SOUND_POWERUP: u32 = 6;
pub const SOUND_BRICK_BUMP: u32 = 7;
pub const SOUND_BRICK_BREAK: u32 = 8;
pub const SOUND_PIPE: u32 = 9;
pub const SOUND_DIE: u32 = 10;
pub const SOUND_FLAGPOLE: u32 = 11;
pub const SOUND_STAGE_CLEAR: u32 = 12;
pub const SOUND_KICK: u32 = 13;
pub const SOUND_FIREBALL: u32 = 14;
pub const SOUND_GAME_OVER: u32 = 15;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Title,
    Playing,
    Dying,
    GameOver,
    FlagpoleSlide,
    CastleWalk,
    StageClear,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MarioForm {
    Small,
    Super,
    Fire,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Air,
    Ground,
    Brick,
    QuestionCoin,
    QuestionMushroom,
    QuestionStar,
    UsedBlock,
    PipeTopLeft,
    PipeTopRight,
    PipeBodyLeft,
    PipeBodyRight,
    Stone,
    Flagpole,
    FlagpoleTop,
    CastleWall,
    CastleTop,
    CastleDoor,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Goomba,
    Koopa,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Mushroom,
    FireFlower,
    Star,
    BlockCoin,
}
