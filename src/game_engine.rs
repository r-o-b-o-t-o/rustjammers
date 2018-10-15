use frisbee::Frisbee;
use shared_data::SharedData;
use player::{ Player, PlayerSide };
use agent::{ Intent, AgentType, Agent, RandomAgent };

use std::mem::transmute;

pub struct GameEngine {
    pub p1:      Player,
    pub p2:      Player,
    pub a1:      Option<Box<Agent>>,
    pub a2:      Option<Box<Agent>>,
    pub frisbee: Frisbee,
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
            p1:      Player::new(),
            p2:      Player::new(),
            a1:      Some(Self::match_agent(AgentType::Random)),
            a2:      Some(Self::match_agent(AgentType::Random)),
            frisbee: Frisbee::new()
        }
    }

    pub fn match_agent(agent_type: AgentType) -> Box<Agent> {
        match agent_type {
            AgentType::Random => Box::new(RandomAgent{}),
            AgentType::HumanPlayer => Box::new(RandomAgent{}),
            AgentType::RandomRollout => Box::new(RandomAgent{}),
            AgentType::Dijkstra => Box::new(RandomAgent{}),
            AgentType::TabularQLearning => Box::new(RandomAgent{}),
            AgentType::None => Box::new(RandomAgent{}),
        }
    }

    #[no_mangle]
    pub extern fn reset(&mut self) {
        self.p1.pos.x = -9.0;
        self.p1.pos.y = 0.0;
        self.p1.score = 0;
        self.p1.side = Some(PlayerSide::Left);

        self.p2.pos.x = 9.0;
        self.p2.pos.y = 0.0;
        self.p2.score = 0;
        self.p2.side = Some(PlayerSide::Right);

        self.frisbee.pos.x = 0.0;
        self.frisbee.pos.y = 0.0;
        self.frisbee.direction.x = 0.0;
        self.frisbee.direction.y = 0.0;
        self.frisbee.speed = 0.0;
    }

    #[no_mangle]
    pub extern fn send_type_p1(&mut self, agent_type: i8) {
        self.a1 = Some(Self::match_agent(::agent::agent_type_from_i8(agent_type)));
    }

    #[no_mangle]
    pub extern fn send_type_t2(&mut self, agent_type: i8) {
        self.a2 = Some(Self::match_agent(::agent::agent_type_from_i8(agent_type)));
    }

    #[no_mangle]
    pub extern fn epoch(&mut self) {
        let mut a1 = self.a1.take().unwrap();
        let mut a2 = self.a2.take().unwrap();

        let action_p1 = a1.act(PlayerSide::Left, self);
        let action_p2 = a2.act(PlayerSide::Left, self);

        self.a1 = Some(a1);
        self.a2 = Some(a2);

        self.step(action_p1, action_p2);
    }

    #[no_mangle]
    pub extern fn get_state(&mut self) -> SharedData {
        let mut data = SharedData::new();
        self.to_shared_data(&mut data);
        data
    }

    pub fn step(&mut self, p1_action: Intent, p2_action: Intent) {
        // Faire les action et a la fin renvoyer les positions des players et du frisbee
    }

    pub fn to_shared_data(&self, shared: &mut SharedData) {
        shared.p1_x = self.p1.pos.x;
        shared.p1_y = self.p1.pos.y;
        shared.p1_score = self.p1.score;
        shared.p1_side = 0;

        shared.p2_x = self.p2.pos.x;
        shared.p2_y = self.p2.pos.y;
        shared.p2_score = self.p2.score;
        shared.p2_side = 1;

        shared.zbee_x = self.frisbee.pos.x;
        shared.zbee_y = self.frisbee.pos.y;
        shared.zbee_held = ::player::player_side_to_i8(self.frisbee.held_by_player);
    }
}
