use frisbee::Frisbee;
use vector2::Vector2;
use player::{ Player, PlayerSide };

struct Rect {
    pub pos: Vector2,
    pub size: Vector2
}

struct Circle {
    pub center: Vector2,
    pub radius: f64
}

pub fn player_collision(player: &mut Player) {
    let side = player.get_horizontal_position();

    if side == 1.0 {
        if player.pos.x > 9.4 {
            player.pos.x = 9.4;
        }
        if player.pos.x <  0.1 {
            player.pos.x = 0.1;
        }
    }
    if side == -1.0 {
        if player.pos.x < -9.4 {
            player.pos.x = -9.4;
        }
        if player.pos.x > -0.1 {
            player.pos.x = -0.1;
        }
    }
    if player.pos.y > 4.4 {
        player.pos.y = 4.4;
    }
    if player.pos.y < -4.4 {
        player.pos.y = -4.4;
    }
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
