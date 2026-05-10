extern crate blas_src;
extern crate serde_json;

use qip_qst::experiment::Experiment;

fn main() {
    let mut test_experiment = Experiment::from_json("examples/atm_config.json");
    dbg!(&test_experiment);
    test_experiment
        .run_experiment("examples/atm_results")
        .unwrap();
}
