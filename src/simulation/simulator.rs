use std::{marker::PhantomData, rc::Rc};

use crate::simulation::{
    Circuit, Hamiltonian, LabFrame, PulseFrame, QubitArray, ReferenceFrame, RotatingFrame,
    SimulationMethod, SimulationResultSaver, SimulationSettings, SimulationTimes,
};

/// Simulator for a given quantum circuit on an array of qubits
pub struct Simulator<Method: SimulationMethod> {
    /// Times and samples for the simulation to be run and saved at
    simulation_times: Rc<SimulationTimes>,
    /// Settings to run the simulation at
    simulation_settings: SimulationSettings,
    /// Circuit to be simulated
    circuit: Circuit,
    /// Array of qubits for the circuit to be simulated on
    qubit_array: QubitArray,
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
        simulation_settings: SimulationSettings,
        // reference_frame: ReferenceFrame,
        // step_size: f64,
        // num_samples: usize,
    ) -> Simulator<Method> {
        return Simulator::<Method> {
            simulation_times: Rc::new(SimulationTimes::new(
                circuit.get_duration(),
                simulation_settings.get_dt(),
                Method::get_num_times_per_step(),
                simulation_settings.get_num_samples(),
            )),
            simulation_settings: simulation_settings,
            circuit: circuit,
            qubit_array: qubit_array,
            simulation_method: PhantomData,
        };
    }
    /// Simulate a given circuit, on a given qubit array, with the given numbers of samples, and iterations
    pub fn simulate_circuit(
        circuit: Circuit,
        qubit_array: QubitArray,
        simulation_settings: SimulationSettings,
    ) -> Method::ResultType {
        let mut simulator: Simulator<Method> =
            Simulator::new(circuit, qubit_array, simulation_settings);
        return simulator.run();
    }
    /// Simulate the circuit currently set
    pub fn run(&mut self) -> Method::ResultType {
        match self.simulation_settings.get_reference_frame() {
            ReferenceFrame::Lab => {
                return self.run_in_frame::<LabFrame>(PhantomData);
            }
            ReferenceFrame::Rotating => {
                return self.run_in_frame::<RotatingFrame>(PhantomData);
            }
            ReferenceFrame::Pulse => {
                return self.run_in_frame::<PulseFrame>(PhantomData);
            }
        }
    }
    fn run_in_frame<T: Hamiltonian>(&mut self, hamiltonian: PhantomData<T>) -> Method::ResultType {
        // Prepare variables for iterating over each qubit evolution
        let (mut simulation_results, iteration_indicies, save_offset, mut qubit_state): (
            Method::ResultType,
            Vec<usize>,
            usize,
            Method::QubitStateType,
        ) = self.prepare_simulation();

        let save_hamiltonian: bool = self.simulation_settings.get_save_hamiltonian();

        if save_hamiltonian && self.simulation_times.get_num_samples() != 1 {
            simulation_results.save_starting_hamiltonian(T::get_matrix(
                &mut self.circuit,
                &self.qubit_array,
                0.,
                0,
                0,
            ));
        }

        // Loop over each shot and simulation the evolution
        for i in 0..self.simulation_settings.get_num_shots() {
            // Loop over all the sample indices and evolve from one sample to the next
            for j in 0..iteration_indicies.len() - 1 {
                qubit_state = Method::evolve_state(
                    &mut self.circuit,
                    &self.qubit_array,
                    self.simulation_times.as_ref(),
                    qubit_state,
                    hamiltonian,
                    i,
                    iteration_indicies[j],
                    iteration_indicies[j + 1],
                );
                simulation_results.save_state(i, j + save_offset, qubit_state.clone());
                if save_hamiltonian {
                    simulation_results.save_hamiltonian(
                        i,
                        j + save_offset,
                        T::get_matrix(
                            &mut self.circuit,
                            &self.qubit_array,
                            self.simulation_times
                                .get_iteration_time(iteration_indicies[j + 1] - 1),
                            i,
                            iteration_indicies[j + 1] - 1,
                        ),
                    );
                }
            }
            qubit_state = self.reset_for_shot();
        }
        return simulation_results;
    }
    fn reset_for_shot(&mut self) -> Method::QubitStateType {
        self.circuit.reset_for_shot();
        return Method::get_state(self.qubit_array.get_starting_state());
    }
    fn prepare_simulation(
        &mut self,
    ) -> (
        Method::ResultType,
        Vec<usize>,
        usize,
        Method::QubitStateType,
    ) {
        // Make an empty simulation results to return
        let mut simulation_results: Method::ResultType =
            Method::ResultType::new(Rc::clone(&self.simulation_times), &self.simulation_settings);

        // Make sure the qubit array has the correct number of qubits for this circuit
        assert!(
            self.qubit_array.get_num_qubits() == self.circuit.get_num_qubits(),
            "Qubit array contains {} qubits but circuit is made for {}",
            self.qubit_array.get_num_qubits(),
            self.circuit.get_num_qubits()
        );

        // Set the simulation times for the circuit and qubit array
        self.qubit_array.initialize_larmor_noise(&self.simulation_settings, &self.simulation_times);
            // .set_simulation_times(Rc::clone(&self.simulation_times));

        // Get the starting state for the simulation
        let qubit_state: Method::QubitStateType =
            Method::get_state(self.qubit_array.get_starting_state());

        // Get the indicies to iterate over
        let mut iteration_indicies: Vec<usize> = self.simulation_times.get_sample_indices().clone();
        *iteration_indicies.last_mut().unwrap() -= 1;

        // If there is is more than 1 sample to be taken then offset the saves to include the
        // starting state
        let save_offset: usize;
        if self.simulation_times.get_num_samples() == 1 {
            save_offset = 0;
            iteration_indicies.insert(0, 0);
        } else {
            simulation_results.save_starting_state(qubit_state.clone());
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
