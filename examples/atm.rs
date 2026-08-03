extern crate blas_src;
extern crate serde_json;

use qip_qst::experiment::Experiment;

fn main() {
    let mut test_experiment: Experiment =
        Experiment::from_json_file("examples/atm_config.json").unwrap();
    dbg!(&test_experiment);
    test_experiment
        .run_experiment("examples/atm_results")
        .unwrap();
}
