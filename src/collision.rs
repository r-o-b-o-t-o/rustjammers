use frisbee::Frisbee;
use vector2::Vector2;
use player::{ Player, PlayerSide };

struct Circle {
    pub center: Vector2,
    pub radius: f64
}

pub fn player_collision(player: &mut Player) -> bool {
    let side = player.get_horizontal_position();
    let mut collided = false;
    const WALL_EXT: f64 = 9.4;
    const NET: f64 = 0.75;
    const WALL_VERTICAL: f64 = 4.4;

    if side > 0.0 {
        if player.pos.x > WALL_EXT {
            player.pos.x = WALL_EXT;
            collided = true;
        }
        if player.pos.x < NET {
            player.pos.x = NET;
            collided = true;
        }
    }
    if side < 0.0 {
        if player.pos.x < -WALL_EXT {
            player.pos.x = -WALL_EXT;
            collided = true;
        }
        if player.pos.x > -NET {
            player.pos.x = -NET;
            collided = true;
        }
    }
    if player.pos.y > WALL_VERTICAL {
        player.pos.y = WALL_VERTICAL;
        collided = true;
    }
    if player.pos.y < -WALL_VERTICAL {
        player.pos.y = -WALL_VERTICAL;
        collided = true;
    }

    collided
}

pub fn frisbee_collision_wall(frisbee: &mut Frisbee) {
    const WALL_VERTICAL: f64 = 4.4;

    // TODO: check rebound angles
    if frisbee.pos.y >= WALL_VERTICAL || frisbee.pos.y <= -WALL_VERTICAL {
        frisbee.direction.y *= -1.0;
        if frisbee.pos.y >= WALL_VERTICAL {
            frisbee.direction = Vector2::new(frisbee.direction.x, -(frisbee.direction.y.abs())).normalized();
        }

        if frisbee.pos.y <= -WALL_VERTICAL {
            frisbee.direction = Vector2::new(frisbee.direction.x, frisbee.direction.y.abs()).normalized();
        }

        if frisbee.direction.y == 0.0 {
            if frisbee.pos.y >= WALL_VERTICAL {
                frisbee.pos.y = WALL_VERTICAL - 0.001;
            } else if frisbee.pos.y <= -WALL_VERTICAL {
                frisbee.pos.y = -WALL_VERTICAL + 0.001;
            }
        }
    }
}

pub fn frisbee_collision_goal(frisbee: &mut Frisbee, players: &mut (Player, Player)) -> bool {
    const WALL_EXT: f64 = 9.4 + 0.5;
    const FIVE_POINTS_START: f64 = 3.3 / 2.0;
    const FIVE_POINTS_END: f64 = -3.3 / 2.0;

    if frisbee.pos.x >= WALL_EXT || frisbee.pos.x <= -WALL_EXT {
        let mut points = 3;
        if frisbee.pos.y > FIVE_POINTS_END && frisbee.pos.y < FIVE_POINTS_START {
            points = 5;
        }

        match frisbee.last_held {
            Some(PlayerSide::Left) => players.0.score += points,
            Some(PlayerSide::Right) => players.1.score += points,
            None => return false,
        }
        frisbee.pos = Vector2::new(0.0, -4.0);
        frisbee.speed = 0.0;
        frisbee.direction = Vector2::zero();
        return true;
    }
    false
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
        radius: 0.5
    };
    let frisbee_bounds = Circle {
        center: frisbee.pos,
        radius: 0.5
    };

    let d2 = (player_bounds.center.x - frisbee_bounds.center.x).powf(2.0) + (player_bounds.center.y - frisbee_bounds.center.y).powf(2.0);
    d2 <= (player_bounds.radius + frisbee_bounds.radius).powf(2.0)
}
