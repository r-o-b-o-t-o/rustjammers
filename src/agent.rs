extern crate rand;

use vector2::Vector2;
use player::PlayerSide;
use game_engine::GameEngine;

use self::rand::Rng;

pub enum AgentType {
    Random = 0,
    HumanPlayer,
    RandomRollout,
    Dijkstra,
    TabularQLearning
}

pub enum Intent {
    None,
    Move(Vector2),
    Dash(Vector2),
    Throw(::frisbee::ThrowDirection),
}

pub trait Agent {
    fn act(&mut self, side: &PlayerSide, engine: &mut GameEngine) -> Intent;
}

pub struct RandomAgent {}

impl Agent for RandomAgent {
    fn act(&mut self, side: &PlayerSide, engine: &mut GameEngine) -> Intent {
        let intent = Intent::None;

        intent
    }
}
