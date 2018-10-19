extern crate rustjammers_engine;
extern crate bincode;

use rustjammers_engine::agent;
use rustjammers_engine::game_engine;

fn main() {
    fn max(arr: &[f32; agent::QVALUES_ACTIONS]) -> f32 {
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

    let mut n = 50_000;
    if let Some(arg) = std::env::args().nth(1) {
        match arg.parse() {
            Ok(arg) => n = arg,
            Err(_) => {
                eprintln!("Could not parse the number of simulations.");
                std::process::exit(1);
            }
        };
    }
    if n <= 0 {
        eprintln!("Incorrect number of simulations.");
        std::process::exit(1);
    }

    let mut engine = game_engine::GameEngine::new();
    let discounting_rate = 0.95f32;
    let learning_rate = 0.8f32;

    let min_explo_rate: f32 = 0.05;
    let max_explo_rate: f32 = 1.0;
    let explo_decay_rate: f32 = 0.005;

    engine.send_type_p1(agent::AgentType::TabularQLearning as i8, 0.0, 0);
    //engine.send_type_p2(agent::AgentType::TabularQLearning as i8);
    engine.send_type_p2(agent::AgentType::Random as i8, 0.0, 0);

    println!("Initializing table...");
    engine.q_values = agent::get_blank_q_values();
    engine.explo_rate = 1.0;

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
    path.push(::std::path::PathBuf::from("Unity"));
    if !path.exists() {
        path = ::std::env::current_dir().unwrap();
    }
    path.push(::std::path::PathBuf::from("q_values.bin"));
    std::fs::write(path.clone(), encoded).expect("Unable to write Q-values.");

    println!("Done!\r\nSaved Q-values to \"{}\".", path.display());
}
