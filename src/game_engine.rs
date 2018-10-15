use frisbee::Frisbee;
use player::{ Player, PlayerSide };

use std::mem::transmute;

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

pub struct GameEngine {
    pub p1:      Player,
    pub p2:      Player,
    pub frisbee: Frisbee,
}

impl GameEngine {
    #[no_mangle]
    pub extern fn initialize() -> *mut Self {
        unsafe { transmute(Box::new(Self::new())) }
    }

    #[no_mangle]
    pub extern fn dispose(ptr: *mut Self) {
        let _state: Box<Self> = unsafe { transmute(ptr) };
    }

    pub fn new() -> Self {
        Self {
            p1:      Player::new(),
            p2:      Player::new(),
            frisbee: Frisbee::new()
        }
    }

    #[no_mangle]
    pub extern fn reset(&mut self) {
        self.p1.pos.x = -10.0;
        self.p1.pos.y = 0.0;
        self.p1.score = 0;
        self.p1.side = Some(PlayerSide::Left);
        
        self.p2.pos.x = 10.0;
        self.p2.pos.y = 0.0;
        self.p2.score = 0;
        self.p2.side = Some(PlayerSide::Right);

        self.frisbee.pos.x = 0.0;
        self.frisbee.pos.y = 0.0;
        self.frisbee.direction.x = 0.0;
        self.frisbee.direction.y = 0.0;
        self.frisbee.speed = 0.0;
    }

    #[no_mangle]
    pub extern fn epoch(&mut self) {
        
    
    }

    /*#[no_mangle]
    pub extern fn getState(&mut self) -> SharedData {
        let test: SharedData;
        game_state = self.to_shared_data(test);
        game_state
    }*/

    pub fn to_shared_data(self, shared_data: &mut SharedData) {
        shared_data.p1_x = self.p1.pos.x;
        shared_data.p1_y = self.p1.pos.y;
        shared_data.p1_score = self.p1.score;
        shared_data.p1_side = 0;
        
        shared_data.p2_x = self.p2.pos.x;
        shared_data.p2_y = self.p2.pos.y;
        shared_data.p2_score = self.p2.score;
        shared_data.p2_side = 1;

        shared_data.zbee_x = self.frisbee.pos.x;
        shared_data.zbee_y = self.frisbee.pos.y;
        shared_data.zbee_dir_x = self.frisbee.direction.x;
        shared_data.zbee_dir_y = self.frisbee.direction.y;
        shared_data.zbee_speed = self.frisbee.speed;
    }
}

impl SharedData {
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