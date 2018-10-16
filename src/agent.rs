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

#[derive(Clone, Copy)]
pub enum Intent {
    None,
    Move(Vector2),
    Dash(Vector2),
    Throw(::frisbee::ThrowDirection),
}

fn simulation(engine: &mut GameEngine, side: PlayerSide, intent: Intent) -> (i8, Intent) {

    let intents = match side {
        PlayerSide::Left => (intent, Intent::None),
        PlayerSide::Right => (Intent::None, intent),
    };

    engine.step(intents);

    for _i in 0..300 {
        engine.epoch(0, 0);
    }

    let score = match side {
        PlayerSide::Left => engine.players.0.score - engine.players.1.score,
        PlayerSide::Right => engine.players.1.score - engine.players.0.score,
    };

    (score, intent)
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
                if rand < 0.25 {
                    // Throw
                    return Intent::Throw(::frisbee::random_throw_direction());
                } else {
                    // Wait, throw later
                }
            },
            _ => {
                // The agent does not hold the frisbee
                let rand = rng.gen_range(0.0, 1.0);
                if rand < 0.5 {
                    // Move
                    let dir = self.get_random_direction();
                    return Intent::Move(dir);
                } else if rand < 0.6 {
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
                    16 => Intent::Throw(::frisbee::ThrowDirection::Middle),//Throw
                    17 => Intent::Throw(::frisbee::ThrowDirection::Up),//Throw up
                    18 => Intent::Throw(::frisbee::ThrowDirection::Down),//Throw down
                    24 => Intent::Throw(::frisbee::ThrowDirection::Middle),//Throw right
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
                    16 => Intent::Throw(::frisbee::ThrowDirection::Middle),//Throw
                    17 => Intent::Throw(::frisbee::ThrowDirection::Up),//Throw up
                    18 => Intent::Throw(::frisbee::ThrowDirection::Down),//Throw down
                    20 => Intent::Throw(::frisbee::ThrowDirection::Middle),//Throw left
                    _ => Intent::None
                }
            }
        }
    }
}

pub struct RandomRolloutAgent {}

impl Agent for RandomRolloutAgent {
    fn get_type(&self) -> AgentType{
        AgentType::RandomRollout
    }
    fn act(&mut self, side: PlayerSide, engine: &GameEngine) -> Intent {
        let mut prev = (0, Intent::None);
        let mut new_game_engine = GameEngine::new();
        engine.get_engine(&mut new_game_engine);


        let test = simulation(&mut new_game_engine, side, Intent::Move(Vector2::new(0.0, 1.0)));
        if prev.0 < test.0 { prev = test; }

        let test = simulation(&mut new_game_engine, side, Intent::Move(Vector2::new(0.0, -1.0)));
        if prev.0 < test.0 { prev = test; }

        let test = simulation(&mut new_game_engine, side, Intent::Move(Vector2::new(-1.0, 0.0)));
        if prev.0 < test.0 { prev = test; }

        let test = simulation(&mut new_game_engine, side, Intent::Move(Vector2::new(1.0, 0.0)));
        if prev.0 < test.0 { prev = test; }

        let test = simulation(&mut new_game_engine, side, Intent::Throw(::frisbee::ThrowDirection::Up));
        if prev.0 < test.0 { prev = test; }

        let test = simulation(&mut new_game_engine, side, Intent::Throw(::frisbee::ThrowDirection::Middle));
        if prev.0 < test.0 { prev = test; }

        let test = simulation(&mut new_game_engine, side, Intent::Throw(::frisbee::ThrowDirection::Down));
        if prev.0 < test.0 { prev = test; }


        prev.1
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
