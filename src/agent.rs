use vector2::Vector2;
use player::PlayerSide;
use frisbee::ThrowDirection;
use game_engine::{ GameEngine, StateOfGame };

use rand::Rng;

pub enum AgentType {
    HumanPlayer = 0,
    Random,
    RandomRollout,
    Dijkstra,
    TabularQLearning,
    None
}

#[derive(Clone, Copy)]
pub enum Intent {
    // Author: Created by Axel
    None,
    Move(Vector2),
    Dash(Vector2),
    Throw(::frisbee::ThrowDirection),
}

fn simulation(engine: &mut GameEngine, side: &PlayerSide, intent: Intent, nb_frames : f64) -> (i8, Intent) {
    // Author: Created by Yohann
    let intents = match *side {
        PlayerSide::Left => (intent, Intent::None),
        PlayerSide::Right => (Intent::None, intent),
    };

    engine.step(intents);

    for _i in 0..nb_frames as i16 {
        engine.epoch(HumanIntent::IDLE, HumanIntent::IDLE);
        if engine.state_of_game != StateOfGame::Playing {
            break;
        }
    }

    let score = match side {
        PlayerSide::Left => engine.players.0.score,
        PlayerSide::Right => engine.players.1.score,
    };

    (score, intent)
}

pub fn agent_type_from_i8(side: i8) -> AgentType {
    // Author: Created by Axel
    match side {
        0 => AgentType::HumanPlayer,
        1 => AgentType::Random,
        2 => AgentType::RandomRollout,
        3 => AgentType::Dijkstra,
        4 => AgentType::TabularQLearning,
        _ => AgentType::None
    }
}

pub trait Agent {
    // Author: Created by Axel
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
    // Author: Created by Axel
    fn get_type(&self) -> AgentType {
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

bitflags! {
    pub struct HumanIntent: u8 {
        // Author: Created by Axel
        const IDLE  = 0;
        const UP    = 1;
        const DOWN  = 2;
        const LEFT  = 4;
        const RIGHT = 8;
        const THROW = 16;
    }
}

impl Agent for HumanPlayerAgent {
    // Author: Created by Yohann / Edited by Axel
    fn get_type(&self) -> AgentType {
        AgentType::HumanPlayer
    }
    fn act(&mut self, side: PlayerSide, engine: &GameEngine) -> Intent {
        let input = match side {
            PlayerSide::Left => engine.inputs.0,
            PlayerSide::Right => engine.inputs.1,
        };
        let has_frisbee = match engine.frisbee.held_by_player {
            Some(held_by) if held_by == side => true,
            _ => false,
        };

        let mut dir = Vector2::zero();
        if input.contains(HumanIntent::UP) {
            dir.y = 1.0;
        }
        if input.contains(HumanIntent::DOWN) {
            dir.y = -1.0;
        }
        if input.contains(HumanIntent::LEFT) {
            dir.x = -1.0;
        }
        if input.contains(HumanIntent::RIGHT) {
            dir.x = 1.0;
        }
        dir.normalize();

        if input.contains(HumanIntent::THROW) {
            if has_frisbee {
                let mut throw_dir = ThrowDirection::Middle;
                if input.contains(HumanIntent::UP) {
                    if (input.contains(HumanIntent::RIGHT) && side == PlayerSide::Left) ||
                       (input.contains(HumanIntent::LEFT) && side == PlayerSide::Right) {
                        throw_dir = ThrowDirection::LightUp;
                    } else {
                        throw_dir = ThrowDirection::Up;
                    }
                } else if input.contains(HumanIntent::DOWN) {
                    if (input.contains(HumanIntent::RIGHT) && side == PlayerSide::Left) ||
                       (input.contains(HumanIntent::LEFT) && side == PlayerSide::Right) {
                        throw_dir = ThrowDirection::LightDown;
                    } else {
                        throw_dir = ThrowDirection::Down;
                    }
                }
                Intent::Throw(throw_dir)
            } else {
                Intent::Dash(dir)
            }
        } else {
            if dir.x == 0.0 && dir.y == 0.0 {
                Intent::None
            } else {
                Intent::Move(dir)
            }
        }
    }
}

pub struct RandomRolloutAgent {pub frames : f64,pub sim: i8}

impl Agent for RandomRolloutAgent {
    // Author: Created by Yohann / Edited by Axel
    fn get_type(&self) -> AgentType {
        AgentType::RandomRollout
    }
    fn act(&mut self, side: PlayerSide, engine: &GameEngine) -> Intent {
        let mut prev = (0, Intent::None);
        let mut new_engine = GameEngine::new();
        let player = match side {
            PlayerSide::Left => &engine.players.0,
            PlayerSide::Right => &engine.players.1,
        };

        fn run_simulation(prev: &mut (i8, Intent), engine: &GameEngine, new_game_engine: &mut GameEngine, side: &PlayerSide, intent: Intent,frames: f64) {
            engine.copy_in(new_game_engine);
            let test = simulation(new_game_engine, side, intent,frames);
            if prev.0 < test.0 {
                prev.0 = test.0;
                prev.1 = test.1;
            }
        }


        for _ in 0..self.sim {
            match engine.frisbee.held_by_player {
                Some(held_by) if held_by == side => {
                    // If the agent holds the frisbee
                    run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Throw(::frisbee::ThrowDirection::Up),self.frames);
                    run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Throw(::frisbee::ThrowDirection::LightUp),self.frames);
                    run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Throw(::frisbee::ThrowDirection::Middle),self.frames);
                    run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Throw(::frisbee::ThrowDirection::LightDown),self.frames);
                    run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Throw(::frisbee::ThrowDirection::Down),self.frames);
                },
                _ => {
                    // If the agent doesn't hold the frisbee
                    if player.slide.is_none() {
                        // Movements are allowed only if the player is not dashing,
                        // so we're saving computing time if they are dashing

                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(0.0, 1.0)),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(0.0, -1.0)),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(-1.0, 0.0)),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(1.0, 0.0)),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(-1.0, -1.0).normalized()),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(-1.0, 1.0).normalized()),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(1.0, -1.0).normalized()),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(1.0, 1.0).normalized()),self.frames);

                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(0.0, 1.0)),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(0.0, -1.0)),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(-1.0, 0.0)),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(1.0, 0.0)),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(-1.0, -1.0).normalized()),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(-1.0, 1.0).normalized()),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(1.0, -1.0).normalized()),self.frames);
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(1.0, 1.0).normalized()),self.frames);
                    }
                }
            };
        }

        prev.1
    }
}

pub struct DijkstraAgent {}

impl Agent for DijkstraAgent {
    fn get_type(&self) -> AgentType {
        AgentType::Dijkstra
    }
    fn act(&mut self, _side: PlayerSide, _engine: &GameEngine) -> Intent {
        Intent::None
    }
}

pub struct TabularQLearningAgent {}

impl Agent for TabularQLearningAgent {
    fn get_type(&self) -> AgentType {
        AgentType::TabularQLearning
    }
    fn act(&mut self, _side: PlayerSide, _engine: &GameEngine) -> Intent {
        Intent::None
    }
}
