#![allow(static_mut_refs)]

pub mod types;
pub mod palette;
pub mod font;
pub mod sprites;
pub mod level;
pub mod entities;
pub mod physics;
pub mod game;
pub mod renderer;

use types::{SCREEN_WIDTH, SCREEN_HEIGHT, GameState};
use game::Game;
use renderer::Renderer;

struct Engine {
    game: Game,
    renderer: Renderer,
    framebuffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
}

static mut ENGINE: Option<Engine> = None;

#[no_mangle]
pub extern "C" fn init_game() {
    unsafe {
        ENGINE = Some(Engine {
            game: Game::new(),
            renderer: Renderer::new(),
            framebuffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
        });
    }
}

#[no_mangle]
pub extern "C" fn update_game(input_bits: u32) -> u32 {
    unsafe {
        if let Some(engine) = ENGINE.as_mut() {
            engine.game.update(input_bits);
            engine.renderer.render(&engine.game, &mut engine.framebuffer);
            engine.game.sound_events
        } else {
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn get_framebuffer_ptr() -> *const u32 {
    unsafe {
        if let Some(engine) = ENGINE.as_ref() {
            engine.framebuffer.as_ptr()
        } else {
            core::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn get_screen_width() -> u32 {
    SCREEN_WIDTH as u32
}

#[no_mangle]
pub extern "C" fn get_screen_height() -> u32 {
    SCREEN_HEIGHT as u32
}

#[no_mangle]
pub extern "C" fn get_game_state() -> u32 {
    unsafe {
        if let Some(engine) = ENGINE.as_ref() {
            match engine.game.state {
                GameState::Title => 0,
                GameState::Playing => 1,
                GameState::Dying => 2,
                GameState::GameOver => 3,
                GameState::FlagpoleSlide => 4,
                GameState::CastleWalk => 5,
                GameState::StageClear => 6,
            }
        } else {
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn get_score() -> u32 {
    unsafe { ENGINE.as_ref().map(|e| e.game.score).unwrap_or(0) }
}

#[no_mangle]
pub extern "C" fn get_coins() -> u32 {
    unsafe { ENGINE.as_ref().map(|e| e.game.coins).unwrap_or(0) }
}

#[no_mangle]
pub extern "C" fn get_lives() -> u32 {
    unsafe { ENGINE.as_ref().map(|e| e.game.lives).unwrap_or(0) }
}

#[no_mangle]
pub extern "C" fn get_time() -> u32 {
    unsafe { ENGINE.as_ref().map(|e| e.game.time_remaining).unwrap_or(0) }
}

#[no_mangle]
pub extern "C" fn restart_game() {
    unsafe {
        if let Some(engine) = ENGINE.as_mut() {
            engine.game.start_game();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_init_and_update() {
        init_game();
        // Simulate title screen start
        update_game(types::INPUT_START);
        // Simulate playing: running right and jumping
        for _ in 0..120 {
            update_game(types::INPUT_RIGHT | types::INPUT_RUN);
        }
        for _ in 0..30 {
            update_game(types::INPUT_RIGHT | types::INPUT_JUMP);
        }
        for _ in 0..200 {
            update_game(types::INPUT_RIGHT);
        }
        assert!(get_screen_width() == 320);
        assert!(get_screen_height() == 240);
        assert!(!get_framebuffer_ptr().is_null());
    }
}

