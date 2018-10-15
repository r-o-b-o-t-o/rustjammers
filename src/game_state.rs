use player::Player;
use frisbee::Frisbee;

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

pub struct GameState {
    pub p1:      Player,
    pub p2:      Player,
    pub frisbee: Frisbee,
}

impl GameState {
    #[no_mangle]
    pub extern fn initialize() -> *mut Self {
        unsafe { transmute(Box::new(Self::new())) }
    }

    #[no_mangle]
    pub extern fn dispose(ptr: *mut Self) {
        let _state: Box<Self> = unsafe { transmute(ptr) };
    }

    fn new() -> Self {
        Self {
            p1:      Player::new(),
            p2:      Player::new(),
            frisbee: Frisbee::new()
        }
    }
}
