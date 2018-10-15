use frisbee::Frisbee;
use player::{ Player, PlayerSide };

use std::mem::transmute;

pub struct SharedData {
    pub p1_x:       f64,
    pub p1_y:       f64,
    pub p1_score:   f64,

    pub p2_x:       f64,
    pub p2_y:       f64,
    pub p2_score:   f64,

    pub zbee_x:     f64,
    pub zbee_y:     f64,
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
}
