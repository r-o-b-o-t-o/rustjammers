use vector2::Vector2;

pub enum PlayerSide {
    Left,
    Right
}

pub struct Player {
    pub pos:   Vector2,
    pub side:  Option<PlayerSide>,
    pub score: i8,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos:   Vector2::zero(),
            side:  None,
            score: 0
        }
    }
}
