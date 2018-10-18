use vector2::Vector2;
use frisbee::Frisbee;
use shared_data::SharedData;
use player::{ Player, PlayerSide };
use agent::{ Intent, AgentType, Agent, RandomAgent, HumanPlayerAgent, RandomRolloutAgent, DijkstraAgent, TabularQLearningAgent, HumanIntent };

use rand::Rng;
use std::mem::transmute;

pub const MAX_ROUND_POINTS: i8       = 30;
pub const MAX_ROUND_TIME: f64        = 60.0;
pub const INITIAL_THROW_TIME: f64    = 2.0;
pub const INITIAL_FRISBEE_SPEED: f64 = 2.5;
pub const PLAYER_DASH_POWER: f64     = 2.5;

pub struct GameEngine {
    // Author: Created by Axel / Edited by all
    pub players:       (Player, Player),
    pub agents:        (Option<Box<Agent>>, Option<Box<Agent>>),
    pub frisbee:       Frisbee,
    pub time:          f64,
    pub start_time:    f64,
    pub state_of_game: StateOfGame,

    // Agent-specific fields
    pub inputs:        (HumanIntent, HumanIntent), // Human agent
}

#[derive(Copy, Clone, PartialEq)]
pub enum StateOfGame {
    // Author: Created by Esteban
    Start,
    Playing,
    End,
}

pub fn state_to_i8(state: &StateOfGame) -> i8 {
    // Author: Created by Esteban
    *state as i8
}

pub fn state_from_i8(state: i8) -> StateOfGame {
    // Author: Created by Esteban
    match state {
        0 => StateOfGame::Start,
        1 => StateOfGame::Playing,
        _ => StateOfGame::End,
    }
}

impl GameEngine {
    #[no_mangle]
    pub extern fn initialize() -> *mut Self {
        // Author: Created by Axel
        unsafe { transmute(Box::new(Self::new())) }
    }

    #[no_mangle]
    pub extern fn dispose(ptr: *mut Self) {
        // Author: Created by Axel
        let _state: Box<Self> = unsafe { transmute(ptr) };
    }

    pub fn new() -> Self {
        // Author: Created by Axel / Edited by all
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
        }
    }

    pub fn copy_in(&self, new_game_engine: &mut GameEngine) {
        // Author: Created by Yohann / Edited by all
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

    fn create_agent_from_type(agent_type: AgentType) -> Box<Agent> {
        // Author: Created by Yohann / Edited by Axel
        match agent_type {
            AgentType::Random =>           Box::new(RandomAgent {}),
            AgentType::HumanPlayer =>      Box::new(HumanPlayerAgent {}),
            AgentType::RandomRollout =>    Box::new(RandomRolloutAgent {}),
            AgentType::Dijkstra =>         Box::new(DijkstraAgent {}),
            AgentType::TabularQLearning => Box::new(TabularQLearningAgent {}),
            AgentType::None =>             panic!("Invalid agent type."),
        }
    }

    #[no_mangle]
    pub extern fn reset(&mut self) {
        // Author: Created by Yohann / Edited by all
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
    }

    #[no_mangle]
    pub extern fn send_type_p1(&mut self, agent_type: i8) {
        // Author: Created by Yohann
        self.agents.0 = Some(Self::create_agent_from_type(::agent::agent_type_from_i8(agent_type)));
    }

    #[no_mangle]
    pub extern fn send_type_p2(&mut self, agent_type: i8) {
        // Author: Created by Yohann
        self.agents.1 = Some(Self::create_agent_from_type(::agent::agent_type_from_i8(agent_type)));
    }

    #[no_mangle]
    pub extern fn epoch(&mut self, p1_h_action: HumanIntent, p2_h_action: HumanIntent) {
        // Author: Created by Yohann / Edited by Axel
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
        // Author: Created by Yohann
        let mut data = SharedData::new();
        self.to_shared_data(&mut data);
        data
    }

    pub fn step(&mut self, intents: (Intent, Intent)) {
        // Author: Created by Yohann / Edited by all
        let time_step = 1.0 / 60.0;
        self.time -= time_step;
        self.start_time += time_step;

        if self.players.0.score >= MAX_ROUND_POINTS ||
           self.players.1.score >= MAX_ROUND_POINTS ||
           self.time <= 0.0 {
           self.state_of_game = StateOfGame::End;
        }
        if self.state_of_game == StateOfGame::End {
            return;
        }
        if self.state_of_game == StateOfGame::Start && self.start_time >= 1.0 {
            self.state_of_game = StateOfGame::Playing;

            let mut rng = ::rand::thread_rng();
            let target = match self.frisbee.last_held {
                Some(ref last_held) => {
                    match last_held {
                        PlayerSide::Left => &self.players.1,
                        PlayerSide::Right => &self.players.0,
                    }
                },
                None => {
                    if rng.gen_range(0.0, 1.0) < 0.5 {
                        self.frisbee.last_held = Some(PlayerSide::Right);
                        &self.players.0
                    } else {
                        self.frisbee.last_held = Some(PlayerSide::Left);
                        &self.players.1
                    }
                },
            };
            self.frisbee.direction = (target.pos + Vector2::new(target.get_horizontal_aim_direction(), 0.0) - self.frisbee.pos).normalized();
            self.frisbee.speed = INITIAL_FRISBEE_SPEED;
        }

        fn apply_action(player: &mut Player, frisbee: &mut Frisbee, intent: &Intent, state_of_game: &StateOfGame) {
            // Author: Created by Axel
            match intent {
                Intent::None => {},
                Intent::Move(dir) => {
                    if *state_of_game == StateOfGame::Playing {
                        if player.slide.is_none() {
                            match frisbee.held_by_player {
                                Some(held_by) if held_by == player.side.unwrap() => {},
                                _ => {
                                    player.pos += *dir * 0.1;
                                }
                            };
                        }
                    }
                },
                Intent::Dash(dir) => {
                    if *state_of_game == StateOfGame::Playing {
                        let dir = dir.normalized();
                        player.dash(dir * PLAYER_DASH_POWER);
                    }
                },
                Intent::Throw(dir) => {
                    match frisbee.held_by_player {
                        Some(held_by) if held_by == player.side.unwrap() => {
                            frisbee.direction = player.get_throw_vector(dir);
                            frisbee.speed = INITIAL_FRISBEE_SPEED;
                            frisbee.last_held = frisbee.held_by_player;
                            frisbee.held_by_player = None;
                        },
                        _ => {}
                    };
                }
            };

            if *state_of_game == StateOfGame::Playing {
                match frisbee.held_by_player {
                    None if ::collision::player_collides_with_frisbee(player, frisbee) => {
                        frisbee.held_by_player = player.side;
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
        }
        apply_action(&mut self.players.0, &mut self.frisbee, &intents.0, &self.state_of_game);
        apply_action(&mut self.players.1, &mut self.frisbee, &intents.1, &self.state_of_game);

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
            self.players.0.slide = None;
        }
        let collided = ::collision::player_collision(&mut self.players.1);
        if collided {
            self.players.1.slide = None;
        }

        ::collision::frisbee_collision_wall(&mut self.frisbee);
        let goal = ::collision::frisbee_collision_goal(&mut self.frisbee, &mut self.players);
        if goal {
            self.state_of_game = StateOfGame::Start;
            self.start_time = 0.0;
            self.players.0.dash_to_pos(Vector2::new(-9.0, 0.0));
            self.players.1.dash_to_pos(Vector2::new(9.0, 0.0));
        }
    }

    pub fn to_shared_data(&self, shared: &mut SharedData) {
        // Author: Created by Yohann / Edited by all
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
}
