use vector2::Vector2;

pub struct Frisbee {
    pub pos:       Vector2,
    pub direction: Vector2,
    pub speed:     f64
}

impl Frisbee {
    pub fn new() -> Self {
        Self {
            pos:       Vector2::zero(),
            direction: Vector2::zero(),
            speed:     0.0
        }
    }
}
