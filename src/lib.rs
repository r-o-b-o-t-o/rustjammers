extern crate rand;
#[macro_use]
extern crate bitflags;
extern crate bincode;

mod agent;
mod player;
mod frisbee;
mod vector2;
mod collision;
mod shared_data;
pub mod game_engine;

#[test]
fn discretize_pos_x() {
    fn discretize(val: f64, min: i64, max: i64, scale: f64) -> u32 {
        let min = min as f64 * scale;
        let max = max as f64 * scale;
        let val = val * scale;
        let amplitude = (max - min + 1.0) as u32;
        ((val.round() + min.abs()) as u32) % amplitude
    }
    println!("{}", discretize(-1.74, -2, 2, 10.0));
    assert!(true);
}

#[test]
fn light_up_vector() {
    let v = vector2::Vector2::new(-2.0, 1.0).normalized();
    println!("{:?}", v);
    assert!(true);
}

#[test]
fn run_q_learning() {
    fn max(arr: &[f32; ::agent::QVALUES_ACTIONS]) -> f32 {
        let mut max = std::f32::MIN;
        for x in arr {
            if max < *x {
                max = *x;
            }
        }
        max
    }

    fn progress_bar(count: i32, total: i32) {
        let bar_len = 30;
        let filled_len = (bar_len as f64 * count as f64 / (total as f64)).round() as i32;

        let percents = (100.0 * count as f64 / total as f64).round();
        let mut bar = "=".repeat(filled_len as usize);
        bar.push_str(&" ".repeat((bar_len - filled_len) as usize));

        print!("\r[{}] {}% ({} / {}) ", bar, percents, count, total);

        if count >= total {
            println!("");
        }
    }

    let mut engine = game_engine::GameEngine::new();
    let discounting_rate = 0.95f32;
    let learning_rate = 0.8f32;

    let min_explo_rate: f32 = 0.05;
    let max_explo_rate: f32 = 1.0;
    let explo_decay_rate: f32 = 0.005;

    engine.send_type_p1(agent::AgentType::TabularQLearning as i8);
    //engine.send_type_p2(agent::AgentType::TabularQLearning as i8);
    engine.send_type_p2(agent::AgentType::Random as i8);

    println!("Initializing table...");
    engine.q_values = agent::get_blank_q_values();
    engine.explo_rate = 1.0;

    let n = 50_000;
    println!("Running {} simulations...", n);
    progress_bar(0, n);
    for i in 0..n {
        engine.reset();
        while engine.state_of_game != game_engine::StateOfGame::End {
            let state = engine.hash();

            engine.epoch(::agent::HumanIntent::IDLE, ::agent::HumanIntent::IDLE);
            let actions = engine.inputs;
            let actions = (
                ::agent::human_intent_to_index(actions.0) as usize,
                /*::agent::human_intent_to_index(actions.1) as usize,*/
            );

            // Update Q-Values
            let new_state = engine.hash();
            engine.q_values.get_mut(&state).unwrap().0[actions.0] = engine.q_values[&state].0[actions.0] + learning_rate * (engine.rewards.0 + discounting_rate * max(&engine.q_values[&new_state].0) - engine.q_values[&state].0[actions.0]);
            //engine.q_values.get_mut(&state).unwrap().1[actions.1] = engine.q_values[&state].1[actions.1] + learning_rate * (engine.rewards.1 + discounting_rate * max(&engine.q_values[&new_state].1) - engine.q_values[&state].1[actions.1]);

            if engine.q_scored {
                break;
            }
        }
        // Update exploration rate
        engine.explo_rate = min_explo_rate + (max_explo_rate - min_explo_rate) * (-explo_decay_rate * i as f32).exp();

        progress_bar(i + 1, n);
    }

    // Save Q-Values
    println!("Saving Q-values...");
    let encoded = bincode::serialize(&engine.q_values).expect("Could not encode Q-Values to binary");
    let mut path = ::std::env::current_dir().unwrap();
    path.push(::std::path::PathBuf::from("Unity/q_values.bin"));
    std::fs::write(path.clone(), encoded).expect("Unable to write Q-values.");

    println!("Done!\r\nSaved Q-values to \"{}\".", path.display());
}
