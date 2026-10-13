pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (r as u32) | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24)
}

pub const fn rgb(r: u8, g: u8, b: u8) -> u32 {
    rgba(r, g, b, 255)
}

// Transparent
pub const COLOR_TRANSPARENT: u32 = 0;

// Classic NES Palette Colors
pub const COLOR_SKY: u32 = rgb(92, 148, 252);
pub const COLOR_BLACK: u32 = rgb(0, 0, 0);
pub const COLOR_WHITE: u32 = rgb(255, 255, 255);

// Mario & Brick Colors
pub const COLOR_RED: u32 = rgb(216, 40, 0);
pub const COLOR_MARIO_BLUE: u32 = rgb(0, 112, 236);
pub const COLOR_SKIN: u32 = rgb(252, 152, 56);
pub const COLOR_DARK_BROWN: u32 = rgb(136, 112, 0);
pub const COLOR_LIGHT_BROWN: u32 = rgb(200, 76, 12);
pub const COLOR_BRICK_ORANGE: u32 = rgb(228, 92, 16);
pub const COLOR_BRICK_LIGHT: u32 = rgb(252, 188, 176);

// Question & Coin Colors
pub const COLOR_GOLD: u32 = rgb(252, 188, 60);
pub const COLOR_YELLOW_LIGHT: u32 = rgb(252, 216, 168);
pub const COLOR_GOLD_DARK: u32 = rgb(180, 100, 0);

// Pipe & Hill Green Colors
pub const COLOR_PIPE_LIGHT: u32 = rgb(128, 208, 16);
pub const COLOR_PIPE_DARK: u32 = rgb(0, 168, 0);
pub const COLOR_BUSH_DARK: u32 = rgb(0, 100, 0);

// Goomba & Koopa Colors
pub const COLOR_GOOMBA_DARK: u32 = rgb(136, 40, 0);
pub const COLOR_GOOMBA_BODY: u32 = rgb(228, 92, 16);
pub const COLOR_KOOPA_SHELL: u32 = rgb(0, 168, 0);
pub const COLOR_KOOPA_BELLY: u32 = rgb(252, 216, 168);

// Cloud / Castle Colors
pub const COLOR_CLOUD_SHADOW: u32 = rgb(216, 232, 248);
pub const COLOR_CASTLE_GRAY: u32 = rgb(188, 188, 188);
pub const COLOR_CASTLE_DARK: u32 = rgb(116, 116, 116);
pub const COLOR_FLAG_GREEN: u32 = rgb(0, 168, 0);
