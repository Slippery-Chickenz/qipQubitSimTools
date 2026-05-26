use std::env;
use std::{fs, io::BufReader};

extern crate blas_src;
extern crate serde_json;
use serde_json::{Map, Value, json};

use std::time::Instant;

use qip_qst::experiment::Experiment;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Benchmarking {}", args[1]);

    let config_filename: String = "examples/".to_owned() + &args[1] + "_config.json";

    // File and reader to read the experiment config from
    let file: fs::File = fs::File::open(config_filename).unwrap();
    let reader: BufReader<fs::File> = BufReader::new(file);

    // Json values read in from the file
    let mut json_values: Map<String, Value> = serde_json::from_reader(reader).unwrap();

    // Map of simulation method to the time it took to simulate
    let mut benchmarking_stats: Vec<(&str, f64)> = vec![
        ("RK", 0.),
        ("RKVectorized", 0.),
        ("PadeVectorized", 0.),
        ("PadeState", 0.),
    ];

    // Loop over the benchmarking methods and test each one
    for (benchmark_string, time) in &mut benchmarking_stats {
        json_values.insert("method".to_string(), json!(benchmark_string));
        let mut test_experiment = Experiment::from_json(json_values.clone());
        let now: Instant = Instant::now();
        test_experiment
            .run_experiment("examples/benchmark")
            .unwrap();

        *time = now.elapsed().as_secs_f64();
    }

    for (benchmark_string, time) in benchmarking_stats {
        println!("{} method took: {:.4} s", benchmark_string, time);
    }
}
