use crate::types::*;

pub struct Mario {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub facing_right: bool,
    pub is_grounded: bool,
    pub is_skidding: bool,
    pub is_ducking: bool,
    pub form: MarioForm,
    pub walk_frame: f32,
    pub invulnerable_timer: u32,
    pub transform_timer: u32,
    pub death_timer: u32,
    pub flagpole_timer: u32,
}

impl Mario {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            facing_right: true,
            is_grounded: false,
            is_skidding: false,
            is_ducking: false,
            form: MarioForm::Small,
            walk_frame: 0.0,
            invulnerable_timer: 0,
            transform_timer: 0,
            death_timer: 0,
            flagpole_timer: 0,
        }
    }

    pub fn width(&self) -> f32 {
        12.0
    }

    pub fn height(&self) -> f32 {
        match self.form {
            MarioForm::Small => 15.0,
            MarioForm::Super | MarioForm::Fire => {
                if self.is_ducking {
                    15.0
                } else {
                    29.0
                }
            }
        }
    }

    pub fn draw_offset_y(&self) -> f32 {
        match self.form {
            MarioForm::Small => 0.0,
            MarioForm::Super | MarioForm::Fire => {
                if self.is_ducking {
                    14.0 // drawn 16px lower when ducking
                } else {
                    0.0
                }
            }
        }
    }
}

pub struct Enemy {
    pub kind: EnemyType,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub alive: bool,
    pub is_shell: bool,
    pub shell_sliding: bool,
    pub squished: bool,
    pub squish_timer: u32,
    pub walk_frame: f32,
    pub facing_right: bool,
}

impl Enemy {
    pub fn new_goomba(x: f32, y: f32) -> Self {
        Self {
            kind: EnemyType::Goomba,
            x,
            y,
            vx: -0.65,
            vy: 0.0,
            alive: true,
            is_shell: false,
            shell_sliding: false,
            squished: false,
            squish_timer: 0,
            walk_frame: 0.0,
            facing_right: false,
        }
    }

    pub fn new_koopa(x: f32, y: f32) -> Self {
        Self {
            kind: EnemyType::Koopa,
            x,
            y,
            vx: -0.65,
            vy: 0.0,
            alive: true,
            is_shell: false,
            shell_sliding: false,
            squished: false,
            squish_timer: 0,
            walk_frame: 0.0,
            facing_right: false,
        }
    }

    pub fn width(&self) -> f32 {
        14.0
    }

    pub fn height(&self) -> f32 {
        match self.kind {
            EnemyType::Goomba => 14.0,
            EnemyType::Koopa => {
                if self.is_shell {
                    14.0
                } else {
                    22.0
                }
            }
        }
    }
}

pub struct Item {
    pub kind: ItemType,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub spawning: bool,
    pub spawn_y_target: f32,
    pub collected: bool,
}

impl Item {
    pub fn new(kind: ItemType, x: f32, y: f32) -> Self {
        let spawn_y_target = y - 16.0;
        let vx = match kind {
            ItemType::Mushroom => 1.0,
            ItemType::Star => 1.2,
            _ => 0.0,
        };
        Self {
            kind,
            x,
            y,
            vx,
            vy: 0.0,
            spawning: true,
            spawn_y_target,
            collected: false,
        }
    }
}

pub enum Particle {
    Debris {
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        life: u32,
    },
    CoinSparkle {
        x: f32,
        y: f32,
        vy: f32,
        frame: u32,
    },
    ScorePopup {
        x: f32,
        y: f32,
        vy: f32,
        text: &'static str,
        life: u32,
    },
}

pub struct BlockBump {
    pub tile_x: usize,
    pub tile_y: usize,
    pub offset_y: f32,
    pub vy: f32,
    pub active: bool,
}
