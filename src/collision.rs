use player::Player;
use frisbee::Frisbee;
use vector2::Vector2;

struct Rect {
    pub pos: Vector2,
    pub size: Vector2
}

struct Circle {
    pub center: Vector2,
    pub radius: f64
}

pub fn player_collides_with_frisbee(player: &Player, frisbee: &Frisbee) -> bool {
    let player_bounds = Circle {
        center: player.pos,
        radius: 1.0
    };
    let frisbee_bounds = Circle {
        center: frisbee.pos,
        radius: 1.0
    };

    let d2 = (player_bounds.center.x - frisbee_bounds.center.x).powf(2.0) + (player_bounds.center.y - frisbee_bounds.center.y).powf(2.0);
    d2 <= (player_bounds.radius + frisbee_bounds.radius).powf(2.0)
}
