use vector2::Vector2;
use shared_data::SharedData;

pub enum AgentType {
    Random = 0,
    HumanPlayer,
    RandomRollout,
    Dijkstra,
    TabularQLearning
}

pub enum Intent {
    Move(Vector2),
    Dash(Vector2),
    Throw(::frisbee::ThrowDirection),
}

pub trait Agent {
    fn act(&mut self, state: &mut SharedData);
}

pub struct RandomAgent {}

impl Agent for RandomAgent {
    fn act(&mut self, state: &mut SharedData) {
        
    }
}
