use vector2::Vector2;
use game_engine::GameEngine;

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
    fn act(&mut self, state: &mut GameEngine);
}

pub struct RandomAgent {}

impl Agent for RandomAgent {
    fn act(&mut self, state: &mut GameEngine) {
        
    }
}
