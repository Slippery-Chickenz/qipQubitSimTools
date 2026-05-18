extern crate blas_src;
extern crate serde_json;

use std::time::{Duration, Instant};

use qip_qst::experiment::Experiment;

fn main() {
    let mut test_experiment = Experiment::from_json("examples/benchmark.json");

    let now: Instant = Instant::now();
    test_experiment
        .run_experiment("examples/benchmark")
        .unwrap();

    let function_time: Duration = now.elapsed();
    println!("Rabi took: {:.4} s", function_time.as_secs_f64());
}
