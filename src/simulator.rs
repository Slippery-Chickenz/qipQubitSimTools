use std::rc::Rc;

use crate::circuit::Circuit;
use crate::qubit_array::QubitArray;
use crate::simulation_results::SimulationResults;
use crate::simulation_times::SimulationTimes;

use num_complex::Complex64;
use ndarray::{ Array3, Array4, Axis };
use ndarray_linalg::expm::expm;

pub struct Simulator { }

impl Simulator {
    pub fn new() -> Simulator {
        return Simulator {}
    }
    pub fn simulate_circuit(&mut self, circuit: &mut Circuit, qubit_array: &mut QubitArray, guess_larmor: f64, num_iterations: usize, num_samples: usize) -> SimulationResults  {

        // Make sure the qubit array has the correct number of qubits for this circuit
        assert!(qubit_array.get_num_qubits() == circuit.get_num_qubits(), 
                "Qubit array contains {} qubits but circuit is made for {}", 
                qubit_array.get_num_qubits(), circuit.get_num_qubits());

        // Times for the simulation
        let simulation_times: Rc<SimulationTimes> = Rc::new(SimulationTimes::new(circuit.get_duration(), num_iterations, num_samples));
        circuit.set_simulation_times(Rc::clone(&simulation_times));
        circuit.integrate_frequencies();
        qubit_array.set_simulation_times(Rc::clone(&simulation_times));

        let mut simulation_results: SimulationResults = SimulationResults::new();

        for i in 0..simulation_times.get_iteration_times().len() {

            let evolution_operators: Array4<Complex64> = self.get_evolution_operator(circuit, 
                                                                                     qubit_array, 
                                                                                     &simulation_times, 
                                                                                     guess_larmor, 
                                                                                     i, 
                                                                                     simulation_times.get_dt());

            qubit_array.evolve_state(evolution_operators);
        }
        simulation_results.add_array(qubit_array);
        return simulation_results;
    }
    fn get_evolution_operator(&self, 
                              circuit: &Circuit, 
                              qubit_array: &QubitArray, 
                              simulation_times: &SimulationTimes,
                              guess_larmor: f64, 
                              sample_num: usize, 
                              dt: f64) -> Array4<Complex64> {

        let mut evolution_operators: Array4<Complex64> = Array4::<Complex64>::zeros([simulation_times.get_num_iterations_per_sample(), 2, 2, 2]);
        let qubit_hamiltonians: Array3<Complex64> = circuit.get_hamiltonian_operator(sample_num) + qubit_array.get_detuning_hamiltonians(guess_larmor);

        for mut iter in qubit_hamiltonians.outer_iter().zip(evolution_operators.outer_iter_mut()) {
            iter.1.index_axis_mut(Axis(0), 0).assign(&expm(&(Complex64::new(0., -1.) * dt * iter.0.to_owned())).0);
            iter.1.index_axis_mut(Axis(0), 1).assign(&expm(&(Complex64::new(0., 1.) * dt * iter.0.to_owned())).0);
        }

        return evolution_operators;
    }
}
















