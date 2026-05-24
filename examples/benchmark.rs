use std::env;

extern crate blas_src;
extern crate serde_json;

use std::time::{Duration, Instant};

use qip_qst::experiment::Experiment;

fn main() {

    let args: Vec<String> = env::args().collect();

    println!("Benchmarking {}", args[1]);

    let config_file: String = "examples/".to_owned() + &args[1] + "_config.json";

    let mut test_experiment = Experiment::from_json(&config_file);
    dbg!(&test_experiment);
    let now: Instant = Instant::now();
    test_experiment
        .run_experiment("examples/benchmark")
        .unwrap();

    let function_time: Duration = now.elapsed();
    println!("{} took: {:.4} s", args[1], function_time.as_secs_f64());
}
