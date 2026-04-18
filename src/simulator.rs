use std::rc::Rc;

use crate::circuit::Circuit;
use crate::qubit_array::QubitArray;
use crate::simulation_results::SimulationResults;
use crate::simulation_times::SimulationTimes;

use ndarray::{Array2, Array3, Array4, Axis};
use ndarray_linalg::OperationNorm;
use ndarray_linalg::expm::expm;
use num_complex::Complex64;
use num_complex::ComplexFloat;

pub struct Simulator {
    circuit: Option<Circuit>,
    qubit_array: Option<QubitArray>,
    simulation_times: Option<Rc<SimulationTimes>>,
}

impl Simulator {
    pub fn new() -> Simulator {
        return Simulator {
            circuit: None,
            qubit_array: None,
            simulation_times: None,
        };
    }
    pub fn set_simulation(
        &mut self,
        circuit: Circuit,
        qubit_array: QubitArray,
        num_iterations: usize,
        num_samples: usize,
    ) -> () {
        self.circuit = Some(circuit);
        self.qubit_array = Some(qubit_array);
        self.simulation_times = Some(Rc::new(SimulationTimes::new(
            self.circuit.as_ref().unwrap().get_duration(),
            num_iterations,
            num_samples,
        )));
        return;
    }
    pub fn simulate_circuit(
        &mut self,
        circuit: Circuit,
        qubit_array: QubitArray,
        num_iterations: usize,
        num_samples: usize,
    ) -> SimulationResults {
        self.circuit = Some(circuit);
        self.qubit_array = Some(qubit_array);
        self.simulation_times = Some(Rc::new(SimulationTimes::new(
            self.circuit.as_ref().unwrap().get_duration(),
            num_iterations,
            num_samples,
        )));
        return self.simulate_current_circuit();
    }

    pub fn simulate_current_circuit(&mut self) -> SimulationResults {
        if let (Some(circuit), Some(qubit_array), Some(simulation_times)) = (
            self.circuit.as_mut(),
            self.qubit_array.as_mut(),
            self.simulation_times.as_mut(),
        ) {
            let mut simulation_results: SimulationResults = SimulationResults::new(
                Rc::clone(&simulation_times),
                qubit_array.get_density_matrix(),
            );

            // Make sure the qubit array has the correct number of qubits for this circuit
            assert!(
                qubit_array.get_num_qubits() == circuit.get_num_qubits(),
                "Qubit array contains {} qubits but circuit is made for {}",
                qubit_array.get_num_qubits(),
                circuit.get_num_qubits()
            );

            circuit.set_simulation_times(Rc::clone(&simulation_times));
            qubit_array.set_simulation_times(Rc::clone(&simulation_times));

            for i in 0..simulation_times.get_iteration_times().len() {
                let evolution_operators: Array4<Complex64> = self.get_evolution_operator(i);
                simulation_results.evolve_state(i, evolution_operators);
            }
            return simulation_results;
        }
        panic!();
    }
    // pub fn simulate_detuning_response(
    //     &mut self,
    //     circuit: &mut Circuit,
    //     qubit_array: &mut QubitArray,
    //     guess_larmors: Vec<f64>,
    //     num_iterations: usize,
    // ) -> LarmorSweepResult {
    //     let mut larmor_sweep_result: LarmorSweepResult = LarmorSweepResult::new();
    //     //
    //     // for l in &guess_larmors {
    //     //     let sim_result =
    //     //         self.simulate_circuit(circuit, &mut qubit_array.clone(), *l, num_iterations, 2);
    //     //     larmor_sweep_result.add_result(sim_result);
    //     // }
    //     //
    //     // larmor_sweep_result.set_larmor_values(guess_larmors);
    //     return larmor_sweep_result;
    // }
    fn get_evolution_operator(&self, sample_num: usize) -> Array4<Complex64> {
        if let (Some(circuit), Some(qubit_array), Some(simulation_times)) =
            (&self.circuit, &self.qubit_array, &self.simulation_times)
        {
            let mut evolution_operators: Array4<Complex64> = Array4::<Complex64>::zeros([
                simulation_times.get_num_iterations_per_sample(),
                2,
                2,
                2,
            ]);
            let qubit_hamiltonians: Array3<Complex64> = circuit
                .get_hamiltonian_operator(sample_num)
                + qubit_array.get_detuning_hamiltonians();

            for mut iter in qubit_hamiltonians
                .outer_iter()
                .zip(evolution_operators.outer_iter_mut())
            {
                let a_one_norm = iter.0.map(|x| x.abs()).opnorm_one().unwrap();

                if a_one_norm < f64::EPSILON * 2. {
                    iter.1.index_axis_mut(Axis(0), 0).assign(&Array2::eye(2));
                    iter.1.index_axis_mut(Axis(0), 1).assign(&Array2::eye(2));
                } else {

                    iter.1.index_axis_mut(Axis(0), 0).assign(
                        &expm(
                            &(Complex64::new(0., -1.)
                                * simulation_times.get_dt()
                                * iter.0.to_owned()),
                        )
                        .0,
                    );
                    iter.1.index_axis_mut(Axis(0), 1).assign(
                        &expm(
                            &(Complex64::new(0., 1.)
                                * simulation_times.get_dt()
                                * iter.0.to_owned()),
                        )
                        .0,
                    );
                }
            }
            return evolution_operators;
        }
        panic!();
    }
}
