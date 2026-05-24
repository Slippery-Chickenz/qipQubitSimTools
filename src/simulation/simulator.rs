use std::{marker::PhantomData, rc::Rc};

use crate::simulation::{
    Circuit, QubitArray, SimulationMethod, SimulationResultSaver, SimulationTimes,
};

/// Simulator for a given quantum circuit on an array of qubits
pub struct Simulator<Method: SimulationMethod> {
    /// Circuit to be simulated
    circuit: Circuit,
    /// Array of qubits for the circuit to be simulated on
    qubit_array: QubitArray,
    /// Times and samples for the simulation to be run and saved at
    simulation_times: Rc<SimulationTimes>,
    /// Phantom data to store the type of method we use to simulate the circuit
    simulation_method: PhantomData<Method>,
}

impl<Method: SimulationMethod> Simulator<Method> {
    /// Make an empty simulator object
    // Set the circuit, qubit array, and simulation times to be simulated. The number of qubits in
    // the circuit and qubit array must be the same (currently only 1 qubit is supported). The
    // simulation times are set as a number of samples to save for the simulation and the number of
    // iterations to perform between each sample. So for 4 samples and 20 iterations there would be
    // 80 total time steps.
    pub fn new(
        circuit: Circuit,
        qubit_array: QubitArray,
        step_size: f64,
        num_samples: usize,
    ) -> Simulator<Method> {
        let duration: f64 = circuit.get_duration();
        return Simulator::<Method> {
            circuit: circuit,
            qubit_array: qubit_array,
            simulation_times: Rc::new(SimulationTimes::new(duration, step_size, num_samples)),
            simulation_method: PhantomData,
        };
    }
    /// Simulate a given circuit, on a given qubit array, with the given numbers of samples, and iterations
    pub fn simulate_circuit(
        circuit: Circuit,
        qubit_array: QubitArray,
        step_size: f64,
        num_samples: usize,
    ) -> Method::ResultType {
        let mut simulator: Simulator<Method> =
            Simulator::new(circuit, qubit_array, step_size, num_samples);
        return simulator.run();
    }
    /// Simulate the circuit currently set
    pub fn run(&mut self) -> Method::ResultType {
        // Prepare variables for iterating over each qubit evolution
        let (mut simulation_results, iteration_indicies, save_offset, mut qubit_state): (
            Method::ResultType,
            Vec<usize>,
            usize,
            Method::QubitState,
        ) = self.prepare_simulation();

        // Loop over all the sample indices and evolve from one sample to the next
        for i in 0..iteration_indicies.len() - 1 {
            qubit_state = Method::evolve_state(
                &mut self.circuit,
                &self.qubit_array,
                self.simulation_times.as_ref(),
                qubit_state,
                iteration_indicies[i],
                iteration_indicies[i + 1] - 1,
            );
            simulation_results.save_state(i + save_offset, qubit_state.clone());
        }
        return simulation_results;
    }
    fn prepare_simulation(
        &mut self,
    ) -> (Method::ResultType, Vec<usize>, usize, Method::QubitState) {
        // Make an empty simulation results to return
        let mut simulation_results: Method::ResultType =
            Method::ResultType::new(Rc::clone(&self.simulation_times));

        // Make sure the qubit array has the correct number of qubits for this circuit
        assert!(
            self.qubit_array.get_num_qubits() == self.circuit.get_num_qubits(),
            "Qubit array contains {} qubits but circuit is made for {}",
            self.qubit_array.get_num_qubits(),
            self.circuit.get_num_qubits()
        );

        // Set the simulation times for the circuit and qubit array
        self.circuit
            .set_simulation_times(Rc::clone(&self.simulation_times));
        self.qubit_array
            .set_simulation_times(Rc::clone(&self.simulation_times));

        // Get the starting state for the simulation
        let qubit_state: Method::QubitState =
            Method::get_state(self.qubit_array.get_starting_state());

        // Get the indicies to iterate over
        let mut iteration_indicies: Vec<usize> = self.simulation_times.get_sample_indices().clone();

        // If there is is more than 1 sample to be taken then offset the saves to include the
        // starting state
        let save_offset: usize;
        if self.simulation_times.get_num_samples() == 1 {
            save_offset = 0;
            iteration_indicies.insert(0, 0);
        } else {
            simulation_results.save_state(0, qubit_state.clone());
            save_offset = 1;
        }

        return (
            simulation_results,
            iteration_indicies,
            save_offset,
            qubit_state,
        );
    }
}
