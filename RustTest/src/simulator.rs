use std::rc::Rc;

use crate::circuit::Circuit;
use crate::qubit_array::QubitArray;
use crate::simulation_results::SimulationResults;
use crate::simulation_times::SimulationTimes;

use num_complex::Complex64;
use ndarray::{ Array2, Array3 };

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

        let simulation_results: SimulationResults = SimulationResults::new(simulation_times.get_num_sample_times());

        for i in 0..(simulation_times.get_sample_indicies().len() - 1) {

            let _evolution_operator: Array2<Complex64> = self.get_evolution_operator(circuit, qubit_array, guess_larmor, i);

        }

        return simulation_results;
    }
    fn get_evolution_operator(&self, circuit: &Circuit, qubit_array: &QubitArray, guess_larmor: f64, sample_num: usize) -> Array2<Complex64> {

        let evolution_operator: Array2<Complex64> = Array2::<Complex64>::zeros([2, 2]);
        let _qubit_hamiltonians: Array3<Complex64> = circuit.get_hamiltonian_operator(sample_num) + qubit_array.get_detuning_hamiltonians(guess_larmor);

        // let mut temp_evolution: Array2<Complex64> = Array2::zeros((2, 2));
        //
        // for hamiltonian in qubit_hamiltonians.outer_iter() {
        //
        //     expm(&hamiltonian, &mut temp_evolution);
        //
        // }

        return evolution_operator;
    }
}
