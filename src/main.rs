use std::env;
use std::rc::Rc;

extern crate blas_src;
extern crate serde_json;

use qip_qst::{experiment::Experiment, simulation::{SimulationResults, SimulationTimes}};

use ndarray::Array1;
use num_complex::Complex64;

fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }


    let mut test_result: SimulationResults = SimulationResults::new(Rc::new(SimulationTimes::new(10., 0.01, 10)));
    let mut test_state: Array1<Complex64> = Array1::<Complex64>::zeros(4);
    test_state[0] = Complex64::new(1., 0.);
    test_result.save_state(0, test_state);
    dbg!(&test_result.get_probability(0, &Array1::<Complex64>::from_vec(vec![Complex64::new(1., 0.), Complex64::new(0., 0.)])));
}

