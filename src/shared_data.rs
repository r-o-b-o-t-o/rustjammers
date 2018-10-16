use player::PlayerSide;
use game_engine::GameEngine;
use std::time::{Duration, Instant};
use game_engine::StateOfGame;



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

    pub time:       f64,
    pub state_of_game: i8,
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
            zbee_held:  -1,

            time:       0.0,

            state_of_game:    0,
        }
    }

    pub fn state_from_i8(state: i8) -> StateOfGame{
        match state {
            0 => StateOfGame::Start,
            1 => StateOfGame::Playing,
            _ => StateOfGame::End,
        }
    }

    pub fn to_game_engine(self, engine: &mut GameEngine) {
        engine.players.0.pos.x = self.p1_x;
        engine.players.0.pos.y = self.p1_y;
        engine.players.0.score = self.p1_score;
        engine.players.0.side = Some(PlayerSide::Left);

        engine.players.1.pos.x = self.p2_x;
        engine.players.1.pos.y = self.p2_y;
        engine.players.1.score = self.p2_score;
        engine.players.1.side = Some(PlayerSide::Right);

        engine.frisbee.pos.x = self.zbee_x;
        engine.frisbee.pos.y = self.zbee_y;
        engine.frisbee.held_by_player = ::player::player_side_from_i8(self.zbee_held);
        
        engine.time = Instant::now();

        engine.state_of_game= Self::state_from_i8(self.state_of_game);
    }
}
