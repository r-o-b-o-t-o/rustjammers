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
    pub zbee_held:  i8,
}

impl SharedData {
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
            zbee_held:  -1
        }
    }

    pub fn to_game_engine(self, engine: &mut GameEngine) {
        engine.p1.pos.x = self.p1_x;
        engine.p1.pos.y = self.p1_y;
        engine.p1.score = self.p1_score;
        engine.p1.side = Some(PlayerSide::Left);
        
        engine.p2.pos.x = self.p2_x;
        engine.p2.pos.y = self.p2_y;
        engine.p2.score = self.p2_score;
        engine.p2.side = Some(PlayerSide::Right);

        engine.frisbee.pos.x = self.zbee_x;
        engine.frisbee.pos.y = self.zbee_y;
        engine.frisbee.held_by_player = ::player::player_side_from_i8(self.zbee_held);
    }
}
