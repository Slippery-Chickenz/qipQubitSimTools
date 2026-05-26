use std::rc::Rc;
use std::{fs, io::BufReader};

mod bloch_coord_results;
mod duration_result;
mod experiment_results;
mod probability_results;
mod sweep_parameter;

pub use sweep_parameter::SweepParameter;

use experiment_results::ExperimentResults;

use crate::simulation::{PadeStateMethod, PadeVectorizedMethod, RKMethod, RKVectorizedMethod};
use crate::{
    blueprints::{CircuitBlueprint, QubitArrayBlueprint, SimulationTimesBlueprint},
    simulation::Simulator,
};

use hdf5::Result;
use indicatif::ProgressBar;
use serde_json::{Map, Value};

#[derive(Debug)]
enum SimulationMethodType {
    RKMethod,
    RKVectorizedMethod,
    PadeVectorizedMethod,
    PadeStateMethod,
}

impl From<&str> for SimulationMethodType {
    fn from(value: &str) -> SimulationMethodType {
        match value {
            "RK" => return SimulationMethodType::RKMethod,
            "RKVectorized" => return SimulationMethodType::RKVectorizedMethod,
            "PadeVectorized" => return SimulationMethodType::PadeVectorizedMethod,
            "PadeState" => return SimulationMethodType::PadeStateMethod,
            _ => return SimulationMethodType::RKMethod,
        }
    }
}

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
    simulation_times_blueprint: SimulationTimesBlueprint,
    /// Vector of parameters to sweep across and run the simulation at each value
    sweep_parameters: Rc<Vec<SweepParameter>>,
    /// Object to store and save the results in. Dynamic depending on what is defined to save
    results: ExperimentResults,
    /// Simulation method to run the experiment with
    simulation_method: SimulationMethodType,
}

impl Experiment {
    /// Get an experiment object from a json file name
    pub fn from_json_file(filename: &str) -> Experiment {
        // File and reader to read the experiment config from
        let file: fs::File = fs::File::open(filename).unwrap();
        let reader: BufReader<fs::File> = BufReader::new(file);

        // Json values read in from the file
        let json_values: Map<String, Value> = serde_json::from_reader(reader).unwrap();
        return Experiment::from_json(json_values);
    }
    /// Get an experiment object from a map of strings to json values
    pub fn from_json(mut json_values: Map<String, Value>) -> Experiment {
        // String representing the simulation method to use. If there is no string resort to RKMethod
        let sim_method_string: &str = json_values
            .get("method")
            .unwrap_or_default()
            .as_str()
            .unwrap_or("");
        let sim_method: SimulationMethodType = SimulationMethodType::from(sim_method_string);

        // Vector to hold the sweep parameters for the experiment
        let mut sweep_parameters: Vec<SweepParameter> = vec![];

        // There should be a map of values for the circuit blueprint under the "circuit" key
        let circuit_json: Map<String, Value> =
            serde_json::from_value(json_values.remove("circuit").unwrap()).unwrap();

        // Construct the circuit blueprint and collect the sweep parameters
        let (circuit_blueprint, mut circuit_sweep_parameters): (
            CircuitBlueprint,
            Vec<SweepParameter>,
        ) = CircuitBlueprint::from_json(circuit_json);
        for sweep_parameter in &mut circuit_sweep_parameters {
            sweep_parameter.add_path("circuit".to_string());
            sweep_parameter.reverse_path(); // Path is reversed so it reads front to back
        }
        sweep_parameters.append(&mut circuit_sweep_parameters);

        // There should be a map of values for the circuit blueprint under the "circuit" key
        let qubit_json: Map<String, Value> =
            serde_json::from_value(json_values.remove("qubits").unwrap()).unwrap();

        // Same for the qubit array blueprint. Construct and collect swept parameters
        let (qubit_array_blueprint, mut qubit_array_sweep_parameters): (
            QubitArrayBlueprint,
            Vec<SweepParameter>,
        ) = QubitArrayBlueprint::from_json(qubit_json);
        for sweep_parameter in &mut qubit_array_sweep_parameters {
            sweep_parameter.add_path("qubits".to_string());
            sweep_parameter.reverse_path();
        }
        sweep_parameters.append(&mut qubit_array_sweep_parameters);

        // Rc of sweep parameters to save here and also send to results
        let rc_sweep_parameters: Rc<Vec<SweepParameter>> = Rc::new(sweep_parameters);

        return Experiment {
            circuit_blueprint: circuit_blueprint,
            qubit_array_blueprint: qubit_array_blueprint,
            simulation_times_blueprint: SimulationTimesBlueprint::from_json(&json_values),
            sweep_parameters: Rc::clone(&rc_sweep_parameters),
            results: ExperimentResults::from_json(
                &json_values["output"].as_object().unwrap(),
                Rc::clone(&rc_sweep_parameters),
            ),
            simulation_method: sim_method,
        };
    }
    /// Run the experiment defined in this class and save the results to the given filename
    pub fn run_experiment(&mut self, filename: &str) -> Result<()> {
        // Dimensions of the results
        let results_dim: Vec<usize> = self.get_results_dimension();

        // Number of iterations to go through all the parameters
        let mut num_experiment_iterations: usize = 1;
        for i in &results_dim {
            num_experiment_iterations *= i;
        }

        // Vector of the current index for each of the swept parameters
        let mut sweep_parameter_indicies: Vec<usize> = results_dim.iter().map(|_| 0).collect();

        // Make a progress bar to display how fast the experiment is going
        let progress_bar: ProgressBar = ProgressBar::new(num_experiment_iterations as u64);

        // Loop the total number of iterations needed to get through all swept values
        for _i in 0..num_experiment_iterations {
            match self.simulation_method {
                SimulationMethodType::RKMethod => self.results.add_simulation_result(
                    &sweep_parameter_indicies,
                    &Simulator::<RKMethod>::simulate_circuit(
                        self.circuit_blueprint.get_circuit(),
                        self.qubit_array_blueprint.get_qubit_array(),
                        self.simulation_times_blueprint.get_step_size(),
                        self.simulation_times_blueprint.get_num_samples(),
                    ),
                ),
                SimulationMethodType::RKVectorizedMethod => self.results.add_simulation_result(
                    &sweep_parameter_indicies,
                    &Simulator::<RKVectorizedMethod>::simulate_circuit(
                        self.circuit_blueprint.get_circuit(),
                        self.qubit_array_blueprint.get_qubit_array(),
                        self.simulation_times_blueprint.get_step_size(),
                        self.simulation_times_blueprint.get_num_samples(),
                    ),
                ),
                SimulationMethodType::PadeVectorizedMethod => self.results.add_simulation_result(
                    &sweep_parameter_indicies,
                    &Simulator::<PadeVectorizedMethod>::simulate_circuit(
                        self.circuit_blueprint.get_circuit(),
                        self.qubit_array_blueprint.get_qubit_array(),
                        self.simulation_times_blueprint.get_step_size(),
                        self.simulation_times_blueprint.get_num_samples(),
                    ),
                ),
                SimulationMethodType::PadeStateMethod => self.results.add_simulation_result(
                    &sweep_parameter_indicies,
                    &Simulator::<PadeStateMethod>::simulate_circuit(
                        self.circuit_blueprint.get_circuit(),
                        self.qubit_array_blueprint.get_qubit_array(),
                        self.simulation_times_blueprint.get_step_size(),
                        self.simulation_times_blueprint.get_num_samples(),
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
    // Get the dimensions for the results vector
    fn get_results_dimension(&self) -> Vec<usize> {
        // Vector to hold the dimensions of the results
        let mut dim_vec: Vec<usize> = vec![];
        // Loop over the sweep parameters and add the len of the values as the length of the dimension
        for sweep_parameter in &*self.sweep_parameters {
            dim_vec.push(sweep_parameter.values_len());
        }
        return dim_vec;
    }
}
