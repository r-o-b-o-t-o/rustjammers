use vector2::Vector2;
use player::PlayerSide;
use frisbee::ThrowDirection;
use game_engine::{ GameEngine, StateOfGame };

use rand::Rng;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy, Debug)]
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

fn simulation(engine: &mut GameEngine, side: &PlayerSide, intent: Intent) -> (i8, Intent) {
    // Author: Created by Yohann
    let intents = match *side {
        PlayerSide::Left => (intent, Intent::None),
        PlayerSide::Right => (Intent::None, intent),
    };

    engine.step(intents);

    for _i in 0..1_000 {
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
    fn act(&mut self, side: PlayerSide, engine: &mut GameEngine) -> Intent;
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
    fn act(&mut self, side: PlayerSide, engine: &mut GameEngine) -> Intent {
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

pub fn human_intent_to_index(val: HumanIntent) -> u8 {
    if val == HumanIntent::UP { return 1; }
    if val == HumanIntent::DOWN { return 2; }
    if val == HumanIntent::LEFT { return 3; }
    if val == HumanIntent::RIGHT { return 4; }
    if val == HumanIntent::UP | HumanIntent::LEFT { return 5; }
    if val == HumanIntent::UP | HumanIntent::RIGHT { return 6; }
    if val == HumanIntent::DOWN | HumanIntent::LEFT { return 7; }
    if val == HumanIntent::DOWN | HumanIntent::RIGHT { return 8; }
    if val == HumanIntent::THROW | HumanIntent::UP { return 9; }
    if val == HumanIntent::THROW | HumanIntent::DOWN { return 10; }
    if val == HumanIntent::THROW | HumanIntent::LEFT { return 11; }
    if val == HumanIntent::THROW | HumanIntent::RIGHT { return 12; }
    if val == HumanIntent::THROW | HumanIntent::UP | HumanIntent::LEFT { return 13; }
    if val == HumanIntent::THROW | HumanIntent::UP | HumanIntent::RIGHT { return 14; }
    if val == HumanIntent::THROW | HumanIntent::DOWN | HumanIntent::LEFT { return 15; }
    if val == HumanIntent::THROW | HumanIntent::DOWN | HumanIntent::RIGHT { return 16; }
    0
}

pub fn human_intent_from_index(idx: u8) -> HumanIntent {
    if idx == 1 { return HumanIntent::UP; }
    if idx == 2 { return HumanIntent::DOWN; }
    if idx == 3 { return HumanIntent::LEFT; }
    if idx == 4 { return HumanIntent::RIGHT; }
    if idx == 5 { return HumanIntent::UP | HumanIntent::LEFT; }
    if idx == 6 { return HumanIntent::UP | HumanIntent::RIGHT; }
    if idx == 7 { return HumanIntent::DOWN | HumanIntent::LEFT; }
    if idx == 8 { return HumanIntent::DOWN | HumanIntent::RIGHT; }
    if idx == 9 { return HumanIntent::THROW | HumanIntent::UP; }
    if idx == 10 { return HumanIntent::THROW | HumanIntent::DOWN; }
    if idx == 11 { return HumanIntent::THROW | HumanIntent::LEFT; }
    if idx == 12 { return HumanIntent::THROW | HumanIntent::RIGHT; }
    if idx == 13 { return HumanIntent::THROW | HumanIntent::UP | HumanIntent::LEFT; }
    if idx == 14 { return HumanIntent::THROW | HumanIntent::UP | HumanIntent::RIGHT; }
    if idx == 15 { return HumanIntent::THROW | HumanIntent::DOWN | HumanIntent::LEFT; }
    if idx == 16 { return HumanIntent::THROW | HumanIntent::DOWN | HumanIntent::RIGHT; }
    HumanIntent::IDLE
}

pub fn human_intent_to_intent(engine: &GameEngine, input: HumanIntent, side: PlayerSide) -> Intent {
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

impl Agent for HumanPlayerAgent {
    // Author: Created by Yohann / Edited by Axel
    fn get_type(&self) -> AgentType {
        AgentType::HumanPlayer
    }
    fn act(&mut self, side: PlayerSide, engine: &mut GameEngine) -> Intent {
        let input = match side {
            PlayerSide::Left => engine.inputs.0,
            PlayerSide::Right => engine.inputs.1,
        };
        human_intent_to_intent(engine, input, side)
    }
}

pub struct RandomRolloutAgent {}

impl Agent for RandomRolloutAgent {
    // Author: Created by Yohann / Edited by Axel
    fn get_type(&self) -> AgentType {
        AgentType::RandomRollout
    }
    fn act(&mut self, side: PlayerSide, engine: &mut GameEngine) -> Intent {
        let mut prev = (0, Intent::None);
        let mut new_engine = GameEngine::new();
        let player = match side {
            PlayerSide::Left => &engine.players.0,
            PlayerSide::Right => &engine.players.1,
        };

        fn run_simulation(prev: &mut (i8, Intent), engine: &GameEngine, new_game_engine: &mut GameEngine, side: &PlayerSide, intent: Intent) {
            engine.copy_in(new_game_engine);
            let test = simulation(new_game_engine, side, intent);
            if prev.0 < test.0 {
                prev.0 = test.0;
                prev.1 = test.1;
            }
        }


        for _ in 0..3 {
            match engine.frisbee.held_by_player {
                Some(held_by) if held_by == side => {
                    // If the agent holds the frisbee
                    run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Throw(::frisbee::ThrowDirection::Up));
                    run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Throw(::frisbee::ThrowDirection::LightUp));
                    run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Throw(::frisbee::ThrowDirection::Middle));
                    run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Throw(::frisbee::ThrowDirection::LightDown));
                    run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Throw(::frisbee::ThrowDirection::Down));
                },
                _ => {
                    // If the agent doesn't hold the frisbee
                    if player.slide.is_none() {
                        // Movements are allowed only if the player is not dashing,
                        // so we're saving computing time if they are dashing

                        // TODO: use `human_intent_to_intent()` to replace the `Vector2::new`s with combined UP / DOWN / LEFT / RIGHT.
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(0.0, 1.0)));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(0.0, -1.0)));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(-1.0, 0.0)));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(1.0, 0.0)));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(-1.0, -1.0).normalized()));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(-1.0, 1.0).normalized()));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(1.0, -1.0).normalized()));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Move(Vector2::new(1.0, 1.0).normalized()));

                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(0.0, 1.0)));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(0.0, -1.0)));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(-1.0, 0.0)));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(1.0, 0.0)));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(-1.0, -1.0).normalized()));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(-1.0, 1.0).normalized()));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(1.0, -1.0).normalized()));
                        run_simulation(&mut prev, &engine, &mut new_engine, &side, Intent::Dash(Vector2::new(1.0, 1.0).normalized()));
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
    fn act(&mut self, _side: PlayerSide, _engine: &mut GameEngine) -> Intent {
        Intent::None
    }
}

pub struct TabularQLearningAgent {}
pub const QVALUES_ACTIONS: usize = 17;
pub type QValues = HashMap<u64, ([f32; QVALUES_ACTIONS], [f32; QVALUES_ACTIONS])>;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionResult {
    None,
    Moved,
    Dashed,
    GrabbedFrisbee,
    Threw,
}

impl Agent for TabularQLearningAgent {
    fn get_type(&self) -> AgentType {
        AgentType::TabularQLearning
    }
    fn act(&mut self, side: PlayerSide, engine: &mut GameEngine) -> Intent {
        let mut rng = ::rand::thread_rng();
        let intent: HumanIntent;

        fn max_index(array: &[f32; QVALUES_ACTIONS]) -> usize {
            let mut idx = 0;

            for (key, &value) in array.iter().enumerate() {
                if value > array[idx] {
                    idx = key;
                }
            }

            idx
        }

        if rng.gen_range(0.0, 1.0) < engine.explo_rate {
            // Explore
            let intent_index = rng.gen_range(0, QVALUES_ACTIONS);
            intent = human_intent_from_index(intent_index as u8);
        } else {
            // Exploit
            let hash = engine.hash();
            let intent_index = match side {
                PlayerSide::Left => {
                    if engine.q_values.contains_key(&hash) {
                        max_index(&engine.q_values[&hash].0)
                    } else {
                        0
                    }
                },
                PlayerSide::Right => {
                    if engine.q_values.contains_key(&hash) {
                        max_index(&engine.q_values[&hash].1)
                    } else {
                        0
                    }
                },
            };
            intent = human_intent_from_index(intent_index as u8);
        }

        match side {
            PlayerSide::Left => {
                engine.inputs.0 = intent;
            },
            PlayerSide::Right => {
                engine.inputs.1 = intent;
            },
        };

        human_intent_to_intent(engine, intent, side)
    }
}

pub fn get_blank_q_values() -> QValues {
    let size: u64 = 82763; // This is the `max_value` printed from GameEngine::hash()
    let mut map = QValues::with_capacity(size as usize);

    for i in 0..size {
        map.insert(i, ([0.0; QVALUES_ACTIONS], [0.0; QVALUES_ACTIONS]));
    }

    map
}
