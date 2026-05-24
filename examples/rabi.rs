use std::env;

extern crate blas_src;
extern crate serde_json;

use qip_qst::experiment::Experiment;

fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let mut test_experiment = Experiment::from_json("examples/rabi_config.json");
    dbg!(&test_experiment);
    test_experiment
        .run_experiment("examples/rabi_results")
        .unwrap();
}
