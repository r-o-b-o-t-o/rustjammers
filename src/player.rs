use vector2::Vector2;

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerSide {
    Left = 0,
    Right = 1
}

pub fn player_side_to_i8(side: Option<PlayerSide>) -> i8 {
    match side {
        Some(ref side) => *side as i8,
        None => -1
    }
}

pub fn player_side_from_i8(side: i8) -> Option<PlayerSide> {
    match side {
        0 => Some(PlayerSide::Left),
        1 => Some(PlayerSide::Right),
        _ => None
    }
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
