use player::Player;
use frisbee::Frisbee;

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
