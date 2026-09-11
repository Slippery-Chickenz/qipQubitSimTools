use std::rc::Rc;
use std::{fs, io::BufReader};

mod adiabaticity_results;
mod bloch_coord_results;
mod duration_result;
mod eigenstate_results;
mod experiment_results;
mod hamiltonian_results;
mod measurement_results;
mod probability_results;
mod state_results;
mod sweep_parameter;
mod time_results;
mod waveform_saver;

pub use waveform_saver::WaveformSaver;

pub use sweep_parameter::SweepParameter;

use experiment_results::ExperimentResults;

use crate::blueprints::SimulationSettingsBlueprint;
use crate::simulation::{PadeStateMethod, PadeVectorizedMethod, RKMethod, RKVectorizedMethod};
use crate::{
    blueprints::{CircuitBlueprint, QubitArrayBlueprint, SimulationMethodType},
    simulation::Simulator,
};

use hdf5::Result;
use indicatif::ProgressBar;
use ndarray::Array1;
use serde_json::{Map, Value};

/// Error to be returned if construction of an experiment failed
pub enum ExperimentConstructionError {}

/// Experiment to be run. Consists of a circuit, qubit array, and simulation times to simulate and
/// then a vector of parameters and values to sweep across and run simulations for each combination
/// of parameters.
#[derive(Debug)]
pub struct Experiment {
    /// Blueprint to construct a circuit to simulate
    circuit_blueprint: CircuitBlueprint,
    /// Blueprint to construct an array of qubits to run the circuit on
    qubit_array_blueprint: QubitArrayBlueprint,
    /// Blueprint to construct simulation times to run the simulation on
    simulation_settings_blueprint: SimulationSettingsBlueprint,
    /// Vector of parameters to sweep across and run the simulation at each value
    sweep_parameters: Rc<Vec<SweepParameter>>,
    /// Object to store and save the results in. Dynamic depending on what is defined to save
    results: ExperimentResults,
    /// Flag to save waveform or not
    save_waveform: bool,
    /// Flag to save hamiltonians in the simulation or not
    save_hamiltonians: bool,
}

impl Experiment {
    /// Get an experiment object from a json file name
    pub fn from_json_file(filename: &str) -> Result<Experiment, std::io::Error> {
        // File and reader to read the experiment config from
        let file: fs::File = fs::File::open(filename)?;
        let reader: BufReader<fs::File> = BufReader::new(file);

        // Json values read in from the file
        let json_values: Map<String, Value> = serde_json::from_reader(reader)?; //.unwrap();
        return Ok(Experiment::from_json(json_values));
    }
    /// Get an experiment object from a map of strings to json values
    pub fn from_json(mut json_values: Map<String, Value>) -> Experiment {
        // Vector to hold the sweep parameters for the experiment
        let mut sweep_parameters: Vec<SweepParameter> = vec![];

        // Construct the circuit blueprint and collect the sweep parameters
        let (circuit_blueprint, mut circuit_sweep_parameters): (
            CircuitBlueprint,
            Vec<SweepParameter>,
        ) = CircuitBlueprint::from_json(
            serde_json::from_value(json_values.remove("circuit").unwrap()).unwrap(),
        );
        for sweep_parameter in &mut circuit_sweep_parameters {
            sweep_parameter.add_path("circuit".to_string());
            sweep_parameter.reverse_path(); // Path is reversed so it reads front to back
        }
        sweep_parameters.append(&mut circuit_sweep_parameters);

        // Same for the qubit array blueprint. Construct and collect swept parameters
        let (qubit_array_blueprint, mut qubit_array_sweep_parameters): (
            QubitArrayBlueprint,
            Vec<SweepParameter>,
        ) = QubitArrayBlueprint::from_json(
            serde_json::from_value(json_values.remove("qubits").unwrap()).unwrap(),
        );
        for sweep_parameter in &mut qubit_array_sweep_parameters {
            sweep_parameter.add_path("qubits".to_string());
            sweep_parameter.reverse_path();
        }
        sweep_parameters.append(&mut qubit_array_sweep_parameters);

        let num_shots: i64 = json_values
            .get("shots")
            .unwrap_or_default()
            .as_i64()
            .unwrap_or(1);

        if num_shots > 1 {
            sweep_parameters.append(&mut vec![SweepParameter::new(
                vec!["shots".to_string()],
                Array1::<f64>::linspace(0., num_shots as f64, num_shots as usize).to_vec(),
                Option::Some(-1),
            )]);
        }

        let simulation_settings_blueprint: SimulationSettingsBlueprint =
            SimulationSettingsBlueprint::from_json(
                serde_json::from_value(json_values.remove("simulation_settings").unwrap()).unwrap(),
            );

        // Rc of sweep parameters to save here and also send to results
        let rc_sweep_parameters: Rc<Vec<SweepParameter>> = Rc::new(sweep_parameters);

        let (results, save_hamiltonians, save_waveform): (ExperimentResults, bool, bool) =
            ExperimentResults::from_json(
                serde_json::from_value(json_values.remove("output").unwrap()).unwrap(),
                Rc::clone(&rc_sweep_parameters),
                simulation_settings_blueprint.get_num_samples(),
            );

        return Experiment {
            circuit_blueprint: circuit_blueprint,
            qubit_array_blueprint: qubit_array_blueprint,
            simulation_settings_blueprint: simulation_settings_blueprint,
            sweep_parameters: Rc::clone(&rc_sweep_parameters),
            results: results,
            save_waveform: save_waveform,
            save_hamiltonians: save_hamiltonians,
        };
    }
    /// Run the experiment defined in this class and save the results to the given filename
    pub fn run_experiment(&mut self, filename: &str) -> Result<()> {
        // Dimensions of the results
        let results_dim: Vec<usize> =
            Vec::from_iter((*self.sweep_parameters).iter().map(|x| x.values_len()));

        // Number of iterations to go through all the parameters
        let num_experiment_iterations: usize = results_dim.iter().product();

        // Vector of the current index for each of the swept parameters
        let mut sweep_parameter_indicies: Vec<usize> = results_dim.iter().map(|_| 0).collect();

        // Make a progress bar to display how fast the experiment is going
        let progress_bar: ProgressBar = ProgressBar::new(num_experiment_iterations as u64);

        // Send the first simulated circuit to the results to save waveform
        if self.save_waveform {
            self.results
                .save_circuit(self.circuit_blueprint.get_circuit());
        }

        // Loop the total number of iterations needed to get through all swept values
        for _i in 0..num_experiment_iterations {
            match self.simulation_settings_blueprint.get_simulation_method() {
                SimulationMethodType::RKMethod => self.results.add_simulation_result(
                    &sweep_parameter_indicies,
                    &Simulator::<RKMethod>::simulate_circuit(
                        self.circuit_blueprint.get_circuit(),
                        self.qubit_array_blueprint.get_qubit_array(),
                        self.simulation_settings_blueprint
                            .get_reference_frame()
                            .clone(),
                        self.simulation_settings_blueprint.get_step_size(),
                        self.simulation_settings_blueprint.get_num_samples(),
                        self.save_hamiltonians,
                    ),
                ),
                SimulationMethodType::RKVectorizedMethod => self.results.add_simulation_result(
                    &sweep_parameter_indicies,
                    &Simulator::<RKVectorizedMethod>::simulate_circuit(
                        self.circuit_blueprint.get_circuit(),
                        self.qubit_array_blueprint.get_qubit_array(),
                        self.simulation_settings_blueprint
                            .get_reference_frame()
                            .clone(),
                        self.simulation_settings_blueprint.get_step_size(),
                        self.simulation_settings_blueprint.get_num_samples(),
                        self.save_hamiltonians,
                    ),
                ),
                SimulationMethodType::PadeVectorizedMethod => self.results.add_simulation_result(
                    &sweep_parameter_indicies,
                    &Simulator::<PadeVectorizedMethod>::simulate_circuit(
                        self.circuit_blueprint.get_circuit(),
                        self.qubit_array_blueprint.get_qubit_array(),
                        self.simulation_settings_blueprint
                            .get_reference_frame()
                            .clone(),
                        self.simulation_settings_blueprint.get_step_size(),
                        self.simulation_settings_blueprint.get_num_samples(),
                        self.save_hamiltonians,
                    ),
                ),
                SimulationMethodType::PadeStateMethod => self.results.add_simulation_result(
                    &sweep_parameter_indicies,
                    &Simulator::<PadeStateMethod>::simulate_circuit(
                        self.circuit_blueprint.get_circuit(),
                        self.qubit_array_blueprint.get_qubit_array(),
                        self.simulation_settings_blueprint
                            .get_reference_frame()
                            .clone(),
                        self.simulation_settings_blueprint.get_step_size(),
                        self.simulation_settings_blueprint.get_num_samples(),
                        self.save_hamiltonians,
                    ),
                ),
            }
            // Loop over the indicies of the swept parameters and increase them
            for j in 0..sweep_parameter_indicies.len() {
                // Increase the parameter index
                sweep_parameter_indicies[j] += 1;
                // If the parameter index that was just increased is the last one for that
                // parameter then reset it and go onto the next parameter index
                if sweep_parameter_indicies[j] >= self.sweep_parameters[j].values_len() {
                    sweep_parameter_indicies[j] = 0;
                } else {
                    // If it was not the last one for that parameter then just break and only
                    // increase that one
                    break;
                }
            }
            // Update the parameters to set the values at the given indicies
            self.update_parameters(&sweep_parameter_indicies);
            progress_bar.inc(1);
        }
        // Save the results and save the circuit data
        self.results.save(filename)?;
        progress_bar.finish();
        return Ok(());
    }

    /// Update the parameters for the blueprints for a given set of indicies. The indicies
    /// correspond to the vector of values held in the sweep parameter.
    fn update_parameters(&mut self, sweep_parameter_indicies: &Vec<usize>) -> () {
        // Loop over all the sweep parameters
        for (i, sweep_parameter) in self.sweep_parameters.iter().enumerate() {
            // Match the first item in the parameter path to either the circuit or the qubits
            match sweep_parameter.get_path(0).as_str() {
                // Update the corresponding blueprint
                "circuit" => self.circuit_blueprint.update_parameters(
                    sweep_parameter,
                    1,
                    sweep_parameter_indicies[i],
                ),
                "qubits" => self.qubit_array_blueprint.update_parameters(
                    sweep_parameter,
                    1,
                    sweep_parameter_indicies[i],
                ),
                _ => return,
            }
        }
        return;
    }
}
