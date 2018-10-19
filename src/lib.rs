extern crate rand;
#[macro_use]
extern crate bitflags;

mod agent;
mod player;
mod frisbee;
mod vector2;
mod collision;
mod shared_data;
pub mod game_engine;


#[test]
fn test_dijkstra() {
    let mut test = game_engine::GameEngine::new();
    let mut step: i64 = 0;
    test.reset();
    test.send_type_p1(agent::AgentType::HumanPlayer as i8, 1000.0, 3);
    test.send_type_p2(agent::AgentType::Dijkstra as i8, 1000.0, 3);
    loop {

        println!("STEP: {}", step.to_string());
        test.epoch(agent::HumanIntent::IDLE, agent::HumanIntent::IDLE);

        if test.state_of_game == game_engine::StateOfGame::End {
            break;
        }
        step+=1;


    } 
}
