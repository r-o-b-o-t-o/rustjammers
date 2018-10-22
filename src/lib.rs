extern crate rand;
#[macro_use]
extern crate bitflags;
extern crate bincode;

pub mod agent;
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

#[test]
fn discretize_frisbee_direction() {
    use vector2::Vector2;

    fn do_it(v: Vector2) -> u32 {
        fn angle(v: Vector2) -> f64 {
            (v.y / v.x.abs()).sin().to_degrees()
        }

        let a = angle(v);
        if a > 40.0 {
            0
        } else if a > 10.0 {
            1
        } else if a < 10.0 && a > -10.0 {
            2
        } else if a < -10.0 && a > -40.0 {
            3
        } else {
            4
        }
    }

    let horizontal = 1.0;
    println!("{:?}", do_it(Vector2::new(horizontal, 1.0).normalized()));
    println!("{:?}", do_it(Vector2::new(2.0 * horizontal, 1.0).normalized()));
    println!("{:?}", do_it(Vector2::new(horizontal, 0.0).normalized()));
    println!("{:?}", do_it(Vector2::new(2.0 * horizontal, -1.0).normalized()));
    println!("{:?}", do_it(Vector2::new(horizontal, -1.0).normalized()));
}
