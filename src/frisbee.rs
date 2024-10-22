use vector2::Vector2;
use player::PlayerSide;

use rand::Rng;

#[derive(Clone, Copy)]
pub struct Frisbee {
    pub pos:            Vector2,
    pub direction:      Vector2,
    pub speed:          f64,
    pub held_by_player: Option<PlayerSide>,
    pub last_held:      Option<PlayerSide>,
}

#[derive(Clone, Copy, Debug)]
pub enum ThrowDirection {
    Up = 0,
    LightUp,
    Middle,
    LightDown,
    Down,
}

pub fn random_throw_direction() -> ThrowDirection {
    let mut rng = ::rand::thread_rng();
    match rng.gen_range(0, 5) {
        0 => ThrowDirection::Up,
        1 => ThrowDirection::LightUp,
        2 => ThrowDirection::Middle,
        3 => ThrowDirection::LightDown,
        _ => ThrowDirection::Down,
    }
}

impl Frisbee {
    pub fn new() -> Self {
        Self {
            pos:            Vector2::zero(),
            direction:      Vector2::zero(),
            speed:          0.0,
            held_by_player: None,
            last_held:      None
        }
    }
}
