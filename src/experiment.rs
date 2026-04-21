use std::{fs, io::BufReader};
use std::rc::Rc;

mod sweep_parameter;
mod experiment_results;

pub use sweep_parameter::SweepParameter;

use experiment_results::{ExperimentResult, get_experiment_result_from_json};

use crate::{
    blueprints::{CircuitBlueprint, QubitArrayBlueprint, SimulationTimesBlueprint},
    simulation::Simulator,
};

use ndarray::{ArrayD};
use hdf5::Result;
use serde_json::{Map, Value};
use indicatif::ProgressBar;

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
    results: Box<dyn ExperimentResult>,
}

impl Experiment {
    /// Get an experiment object from a json file name.
    pub fn from_json(filename: &str) -> Experiment {
        // File and reader to read the experiment config from
        let file: fs::File = fs::File::open(filename).unwrap();
        let reader: BufReader<fs::File> = BufReader::new(file);

        // Json values read in from the file
        let mut json_values: Map<String, Value> = serde_json::from_reader(reader).unwrap();

        // There should be a map of values for the circuit blueprint under the "circuit" key
        let mut circuit_json: Value = json_values.remove("circuit").unwrap();

        // If there is a key in the circuit map called "filename" then assume that is a seperate
        // file defining the circuit and look there
        if circuit_json.as_object().unwrap().contains_key("filename") {
            let circuit_file: fs::File =
                fs::File::open(circuit_json["filename"].as_str().unwrap()).unwrap();
            let circuit_reader: BufReader<fs::File> = BufReader::new(circuit_file);
            circuit_json = serde_json::from_reader(circuit_reader).unwrap();
        }

        // Similar to the circuit read in for "qubits" to get the qubit array json values and look
        // as well for the "filename" key
        let mut qubit_json: Value = json_values.remove("qubits").unwrap();
        if qubit_json.as_object().unwrap().contains_key("filename") {
            let qubit_file: fs::File =
                fs::File::open(circuit_json["filename"].as_str().unwrap()).unwrap();
            let qubit_reader: BufReader<fs::File> = BufReader::new(qubit_file);
            qubit_json = serde_json::from_reader(qubit_reader).unwrap();
        }

        // Vector to hold the sweep parameters for the experiment
        let mut sweep_parameters: Vec<SweepParameter> = vec![];

        // Construct the circuit blueprint and collect the sweep parameters
        let (circuit_blueprint, mut circuit_sweep_parameters): (
            CircuitBlueprint,
            Vec<SweepParameter>,
        ) = CircuitBlueprint::from_json(circuit_json.as_object().unwrap());
        for sweep_parameter in &mut circuit_sweep_parameters {
            sweep_parameter.add_path("circuit".to_string());
            sweep_parameter.reverse_path(); // Path is reversed so it reads front to back
        }
        sweep_parameters.append(&mut circuit_sweep_parameters);

        // Same for the qubit array blueprint. Construct and collect swept parameters
        let (qubit_array_blueprint, mut qubit_array_sweep_parameters): (
            QubitArrayBlueprint,
            Vec<SweepParameter>,
        ) = QubitArrayBlueprint::from_json(qubit_json.as_object().unwrap());
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
            results: get_experiment_result_from_json(&json_values["output"].as_object().unwrap(), Rc::clone(&rc_sweep_parameters)),
        };
    }
    /// Run the experiment defined in this class and save the results to the given filename
    pub fn run_experiment(&mut self, filename: &String) -> Result<()> {
        // Dimensions of the results
        let results_dim: Vec<usize> = self.get_results_dimension();

        // Number of iterations to go through all the parameters
        let mut num_experiment_iterations: usize = 1;
        for i in &results_dim {
            num_experiment_iterations *= i;
        }

        // Array for results of experiment
        // let mut results: ArrayD<f64> = ArrayD::<f64>::zeros(IxDyn(&results_dim));

        // Vector of the current index for each of the swept parameters
        let mut sweep_parameter_indicies: Vec<usize> = results_dim.iter().map(|_| 0).collect();

        // Make a progress bar to display how fast the experiment is going
        let progress_bar: ProgressBar = ProgressBar::new(num_experiment_iterations as u64);
        
        // Loop the total number of iterations needed to get through all swept values
        for _i in 0..num_experiment_iterations {
            // Construct and simulate the given circuit and qubit array and save the final
            // probability ot be in the -Z state
            let sim_result = Simulator::new()
                .simulate_circuit(
                    self.circuit_blueprint.get_circuit(),
                    self.qubit_array_blueprint.get_qubit_array(),
                    self.simulation_times_blueprint.get_num_iterations(),
                    self.simulation_times_blueprint.get_num_samples(),
                );
                // .get_final_probability();
            // Set the value in the results
            // results[IxDyn(&sweep_parameter_indicies)] = sim_result;
            self.results.add_simulation_result(&sweep_parameter_indicies, sim_result);
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
        // Save teh results and save the circuit data
        self.results.save(filename.clone())?;
        // self.save_results(results, &mut filename.clone())?;
        self.circuit_blueprint.get_circuit().save_circuit_data();
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
    /// Save a given array of results to an HDF5 file. The results are N Dimensional where N should
    /// be the number of swept parameters. The size in each dimension corresponds to the number of
    /// values for the parameter across that axis. The file is saved under the given filename
    fn save_results(&self, results: ArrayD<f64>, filename: &mut String) -> Result<()> {
        // Add the .h5 extension to the filename
        filename.push_str(".h5");

        // Open an HDF5 file under the given name and make a parameters group
        let file = hdf5::File::create(filename)?;
        let group = file.create_group("parameters")?;

        // Loop over all the swept parameters in this experiment
        for (i, sweep_parameter) in self.sweep_parameters.iter().enumerate() {
            // Construct a builder for this parameter
            let builder = group.new_dataset_builder();
            // Build a dataset with the values this parameter is swept over
            let parameter_ds = builder
                .with_data(&sweep_parameter.get_values())
                .create(sweep_parameter.get_full_path().as_str())?;
            // Create at attribute for this parameter and write which number axis this parameter is
            let attr = parameter_ds.new_attr::<usize>().shape([1]).create("axis")?;
            attr.write(&[i])?;
        }
        // Make a builder and put the results data set into the file
        let builder = file.new_dataset_builder();
        let _ds = builder.with_data(&results).create("results")?;
        return Ok(());
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
