use agent::Intent;
use player::PlayerSide;
use game_engine::GameEngine;

#[repr(C)]
pub struct SharedData {
    pub p1_x:       f64,
    pub p1_y:       f64,
    pub p1_score:   i8,
    pub p1_side:    i8,

    pub p2_x:       f64,
    pub p2_y:       f64,
    pub p2_score:   i8,
    pub p2_side:    i8,

    pub zbee_x:     f64,
    pub zbee_y:     f64,
    pub zbee_dir_x: f64,
    pub zbee_dir_y: f64,
    pub zbee_speed: f64,
}

impl SharedData {
    pub fn step(&mut self, p1_action: Intent, p2_action: Intent) {
        
    }

    pub fn new() -> Self {
        Self {
            p1_x:       0.0,
            p1_y:       0.0,
            p1_score:   0,
            p1_side:    0,

            p2_x:       0.0,
            p2_y:       0.0,
            p2_score:   0,
            p2_side:    0,

            zbee_x:     0.0,
            zbee_y:     0.0,
            zbee_dir_x: 0.0,
            zbee_dir_y: 0.0,
            zbee_speed: 0.0,
        }
    }

    pub fn to_game_engine(self, game_engine: &mut GameEngine) {
        game_engine.p1.pos.x = self.p1_x;
        game_engine.p1.pos.y = self.p1_y;
        game_engine.p1.score = self.p1_score;
        game_engine.p1.side = Some(PlayerSide::Left);
        
        game_engine.p2.pos.x = self.p2_x;
        game_engine.p2.pos.y = self.p2_y;
        game_engine.p2.score = self.p2_score;
        game_engine.p2.side = Some(PlayerSide::Right);

        game_engine.frisbee.pos.x = self.zbee_x;
        game_engine.frisbee.pos.y = self.zbee_y;
        game_engine.frisbee.direction.x = self.zbee_dir_x;
        game_engine.frisbee.direction.y = self.zbee_dir_y;
        game_engine.frisbee.speed = self.zbee_speed;

    }
}
