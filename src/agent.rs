pub enum AgentType {
    Random,
    HumanPlayer,
    RandomRollout,
    Dijkstra,
    TabularQLearning
}

pub trait Agent {
    fn act(&mut self);
}
