use vector2::Vector2;
use frisbee::Frisbee;
use shared_data::SharedData;
use player::{ Player, PlayerSide };
use agent::{ Intent, AgentType, Agent, RandomAgent, HumanPlayerAgent, RandomRolloutAgent, DijkstraAgent, TabularQLearningAgent };

use std::mem::transmute;

pub struct GameEngine {
    pub players: (Player, Player),
    pub agents:  (Option<Box<Agent>>, Option<Box<Agent>>),
    pub frisbee: Frisbee,
    pub inputs: (i8, i8)
}

impl GameEngine {
    #[no_mangle]
    pub extern fn initialize() -> *mut Self {
        unsafe { transmute(Box::new(Self::new())) }
    }

    #[no_mangle]
    pub extern fn dispose(ptr: *mut Self) {
        let _state: Box<Self> = unsafe { transmute(ptr) };
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
            inputs: (
                0,
                0,
            )
        }
    }

    pub fn create_agent_from_type(agent_type: AgentType) -> Box<Agent> {
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
    }

    #[no_mangle]
    pub extern fn send_type_p1(&mut self, agent_type: i8) {
        self.agents.0 = Some(Self::create_agent_from_type(::agent::agent_type_from_i8(agent_type)));
    }

    #[no_mangle]
    pub extern fn send_type_p2(&mut self, agent_type: i8) {
        self.agents.1 = Some(Self::create_agent_from_type(::agent::agent_type_from_i8(agent_type)));
    }

    #[no_mangle]
    pub extern fn epoch(&mut self, p1_h_action: i8, p2_h_action: i8) {
        let mut a1 = self.agents.0.take().unwrap();
        let mut a2 = self.agents.1.take().unwrap();

        let input1 = match a1.get_type() { 
            AgentType::HumanPlayer => p1_h_action,
            _ => 0
        };

        let input2 = match a2.get_type() {
            AgentType::HumanPlayer => p2_h_action,
            _ => 0
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
        fn apply_action(player: &mut Player, frisbee: &mut Frisbee, intent: &Intent) {
            match intent {
                Intent::None => {},
                Intent::Move(dir) => {
                    player.pos += *dir * 0.1;
                },
                Intent::Dash(dir) => {
                    // TODO: accelerate instead of teleport
                    player.pos.x += dir.x * 0.5;
                    player.pos.y += dir.y * 0.5;
                },
                Intent::Throw(dir) => {
                    match frisbee.held_by_player {
                        Some(held_by) if held_by == player.side.unwrap() => {
                            use frisbee::ThrowDirection;

                            let horizontal = player.get_horizontal_aim_direction();
                            frisbee.direction = match dir {
                                ThrowDirection::Up => {
                                    let mut dir = Vector2::new(horizontal, 1.0);
                                    dir.normalize();
                                    dir
                                },
                                ThrowDirection::Middle => {
                                    Vector2::new(horizontal, 0.0)
                                },
                                ThrowDirection::Down => {
                                    let mut dir = Vector2::new(horizontal, -1.0);
                                    dir.normalize();
                                    dir
                                }
                            };
                            frisbee.speed = 2.0;
                            frisbee.last_held = frisbee.held_by_player;
                            frisbee.held_by_player = None;
                        },
                        _ => {}
                    };
                }
            };

            match frisbee.held_by_player {
                None if ::collision::player_collides_with_frisbee(player, frisbee) => {
                    frisbee.held_by_player = player.side;
                },
                _ => {}
            };
        }
        apply_action(&mut self.players.0, &mut self.frisbee, &intents.0);
        apply_action(&mut self.players.1, &mut self.frisbee, &intents.1);

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

        ::collision::player_collision(&mut self.players.0);
        ::collision::player_collision(&mut self.players.1);

        ::collision::frisbee_collision_wall(&mut self.frisbee);
        // TODO: handle frisbee-goal collisions
        ::collision::frisbee_collision_goal(&mut self.frisbee, &mut self.players);
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
    }
}
