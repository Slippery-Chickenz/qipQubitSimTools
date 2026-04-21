use std::env;

extern crate blas_src;
extern crate serde_json;

use qip_qst::experiment::Experiment;

fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let args: Vec<String> = env::args().collect();

    println!("Config File: {}\nSave File: {}", &args[1], &args[2]);

    let mut test_experiment = Experiment::from_json(&args[1]);
    dbg!(&test_experiment);
    test_experiment.run_experiment(&args[2]).unwrap();

}
