use vector2::Vector2;
use frisbee::Frisbee;
use shared_data::SharedData;
use player::{ Player, PlayerSide };
use agent::{ Intent, AgentType, Agent, RandomAgent, HumanPlayerAgent, RandomRolloutAgent, DijkstraAgent, TabularQLearningAgent, QValues, HumanIntent, ActionResult };

use rand::Rng;

pub const MAX_ROUND_POINTS: i8       = 30;
pub const MAX_ROUND_TIME: f64        = 60.0;
pub const INITIAL_THROW_TIME: f64    = 2.0;
pub const INITIAL_FRISBEE_SPEED: f64 = 2.5;
pub const PLAYER_DASH_POWER: f64     = 2.5;

pub struct GameEngine {
    pub players:       (Player, Player),
    pub agents:        (Option<Box<Agent>>, Option<Box<Agent>>),
    pub frisbee:       Frisbee,
    pub time:          f64,
    pub start_time:    f64,
    pub state_of_game: StateOfGame,

    // Agent-specific fields
    pub inputs:        (HumanIntent, HumanIntent), // Human agent / Q-Learning
    pub q_values:      QValues, // Q-Learning
    pub rewards:       (f32, f32), // Q-Learning
    pub q_scored:      bool, // Q-Learning
    pub explo_rate:    f32, // Q-Learning
}

#[derive(Copy, Clone, PartialEq)]
pub enum StateOfGame {
    Start,
    Playing,
    End,
}

pub fn state_to_i8(state: &StateOfGame) -> i8 {
    *state as i8
}

pub fn state_from_i8(state: i8) -> StateOfGame {
    match state {
        0 => StateOfGame::Start,
        1 => StateOfGame::Playing,
        _ => StateOfGame::End,
    }
}

impl GameEngine {
    #[no_mangle]
    pub extern fn initialize() -> *mut Self {
        let boxed = Box::new(Self::new());
        Box::into_raw(boxed)
    }

    pub fn log(&self, s: &str) {
        use std::fs::OpenOptions;
        use std::io::prelude::*;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("rustjammers_debug.log")
            .unwrap();

        let mut s = String::from(s);
        s.push_str("\r\n");
        file.write(&s.into_bytes()).unwrap();
        file.flush().unwrap();
    }

    #[no_mangle]
    pub unsafe extern fn dispose(ptr: *mut Self) {
        if !ptr.is_null() {
            let _state: Box<Self> = Box::from_raw(ptr);
        }
    }

    pub fn new() -> Self {
        Self {
            players: (
                Player::new(),
                Player::new(),
            ),
            agents: (
                None,
                None,
            ),
            frisbee: Frisbee::new(),
            time: 0.0,
            start_time: 0.0,
            state_of_game: StateOfGame::Start,

            inputs: (
                HumanIntent::IDLE,
                HumanIntent::IDLE,
            ),
            q_values: QValues::new(),
            rewards: (0.0, 0.0),
            q_scored: false,
            explo_rate: 0.05,
        }
    }

    pub fn copy_in(&self, new_game_engine: &mut GameEngine) {
        new_game_engine.players = self.players;
        new_game_engine.agents = (
            Some(Box::new(RandomAgent {})),
            Some(Box::new(RandomAgent {}))
        );
        new_game_engine.frisbee = self.frisbee;
        new_game_engine.inputs = self.inputs;
        new_game_engine.time = self.time;
        new_game_engine.start_time = self.start_time;
        new_game_engine.state_of_game = self.state_of_game;
    }

    fn create_agent_from_type(agent_type: AgentType, frames: f64, sim: i8) -> Box<Agent> {
        match agent_type {
            AgentType::Random =>           Box::new(RandomAgent {}),
            AgentType::HumanPlayer =>      Box::new(HumanPlayerAgent {}),
            AgentType::RandomRollout =>    Box::new(RandomRolloutAgent {frames: frames,sim: sim}),
            AgentType::Dijkstra =>         Box::new(DijkstraAgent {}),
            AgentType::TabularQLearning => Box::new(TabularQLearningAgent {}),
            AgentType::None =>             panic!("Invalid agent type."),
        }
    }

    #[no_mangle]
    pub extern fn reset(&mut self) {
        self.players.0.pos.x = -9.0;
        self.players.0.pos.y = 0.0;
        self.players.0.score = 0;
        self.players.0.side = Some(PlayerSide::Left);

        self.players.1.pos.x = 9.0;
        self.players.1.pos.y = 0.0;
        self.players.1.score = 0;
        self.players.1.side = Some(PlayerSide::Right);

        self.frisbee.pos.x = 0.0;
        self.frisbee.pos.y = -4.0;
        self.frisbee.direction.x = 0.0;
        self.frisbee.direction.y = 0.0;
        self.frisbee.speed = 0.0;
        self.frisbee.last_held = None;
        self.frisbee.held_by_player = None;

        self.time = MAX_ROUND_TIME;
        self.start_time = 0.0;

        self.state_of_game = StateOfGame::Start;

        self.inputs = (HumanIntent::IDLE, HumanIntent::IDLE);
        self.rewards = (0.0, 0.0);
        self.q_scored = false;
    }

    #[no_mangle]
    pub extern fn send_type_p1(&mut self, agent_type: i8, frames: f64, sim: i8) {
        let t = ::agent::agent_type_from_i8(agent_type);
        self.agents.0 = Some(Self::create_agent_from_type(t, frames, sim));
        if t == AgentType::TabularQLearning {
            self.load_q_values();
        }
    }

    #[no_mangle]
    pub extern fn send_type_p2(&mut self, agent_type: i8, frames: f64, sim: i8) {
        let t = ::agent::agent_type_from_i8(agent_type);
        self.agents.1 = Some(Self::create_agent_from_type(t, frames, sim));
        if t == AgentType::TabularQLearning {
            self.load_q_values();
        }
    }

    fn load_q_values(&mut self) {
        if !self.q_values.is_empty() {
            return;
        }
        use ::std::fs::File;
        use ::std::io::BufReader;

        let f = match File::open("q_values.bin") {
            Ok(f) => f,
            _ => return,
        };
        let br = BufReader::new(f);
        self.q_values = ::bincode::deserialize_from(br).unwrap();
    }

    #[no_mangle]
    pub extern fn epoch(&mut self, p1_h_action: HumanIntent, p2_h_action: HumanIntent) {
        let mut a1 = self.agents.0.take().unwrap();
        let mut a2 = self.agents.1.take().unwrap();

        let input1 = match a1.get_type() {
            AgentType::HumanPlayer => p1_h_action,
            _ => HumanIntent::IDLE
        };

        let input2 = match a2.get_type() {
            AgentType::HumanPlayer => p2_h_action,
            _ => HumanIntent::IDLE
        };

        self.inputs = (input1, input2);
        let action_p1 = a1.act(PlayerSide::Left, self);
        let action_p2 = a2.act(PlayerSide::Right, self);

        self.agents = (
            Some(a1),
            Some(a2)
        );

        self.step((
            action_p1,
            action_p2
        ));
    }

    #[no_mangle]
    pub extern fn get_state(&mut self) -> SharedData {
        let mut data = SharedData::new();
        self.to_shared_data(&mut data);
        data
    }

    pub fn step(&mut self, intents: (Intent, Intent)) {
        // Update timers
        let time_step = 1.0 / 60.0; // Assume we run at 60 frames per second
        self.time -= time_step;
        self.start_time += time_step;

        // End game if one of the players reached the maximum score
        // or if the time runs out
        if self.players.0.score >= MAX_ROUND_POINTS ||
           self.players.1.score >= MAX_ROUND_POINTS ||
           self.time <= 0.0 {
           self.state_of_game = StateOfGame::End;
        }
        if self.state_of_game == StateOfGame::End {
            // We don't need to update the rest if the game just ended
            return;
        }

        // Start the round after waiting a bit for players to reset their positions
        if self.state_of_game == StateOfGame::Start && self.start_time >= 1.0 {
            // Resume the game
            self.state_of_game = StateOfGame::Playing;

            // If it is the first round, throw the frisbee at the player who lost the last round
            // Otherwise, target a random player
            let target = match self.frisbee.last_held {
                Some(ref last_held) => {
                    match last_held {
                        PlayerSide::Left => &self.players.1,
                        PlayerSide::Right => &self.players.0,
                    }
                },
                None => {
                    let mut rng = ::rand::thread_rng();
                    if rng.gen_range(0.0, 1.0) < 0.5 {
                        self.frisbee.last_held = Some(PlayerSide::Right);
                        &self.players.0
                    } else {
                        self.frisbee.last_held = Some(PlayerSide::Left);
                        &self.players.1
                    }
                },
            };
            // Set direction so that the frisbee arrives in the player's hands
            self.frisbee.direction = (target.pos + Vector2::new(target.get_horizontal_aim_direction(), 0.0) - self.frisbee.pos).normalized();
            self.frisbee.speed = INITIAL_FRISBEE_SPEED;
        }

        fn apply_action(player: &mut Player, frisbee: &mut Frisbee, intent: &Intent, state_of_game: &StateOfGame) -> ActionResult {
            let mut res = ActionResult::None;

            match intent {
                Intent::None => {},
                Intent::Move(dir) => {
                    if *state_of_game == StateOfGame::Playing {
                        if player.slide.is_none() { // Cannot move while dashing
                            match frisbee.held_by_player {
                                // Cannot move while holding frisbee
                                Some(held_by) if held_by == player.side.unwrap() => {},
                                _ => {
                                    player.pos += *dir * 0.1;
                                    res = ActionResult::Moved;
                                }
                            };
                        }
                    }
                },
                Intent::Dash(dir) => {
                    if *state_of_game == StateOfGame::Playing {
                        let dir = dir.normalized();
                        player.dash(dir * PLAYER_DASH_POWER);
                        res = ActionResult::Dashed;
                    }
                },
                Intent::Throw(dir) => {
                    match frisbee.held_by_player {
                        Some(held_by) if held_by == player.side.unwrap() => {
                            frisbee.direction = player.get_throw_vector(dir);
                            frisbee.speed = INITIAL_FRISBEE_SPEED;
                            frisbee.last_held = frisbee.held_by_player;
                            frisbee.held_by_player = None;
                            res = ActionResult::Threw;
                        },
                        _ => {}
                    };
                }
            };

            if *state_of_game == StateOfGame::Playing {
                // We check the state of game to prevent grabbing the frisbee before it is initially thrown (Start state)
                match frisbee.held_by_player {
                    None if ::collision::player_collides_with_frisbee(player, frisbee) => {
                        // Grab frisbee if the player collides with it
                        frisbee.held_by_player = player.side;
                        res = ActionResult::GrabbedFrisbee;
                    },
                    _ => {}
                };
            }
            if player.slide.is_some() {
                let slide = player.slide.unwrap();
                player.pos += slide.dir * 4.0 * 0.1;
                if slide.has_reached_goal(&player.pos) {
                    player.pos = slide.target;
                    player.slide = None;
                }
            }

            res
        }

        fn apply_action_rewards_to_q_agent(action_result: ActionResult, reward: &mut f32) -> bool {
            match action_result {
                ActionResult::None => *reward = -1.0,
                ActionResult::Moved => *reward = -1.0,
                ActionResult::Dashed => *reward = -5.0,
                ActionResult::GrabbedFrisbee => *reward = 0.0,
                ActionResult::Threw => {
                    *reward = 0.0;
                    return true;
                },
            };
            false
        }

        fn reward_q_for_goal(engine: &mut GameEngine) {
            let a1 = engine.agents.0.take().unwrap();
            let a2 = engine.agents.1.take().unwrap();

            match engine.frisbee.last_held {
                Some(PlayerSide::Left) => {
                    if a2.get_type() == AgentType::TabularQLearning {
                        engine.rewards.1 = -100.0;
                    }
                },
                Some(PlayerSide::Right) => {
                    if a1.get_type() == AgentType::TabularQLearning {
                        engine.rewards.0 = -100.0;
                    }
                },
                _ => {}
            };

            engine.agents = (Some(a1), Some(a2));
            engine.q_scored = true;
        }

        let a1 = self.agents.0.take().unwrap();
        let res = apply_action(&mut self.players.0, &mut self.frisbee, &intents.0, &self.state_of_game);
        if a1.get_type() == AgentType::TabularQLearning {
            apply_action_rewards_to_q_agent(res, &mut self.rewards.0);
        }

        let a2 = self.agents.1.take().unwrap();
        let res = apply_action(&mut self.players.1, &mut self.frisbee, &intents.1, &self.state_of_game);
        if a2.get_type() == AgentType::TabularQLearning {
            apply_action_rewards_to_q_agent(res, &mut self.rewards.1);
        }

        self.agents = (Some(a1), Some(a2));

        match self.frisbee.held_by_player {
            Some(held_by) => {
                match held_by {
                    // Snap frisbee to player hands
                    PlayerSide::Left => self.frisbee.pos = self.players.0.pos,
                    PlayerSide::Right => self.frisbee.pos = self.players.1.pos
                };
            },
            None => {
                if self.frisbee.speed != 0.0 {
                    self.frisbee.pos += self.frisbee.direction * self.frisbee.speed * 0.1;
                }
            },
        };

        let collided = ::collision::player_collision(&mut self.players.0);
        if collided {
            // Cancels slide if the player hits an obstacle to prevent being stuck
            self.players.0.slide = None;
        }
        let collided = ::collision::player_collision(&mut self.players.1);
        if collided {
            // Cancels slide if the player hits an obstacle to prevent being stuck
            self.players.1.slide = None;
        }

        ::collision::frisbee_collision_wall(&mut self.frisbee);
        let goal = ::collision::frisbee_collision_goal(&mut self.frisbee, &mut self.players);
        if goal {
            self.state_of_game = StateOfGame::Start;
            self.start_time = 0.0;
            self.players.0.dash_to_pos(Vector2::new(-9.0, 0.0));
            self.players.1.dash_to_pos(Vector2::new(9.0, 0.0));

            reward_q_for_goal(self);
        }
    }

    pub fn to_shared_data(&self, shared: &mut SharedData) {
        shared.p1_x = self.players.0.pos.x;
        shared.p1_y = self.players.0.pos.y;
        shared.p1_score = self.players.0.score;
        shared.p1_side = 0;

        shared.p2_x = self.players.1.pos.x;
        shared.p2_y = self.players.1.pos.y;
        shared.p2_score = self.players.1.score;
        shared.p2_side = 1;

        shared.zbee_x = self.frisbee.pos.x;
        shared.zbee_y = self.frisbee.pos.y;
        shared.zbee_held = ::player::player_side_to_i8(self.frisbee.held_by_player);

        shared.time = self.time;

        shared.state_of_game = state_to_i8(&self.state_of_game);
    }

    pub fn hash(&self) -> u64 {
        fn set_state(hash: &mut u64, val: f64, min: i64, max: i64, scale: f64, amplitudes: &mut Vec<u32>, max_value: &mut u64) {
            fn discretize(val: f64, min: i64, max: i64, scale: f64) -> (u32, u32) {
                let min = min as f64 * scale;
                let max = max as f64 * scale;
                let val = val * scale;
                let amplitude = (max - min + 1.0) as u32;
                let res = ((val.round() + min.abs()) as u32) % amplitude;
                (res, amplitude)
            }

            let mut factor = 1;
            for a in amplitudes.iter() {
                factor *= *a;
            }
            let (val, _) = discretize(val, min, max, scale);
            *hash += (val * factor) as u64;
            let (val, amplitude) = discretize(max as f64, min, max, scale);
            *max_value += (val * factor) as u64;
            amplitudes.push(amplitude);
        }

        fn discretize_frisbee_direction(v: Vector2) -> f64 {
            fn angle(v: Vector2) -> f64 {
                (v.y / v.x.abs()).sin().to_degrees()
            }

            let a = angle(v);
            if a > 40.0 {
                0.0
            } else if a > 10.0 {
                1.0
            } else if a < 10.0 && a > -10.0 {
                2.0
            } else if a < -10.0 && a > -40.0 {
                3.0
            } else {
                4.0
            }
        }

        let mut val = 0;
        let mut max_value = 0;
        let mut amplitudes: Vec<u32> = Vec::new();

        let scale = 1.0;
        set_state(&mut val, self.players.0.pos.x, -9, -1, scale, &mut amplitudes, &mut max_value);
        set_state(&mut val, self.players.0.pos.y, -5, 5, scale, &mut amplitudes, &mut max_value);

        set_state(&mut val, self.frisbee.pos.x, -9, 9, scale, &mut amplitudes, &mut max_value);
        set_state(&mut val, self.frisbee.pos.y, -5, 5, scale, &mut amplitudes, &mut max_value);

        set_state(&mut val, match self.frisbee.last_held {
            Some(side) => match side {
                PlayerSide::Left => 1.0,
                PlayerSide::Right => 0.0,
            },
            None => 0.0
        }, 0, 1, 1.0, &mut amplitudes, &mut max_value);

        set_state(&mut val, discretize_frisbee_direction(self.frisbee.direction), 0, 4, 1.0, &mut amplitudes, &mut max_value);

        val
    }
}
