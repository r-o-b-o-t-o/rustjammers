use player::Player;
use frisbee::Frisbee;
use vector2::Vector2;

struct Circle {
    pub center: Vector2,
    pub radius: f64
}

pub fn player_collision(player: &mut Player) {
    let side = player.get_horizontal_position();
    const WALL_EXT: f64 = 9.4;
    const NET: f64 = 0.1;
    const WALL_VERTICAL: f64 = 4.4;

    if side > 0.0 {
        if player.pos.x > WALL_EXT {
            player.pos.x = WALL_EXT;
        }
        if player.pos.x < NET {
            player.pos.x = NET;
        }
    }
    if side < 0.0 {
        if player.pos.x < -WALL_EXT {
            player.pos.x = -WALL_EXT;
        }
        if player.pos.x > -NET {
            player.pos.x = -NET;
        }
    }
    if player.pos.y > WALL_VERTICAL {
        player.pos.y = WALL_VERTICAL;
    }
    if player.pos.y < -WALL_VERTICAL {
        player.pos.y = -WALL_VERTICAL;
    }
}

pub fn player_collides_with_frisbee(player: &Player, frisbee: &Frisbee) -> bool {
    match frisbee.last_held {
        Some(last_held) => {
            if last_held == player.side.unwrap() {
                return false;
            }
        },
        None => {}
    };

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
