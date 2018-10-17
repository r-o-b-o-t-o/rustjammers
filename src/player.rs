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
pub struct Slide {
    pub target: Vector2,
    pub origin: Vector2,
    pub dir: Vector2,
}

impl Slide {
    pub fn has_reached_goal(&self, pos: &Vector2) -> bool {
        fn sqr_dist(a: &Vector2, b: &Vector2) -> f64 {
            (b.x - a.x).powf(2.0) + (b.y - a.y).powf(2.0)
        }
        sqr_dist(pos, &self.origin) >= sqr_dist(&self.origin, &self.target)
    }
}

#[derive(Clone, Copy)]
pub struct Player {
    pub pos:   Vector2,
    pub side:  Option<PlayerSide>,
    pub score: i8,
    pub slide: Option<Slide>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos:   Vector2::zero(),
            side:  None,
            score: 0,
            slide: None,
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
            ThrowDirection::LightUp => {
                let mut dir = Vector2::new(horizontal * 2.0, 1.0);
                dir.normalize();
                dir
            },
            ThrowDirection::Middle => {
                Vector2::new(horizontal, 0.0)
            },
            ThrowDirection::LightDown => {
                let mut dir = Vector2::new(horizontal * 2.0, -1.0);
                dir.normalize();
                dir
            },
            ThrowDirection::Down => {
                let mut dir = Vector2::new(horizontal, -1.0);
                dir.normalize();
                dir
            }
        }
    }

    pub fn dash(&mut self, dir: Vector2) {
        if self.slide.is_none() {
            self.slide = Some(Slide {
                origin: self.pos,
                target: self.pos + dir,
                dir: dir.normalized(),
            });
        }
    }

    pub fn dash_to_pos(&mut self, pos: Vector2) {
        self.slide = Some(Slide {
            origin: self.pos,
            target: pos,
            dir: (pos - self.pos).normalized()
        });
    }
}
