use vector2::Vector2;
use player::PlayerSide;

pub struct Frisbee {
    pub pos:            Vector2,
    pub direction:      Vector2,
    pub speed:          f64,
    pub held_by_player: Option<PlayerSide>
}

pub enum ThrowDirection {
    Up = 0,
    Middle,
    Down,
}

impl Frisbee {
    pub fn new() -> Self {
        Self {
            pos:            Vector2::zero(),
            direction:      Vector2::zero(),
            speed:          0.0,
            held_by_player: None
        }
    }
}
