use vector2::Vector2;
use player::PlayerSide;
use game_engine::GameEngine;

use rand::Rng;

pub enum AgentType {
    Random = 0,
    HumanPlayer,
    RandomRollout,
    Dijkstra,
    TabularQLearning,
    None
}

pub enum Intent {
    None,
    Move(Vector2),
    Dash(Vector2),
    Throw(::frisbee::ThrowDirection),
}

pub fn agent_type_from_i8(side: i8) -> AgentType {
    match side {
        0 => AgentType::Random,
        1 => AgentType::HumanPlayer,
        2 => AgentType::RandomRollout,
        3 => AgentType::Dijkstra,
        4 => AgentType::TabularQLearning,
        _ => AgentType::None
    }
}


pub trait Agent {
    fn act(&mut self, side: PlayerSide, engine: &GameEngine) -> Intent;
    fn get_type(&self) -> AgentType;

    fn get_random_direction(&self) -> Vector2 {
        let mut rng = ::rand::thread_rng();
        let dir = Vector2::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0)
        );
        dir.normalized()
    }
}

pub struct RandomAgent {}

impl Agent for RandomAgent {
    fn get_type(&self) -> AgentType{
        AgentType::Random
    }
    fn act(&mut self, side: PlayerSide, engine: &GameEngine) -> Intent {
        let mut rng = ::rand::thread_rng();

        match engine.frisbee.held_by_player {
            Some(held_side) if held_side == side => {
                // The agent holds the frisbee
                let rand = rng.gen_range(0.0, 1.0);
                if rand < 0.7 {
                    // Throw
                    return Intent::Throw(::frisbee::random_throw_direction());
                } else if rand < 0.8 {
                    // Dash, throw later
                    let dir = self.get_random_direction();
                    return Intent::Dash(dir);
                } else if rand < 0.9 {
                    // Move, throw later
                    let dir = self.get_random_direction();
                    return Intent::Move(dir);
                } else {
                    // Wait, throw later
                }
            },
            _ => {
                // The agent does not hold the frisbee
                let rand = rng.gen_range(0.0, 1.0);
                if rand < 0.7 {
                    // Move
                    let dir = self.get_random_direction();
                    return Intent::Move(dir);
                } else if rand < 0.85 {
                    // Dash
                    let dir = self.get_random_direction();
                    return Intent::Dash(dir);
                } else {
                    // Wait
                }
            }
        };

        Intent::None
    }
}

pub struct HumanPlayerAgent {}

impl Agent for HumanPlayerAgent {
    fn get_type(&self) -> AgentType{
        AgentType::HumanPlayer
    }
    fn act(&mut self, side: PlayerSide, engine: &GameEngine) -> Intent {
        match side {
            PlayerSide::Left => {
                match engine.inputs.0 {
                    0 => Intent::None,
                    1 => Intent::Move(Vector2::new(0.0, 1.0)),//up
                    2 => Intent::Move(Vector2::new(0.0, -1.0)),//Down
                    4 => Intent::Move(Vector2::new(-1.0, 0.0)),//Left
                    5 => Intent::Move(Vector2::new(-1.0, 1.0).normalized()),//Up + Left
                    6 => Intent::Move(Vector2::new(-1.0, -1.0).normalized()),//Down + Left
                    8 => Intent::Move(Vector2::new(1.0, 0.0)),//Right
                    9 => Intent::Move(Vector2::new(1.0, 1.0).normalized()),//Up + Right
                    10 => Intent::Move(Vector2::new(1.0, -1.0).normalized()),//Down + Right
                    //Dash = 16,
                    32 => Intent::Throw(::frisbee::ThrowDirection::Middle),//Throw
                    33 => Intent::Throw(::frisbee::ThrowDirection::Up),//Throw up
                    34 => Intent::Throw(::frisbee::ThrowDirection::Down),//Throw down
                    40 => Intent::Throw(::frisbee::ThrowDirection::Middle),//Throw right
                    _ => Intent::None
                }
            },
            PlayerSide::Right => {
                match engine.inputs.1 {
                    0 => Intent::None,
                    1 => Intent::Move(Vector2::new(0.0, 1.0)),//up
                    2 => Intent::Move(Vector2::new(0.0, -1.0)),//Down
                    4 => Intent::Move(Vector2::new(-1.0, 0.0)),//Left
                    5 => Intent::Move(Vector2::new(-1.0, 1.0).normalized()),//Up + Left
                    6 => Intent::Move(Vector2::new(-1.0, -1.0).normalized()),//Down + Left
                    8 => Intent::Move(Vector2::new(1.0, 0.0)),//Right
                    9 => Intent::Move(Vector2::new(1.0, 1.0).normalized()),//Up + Right
                    10 => Intent::Move(Vector2::new(1.0, -1.0).normalized()),//Down + Right
                    //Dash = 16,
                    32 => Intent::Throw(::frisbee::ThrowDirection::Middle),//Throw
                    33 => Intent::Throw(::frisbee::ThrowDirection::Up),//Throw up
                    34 => Intent::Throw(::frisbee::ThrowDirection::Down),//Throw down
                    36 => Intent::Throw(::frisbee::ThrowDirection::Middle),//Throw left
                    _ => Intent::None
                }}
        }
    }
}

pub struct RandomRolloutAgent {}

impl Agent for RandomRolloutAgent {
    fn get_type(&self) -> AgentType{
        AgentType::RandomRollout
    }
    fn act(&mut self, _side: PlayerSide, _engine: &GameEngine) -> Intent {
        Intent::None
    }
}

pub struct DijkstraAgent {}

impl Agent for DijkstraAgent {
    fn get_type(&self) -> AgentType{
        AgentType::Dijkstra
    }
    fn act(&mut self, _side: PlayerSide, _engine: &GameEngine) -> Intent {
        Intent::None
    }
}

pub struct TabularQLearningAgent {}

impl Agent for TabularQLearningAgent {
    fn get_type(&self) -> AgentType{
        AgentType::TabularQLearning
    }
    fn act(&mut self, _side: PlayerSide, _engine: &GameEngine) -> Intent {
        Intent::None
    }
}
