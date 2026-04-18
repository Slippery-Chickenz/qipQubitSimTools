use std::{fs};
use std::io::BufReader;

use crate::sweep_parameter::{SweepParameter};
use crate::{
    circuit_blueprint::CircuitBlueprint,
    qubit_array_blueprint::QubitArrayBlueprint,
    simulation_times_blueprint::SimulationTimesBlueprint,
    simulator::Simulator,
};

use ndarray::{ArrayD, IxDyn};

use hdf5::{Result};

use serde_json::{Map, Value};

#[derive(Debug)]
pub struct Experiment {
    circuit_blueprint: CircuitBlueprint,
    qubit_array_blueprint: QubitArrayBlueprint,
    simulation_times_blueprint: SimulationTimesBlueprint,
    sweep_parameters: Vec<SweepParameter>,
}

impl Experiment {
    pub fn from_json(filename: &str) -> Experiment {
        let file: fs::File = fs::File::open(filename).unwrap();
        let reader: BufReader<fs::File> = BufReader::new(file);

        let mut json_values: Map<String, Value> = serde_json::from_reader(reader).unwrap();

        let mut circuit_json: Value = json_values.remove("circuit").unwrap();

        if circuit_json.as_object().unwrap().contains_key("filename") {
            let circuit_file: fs::File =
                fs::File::open(circuit_json["filename"].as_str().unwrap()).unwrap();
            let circuit_reader: BufReader<fs::File> = BufReader::new(circuit_file);
            circuit_json = serde_json::from_reader(circuit_reader).unwrap();
        }

        let mut qubit_json: Value = json_values.remove("qubits").unwrap();

        if qubit_json.as_object().unwrap().contains_key("filename") {
            let qubit_file: fs::File =
                fs::File::open(circuit_json["filename"].as_str().unwrap()).unwrap();
            let qubit_reader: BufReader<fs::File> = BufReader::new(qubit_file);
            qubit_json = serde_json::from_reader(qubit_reader).unwrap();
        }

        let mut sweep_parameters: Vec<SweepParameter> = vec![];

        let (circuit_blueprint, mut circuit_sweep_parameters): (
            CircuitBlueprint,
            Vec<SweepParameter>,
        ) = CircuitBlueprint::from_json(circuit_json.as_object().unwrap());

        for sweep_parameter in &mut circuit_sweep_parameters {
            sweep_parameter.add_path("circuit".to_string());
            sweep_parameter.reverse_path();
        }
        sweep_parameters.append(&mut circuit_sweep_parameters);

        let (qubit_array_blueprint, mut qubit_array_sweep_parameters): (
            QubitArrayBlueprint,
            Vec<SweepParameter>,
        ) = QubitArrayBlueprint::from_json(qubit_json.as_object().unwrap());

        for sweep_parameter in &mut qubit_array_sweep_parameters {
            sweep_parameter.add_path("qubits".to_string());
            sweep_parameter.reverse_path();
        }
        sweep_parameters.append(&mut qubit_array_sweep_parameters);

        return Experiment {
            circuit_blueprint: circuit_blueprint,
            qubit_array_blueprint: qubit_array_blueprint,
            simulation_times_blueprint: SimulationTimesBlueprint::from_json(&json_values),
            sweep_parameters: sweep_parameters,
        };
    }
    pub fn run_experiment(&mut self, filename: &String) -> Result<()> {
        // Dimensions of the results
        let results_dim: Vec<usize> = self.get_results_dimension();

        // Number of iterations to go through all the parameters
        let mut num_experiment_iterations: usize = 1;
        for i in &results_dim {
            num_experiment_iterations *= i;
        }

        // Array for results of experiment
        let mut results: ArrayD<f64> = ArrayD::<f64>::zeros(IxDyn(&results_dim));

        // Vector of the current index for each of the swept parameters
        let mut sweep_parameter_indicies: Vec<usize> = results_dim.iter().map(|_| 0).collect();

        for _i in 0..num_experiment_iterations {
            let sim_result = Simulator::new()
                .simulate_circuit(
                    self.circuit_blueprint.get_circuit(),
                    self.qubit_array_blueprint.get_qubit_array(),
                    self.simulation_times_blueprint.get_num_iterations(),
                    self.simulation_times_blueprint.get_num_samples(),
                ).get_final_probability();
            results[IxDyn(&sweep_parameter_indicies)] = sim_result;
            for j in 0..sweep_parameter_indicies.len() {
                sweep_parameter_indicies[j] += 1;
                if sweep_parameter_indicies[j] >= self.sweep_parameters[j].values_len() {
                    sweep_parameter_indicies[j] = 0;
                }
                else {
                    break;
                }
            }
            self.update_parameters(&sweep_parameter_indicies);
        }
        self.save_results(results, &mut filename.clone())?;
        self.circuit_blueprint.get_circuit().save_circuit_data();
        return Ok(());
    }
    fn update_parameters(&mut self, sweep_parameter_indicies: &Vec<usize>) -> () {

        for (i, sweep_parameter) in self.sweep_parameters.iter().enumerate() {
            match sweep_parameter.get_path(0).as_str() {
                "circuit" => self.circuit_blueprint.update_parameters(sweep_parameter, 1, sweep_parameter_indicies[i]),
                "qubits" => self.qubit_array_blueprint.update_parameters(sweep_parameter, 1, sweep_parameter_indicies[i]),
                _ => return,
            }
        }
        return;
    }
    fn save_results(&self, results: ArrayD<f64>, filename: &mut String) -> Result<()> {

        filename.push_str(".h5");
        let file = hdf5::File::create(filename)?; // open for writing
        let group = file.create_group("parameters")?; // create a group

        for (i, sweep_parameter) in self.sweep_parameters.iter().enumerate() {
            let builder = group.new_dataset_builder();
            let larmor_ds = builder
                .with_data(&sweep_parameter.get_values())
                .create(sweep_parameter.get_full_path().as_str())?;
            // create an attr with fixed shape but don't write the data
            let attr = larmor_ds
                .new_attr::<usize>()
                .shape([1])
                .create("axis")?;
            // write the attr data
            attr.write(&[i])?;
        }

        let builder = file.new_dataset_builder();
        let _ds = builder
            .with_data(&results)
            // finalize and write the dataset
            .create("results")?;
        return Ok(());
    }
    fn get_results_dimension(&self) -> Vec<usize> {
        // Vector to hold the dimensions of the results
        let mut dim_vec: Vec<usize> = vec![];

        for sweep_parameter in &self.sweep_parameters {
            dim_vec.push(sweep_parameter.values_len());
        }

        // If there is more than one sample then add a dimension of that size
        // if self.simulation_times_blueprint.get_num_samples() > 1 {
        //     dim_vec.push(self.simulation_times_blueprint.get_num_samples());
        // }

        return dim_vec;
    }
}
