use std::rc::Rc;

use crate::simulation::{
    Circuit, QubitArray, SimulationResults, SimulationTimes, simulation_results,
};

use ndarray::Array2;
use num_complex::Complex64;

/// Simulator for a given quantum circuit on an array of qubits
pub struct Simulator {
    /// Circuit to be simulated
    circuit: Option<Circuit>,
    /// Array of qubits for the circuit to be simulated on
    qubit_array: Option<QubitArray>,
    /// Times and samples for the simulation to be run and saved at
    simulation_times: Option<Rc<SimulationTimes>>,
}

impl Simulator {
    /// Make an empty simulator object
    pub fn new() -> Simulator {
        return Simulator {
            circuit: None,
            qubit_array: None,
            simulation_times: None,
        };
    }
    // Set the circuit, qubit array, and simulation times to be simulated. The number of qubits in
    // the circuit and qubit array must be the same (currently only 1 qubit is supported). The
    // simulation times are set as a number of samples to save for the simulation and the number of
    // iterations to perform between each sample. So for 4 samples and 20 iterations there would be
    // 80 total time steps.
    pub fn set_simulation(
        &mut self,
        circuit: Circuit,
        qubit_array: QubitArray,
        step_size: f64,
        num_samples: usize,
    ) -> () {
        self.circuit = Some(circuit);
        self.qubit_array = Some(qubit_array);
        self.simulation_times = Some(Rc::new(SimulationTimes::new(
            self.circuit.as_ref().unwrap().get_duration(),
            step_size,
            num_samples,
        )));
        return;
    }
    /// Simulate a given circuit, on a given qubit array, with the given numbers of samples, and iterations
    pub fn simulate_circuit(
        &mut self,
        circuit: Circuit,
        qubit_array: QubitArray,
        step_size: f64,
        num_samples: usize,
    ) -> SimulationResults {
        self.circuit = Some(circuit);
        self.qubit_array = Some(qubit_array);
        self.simulation_times = Some(Rc::new(SimulationTimes::new(
            self.circuit.as_ref().unwrap().get_duration(),
            step_size,
            num_samples,
        )));
        return self.run();
    }
    /// Simulate the circuit currently set
    pub fn run(&mut self) -> SimulationResults {
        let (mut simulation_results, mut sample_indicies): (SimulationResults, Vec<usize>) =
            self.prepare_simulation();

        let mut save_offset: usize = 1;
        if sample_indicies.len() == 1 {
            save_offset = 0;
        }

        let mut qubit_state: Array2<Complex64> = simulation_results.get_density_matrix(0);

        // If the first index is 0 then save the starting state
        if sample_indicies[0] == 0 {
            simulation_results.save_state(0, qubit_state.clone());
        } else {
            sample_indicies.insert(0, 0);
        }

        // Loop over all the sample indices and evolve from one sample to the next
        for i in 0..sample_indicies.len() - 1 {
            qubit_state =
                self.evolve_qubit(qubit_state, sample_indicies[i], sample_indicies[i + 1] - 1);
            simulation_results.save_state(i + save_offset, qubit_state.clone());
        }
        return simulation_results;
    }
    fn prepare_simulation(&mut self) -> (SimulationResults, Vec<usize>) {
        // If any of the circuit, qubit array, or simulation times are not set then panic
        if let (Some(circuit), Some(qubit_array), Some(simulation_times)) = (
            self.circuit.as_mut(),
            self.qubit_array.as_mut(),
            self.simulation_times.as_mut(),
        ) {
            // Make an empty simulation results to return
            let simulation_results = SimulationResults::new(
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

            // Set the simulation times for the circuit and qubit array
            circuit.set_simulation_times(Rc::clone(&simulation_times));
            qubit_array.set_simulation_times(Rc::clone(&simulation_times));

            return (
                simulation_results,
                self.simulation_times
                    .as_mut()
                    .unwrap()
                    .get_sample_indices()
                    .clone(),
            );
        }
        panic!("Nothin to simulate");
    }
    /// Evolve a given state between two time indicies
    fn evolve_qubit(
        &mut self,
        mut density_matrix: Array2<Complex64>,
        t_start: usize,
        t_end: usize,
    ) -> Array2<Complex64> {
        // If any of the circuit, qubit array, or simulation times are not set then panic
        if let (Some(circuit), Some(qubit_array), Some(simulation_times)) = (
            self.circuit.as_mut(),
            self.qubit_array.as_mut(),
            self.simulation_times.as_mut(),
        ) {
            let dt: f64 = simulation_times.get_dt();
            for t_index in t_start..t_end {
                let time = simulation_times.get_iteration_time(t_index);

                // Four coefficients for Runge Kutta
                let k_1: Array2<Complex64> =
                    Simulator::get_lindblad_operator(circuit, qubit_array, time, &density_matrix);
                let k_2: Array2<Complex64> = Simulator::get_lindblad_operator(
                    circuit,
                    qubit_array,
                    time + (dt / 2.),
                    &(&density_matrix + (&k_1 * (dt / 2.))),
                );
                let k_3: Array2<Complex64> = Simulator::get_lindblad_operator(
                    circuit,
                    qubit_array,
                    time + (dt / 2.),
                    &(&density_matrix + (&k_2 * (dt / 2.))),
                );
                let k_4: Array2<Complex64> = Simulator::get_lindblad_operator(
                    circuit,
                    qubit_array,
                    time + dt,
                    &(&density_matrix + (&k_3 * dt)),
                );

                let next_state = density_matrix + (k_1 + (k_2 * 2.) + (k_3 * 2.) + k_4) * (dt / 6.);
                density_matrix = next_state;
            }
        }
        return density_matrix;
    }

    /// Get the lindblad operator for the defined circuit at a specific time
    fn get_lindblad_operator(
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        time: f64,
        density_matrix: &Array2<Complex64>,
    ) -> Array2<Complex64> {
        let hamiltonian: Array2<Complex64> =
            circuit.get_hamiltonian_operator(time) + qubit_array.get_detuning_hamiltonian();

        let system_term: Array2<Complex64> = Complex64::new(0., -1.)
            * (hamiltonian.dot(density_matrix) - density_matrix.dot(&hamiltonian));

        let mut jump_operator: Array2<Complex64> = Array2::<Complex64>::eye(2);
        jump_operator[[1, 1]] = Complex64::new(-1., 0.);

        let jump_operator_conj: Array2<Complex64> =
            jump_operator.clone().mapv(|x| x.conj()).reversed_axes();

        let dephasing_intensity: Complex64 = Complex64::new(qubit_array.get_decoherence(), 0.);

        let environment_term: Array2<Complex64> = 0.5
            * dephasing_intensity
            * (jump_operator.dot(density_matrix).dot(&jump_operator_conj) * 2.
                - jump_operator_conj.dot(&jump_operator).dot(density_matrix)
                - density_matrix.dot(&jump_operator_conj).dot(&jump_operator));

        return system_term + environment_term;
    }
}
