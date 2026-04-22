use std::rc::Rc;

use crate::simulation::{Circuit, QubitArray, SimulationResults, SimulationTimes};

use ndarray::{Array2, Array3, Array4, Axis};
use ndarray_linalg::{OperationNorm, expm::expm};
use num_complex::{Complex64, ComplexFloat};

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
    /// Simulate a given circuit, on a given qubit array, with the given numbers of samples, and iterations
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
    /// Simulate the current set circuit and return the simulation results.
    pub fn simulate_current_circuit(&mut self) -> SimulationResults {
        // If any of the circuit, qubit array, or simulation times are not set then panic
        if let (Some(circuit), Some(qubit_array), Some(simulation_times)) = (
            self.circuit.as_mut(),
            self.qubit_array.as_mut(),
            self.simulation_times.as_mut(),
        ) {
            // Make an empty simulation results to return
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

            // Set the simulation times for the circuit and qubit array
            circuit.set_simulation_times(Rc::clone(&simulation_times));
            qubit_array.set_simulation_times(Rc::clone(&simulation_times));

            let channel_coefficients: Array2<Complex64> = qubit_array
                .get_channel_coefficients()
                .mapv(|x| Complex64::new(x, 0.));

            // Loop over every sample and evolve to the next sample
            for i in 0..simulation_times.get_num_samples() - 1 {
                //let evolution_operators: Array4<Complex64> = self.get_evolution_operator(i);
                let evolution_operators: Array4<Complex64> =
                    Simulator::get_evolution_operator(circuit, qubit_array, simulation_times, i);
                simulation_results.evolve_state(i, evolution_operators, &channel_coefficients);
            }

            // If there is only a single sample time then the results should only save the ending
            // state not the starting state
            if simulation_times.get_sample_times().len() == 1 {
                simulation_results.remove_starting_sample();
            }
            return simulation_results;
        }
        panic!();
    }
    /// Get the evolution operators to go from given sample n to n+1. Returns a 4D Array. The first
    /// axis is the iteration number, second axis is for the either side of the evolution of the
    /// density matrix (index 0 is $e^iHdt$, index 1 is $e^-iHdt$). Final 2 axes are the 2x2
    /// evolution matrices
    // fn get_evolution_operator(&self, sample_num: usize) -> Array4<Complex64> {
    fn get_evolution_operator(
        circuit: &Circuit,
        qubit_array: &QubitArray,
        simulation_times: &SimulationTimes,
        sample_num: usize,
    ) -> Array4<Complex64> {
        // Array to set and return the evolution operators
        let mut evolution_operators: Array4<Complex64> =
            Array4::<Complex64>::zeros([simulation_times.get_num_iterations_per_sample(), 2, 2, 2]);

        // Array of the Hamiltonians at each iteration to exponate into the eovlution operators
        let qubit_hamiltonians: Array3<Complex64> = circuit.get_hamiltonian_operator(sample_num)
            + qubit_array.get_detuning_hamiltonians(sample_num);

        // Loop over all the hamiltonians and the outer axis of the evolution operators to assign
        for mut iter in qubit_hamiltonians
            .outer_iter()
            .zip(evolution_operators.outer_iter_mut())
        {
            // Checking to make sure the one norm of the hamiltonian is above the f64 epsilon.
            // This is mostly because the expm function will panic if not. Should be fixed by
            // just propagating the error out from the expm function
            let a_one_norm = iter.0.map(|x| x.abs()).opnorm_one().unwrap();
            if simulation_times.get_dt() == 0. || a_one_norm < f64::EPSILON * 2. {
                iter.1.index_axis_mut(Axis(0), 0).assign(&Array2::eye(2));
                iter.1.index_axis_mut(Axis(0), 1).assign(&Array2::eye(2));
            } else {
                // Assign both the left and right evolution operators
                iter.1.index_axis_mut(Axis(0), 0).assign(
                    &expm(
                        &(Complex64::new(0., -1.) * simulation_times.get_dt() * iter.0.to_owned()),
                    )
                    .0,
                );
                iter.1.index_axis_mut(Axis(0), 1).assign(
                    &expm(
                        &(Complex64::new(0., 1.) * simulation_times.get_dt() * iter.0.to_owned()),
                    )
                    .0,
                );
            }
        }
        return evolution_operators;
    }
}
