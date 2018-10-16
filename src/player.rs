use vector2::Vector2;
use frisbee::ThrowDirection;

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

#[derive(Clone, Copy)]
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

    pub fn get_horizontal_position(&self) -> f64 {
        self.get_horizontal_aim_direction() * -1.0
    }

    pub fn get_horizontal_aim_direction(&self) -> f64 {
        match self.side {
            Some(side) => {
                match side {
                    PlayerSide::Left => 1.0,
                    PlayerSide::Right => -1.0,
                }
            },
            None => 0.0
        }
    }

    pub fn get_throw_vector(&self, dir: &ThrowDirection) -> Vector2 {
        let horizontal = self.get_horizontal_aim_direction();
        match dir {
            ThrowDirection::Up => {
                let mut dir = Vector2::new(horizontal, 1.0);
                dir.normalize();
                dir
            },
            ThrowDirection::Middle => {
                Vector2::new(horizontal, 0.0)
            },
            ThrowDirection::Down => {
                let mut dir = Vector2::new(horizontal, -1.0);
                dir.normalize();
                dir
            }
        }
    }
}
