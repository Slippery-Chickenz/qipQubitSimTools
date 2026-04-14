use std::fs;
use std::io::BufReader;

use crate::{
    circuit::Circuit,
    circuit_blueprint::CircuitBlueprint,
    gate::{Idle, PiO2X, PiO2Y},
    qubit_array::QubitArray,
    qubit_array_blueprint::QubitArrayBlueprint,
    simulator::Simulator,
};

use ndarray::{Array2, ArrayD, IxDyn};
use num_complex::{Complex, Complex64};

use hdf5::{H5Type, Result};

use serde_json::{Map, Value};

#[derive(Debug)]
pub struct Experiment {
    circuit_blueprint: CircuitBlueprint,
    qubit_array_blueprint: QubitArrayBlueprint,
    num_iterations: usize,
    num_samples: usize,
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

        return Experiment {
            circuit_blueprint: CircuitBlueprint::from_json(circuit_json.as_object().unwrap()),
            qubit_array_blueprint: QubitArrayBlueprint::from_json(qubit_json.as_object().unwrap()),
            num_iterations: json_values["num_iterations"].as_u64().unwrap() as usize,
            num_samples: json_values["num_samples"].as_u64().unwrap() as usize,
        };
    }
    pub fn run_experiment(&self) -> Result<()> {

        // Dimensions of the results
        let results_dim: Vec<usize> = self.get_results_dimension();

        // Array of the results of the experiment
        let results: ArrayD<Complex64> = Simulator::new().simulate_circuit(self.circuit_blueprint.get_circuit(), self.qubit_array_blueprint.get_qubit_array(), self.num_iterations, self.num_samples).get_probabilities().mapv(|x| Complex64::new(x, 0.)).into_dyn();


        return Ok(());
    }
    fn save_results(&self, results: ArrayD<Complex64>) -> Result<()> {

        let file = hdf5::File::create("ramsey_results.h5")?; // open for writing

        // let group = file.create_group("parameters")?; // create a group
        //
        // let builder = group.new_dataset_builder();
        // let larmor_ds = builder
        //     .with_data(&self.guess_larmors)
        //     .create("guess_larmors")?;
        // // create an attr with fixed shape but don't write the data
        // let attr = larmor_ds
        //     .new_attr::<usize>()
        //     .shape([1])
        //     .create("guess_larmors")?;
        // // write the attr data
        // attr.write(&[0])?;
        //
        // let builder = group.new_dataset_builder();
        // let taus_ds = builder.with_data(&self.taus).create("taus")?;
        // // create an attr with fixed shape but don't write the data
        // let attr = taus_ds.new_attr::<usize>().shape([1]).create("taus")?;
        // // write the attr data
        // attr.write(&[1])?;

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

        // If there is more than one sample then add a dimension of that size
        if self.num_samples > 1 {
            dim_vec.push(self.num_samples);
        }

        return dim_vec;
    }
}

pub enum RamseyEndGate {
    X,
    Y,
}

pub struct RamseyExperiment {
    end_gate: RamseyEndGate,
    guess_larmors: Vec<f64>,
    taus: Vec<f64>,
}

impl RamseyExperiment {
    pub fn new(
        end_gate: RamseyEndGate,
        guess_larmors: Vec<f64>,
        taus: Vec<f64>,
    ) -> RamseyExperiment {
        return RamseyExperiment {
            end_gate: end_gate,
            guess_larmors: guess_larmors,
            taus: taus,
        };
    }
    pub fn run_experiment(&self) -> Result<()> {
        let mut experiment_results: Array2<f64> =
            Array2::<f64>::zeros([self.taus.len(), self.guess_larmors.len()]);

        for (i, guess_larmor) in self.guess_larmors.iter().enumerate() {
            for (j, tau) in self.taus.iter().enumerate() {
                let mut circuit: Circuit = Circuit::from_vec(vec![PiO2X::new(), Idle::new(*tau)]);
                match self.end_gate {
                    RamseyEndGate::X => circuit.add_gate(PiO2X::new()),
                    RamseyEndGate::Y => circuit.add_gate(PiO2Y::new()),
                }

                experiment_results[[i, j]] = Simulator::new()
                    .simulate_circuit(circuit, QubitArray::new(1, 0., *guess_larmor), 100, 2)
                    .get_final_probability();
            }
        }

        self.save_experiment(experiment_results)?;
        return Ok(());
    }
    fn save_experiment(&self, experiment_results: Array2<f64>) -> Result<()> {
        let file = hdf5::File::create("ramsey_results.h5")?; // open for writing

        let group = file.create_group("parameters")?; // create a group

        let builder = group.new_dataset_builder();
        let larmor_ds = builder
            .with_data(&self.guess_larmors)
            .create("guess_larmors")?;
        // create an attr with fixed shape but don't write the data
        let attr = larmor_ds
            .new_attr::<usize>()
            .shape([1])
            .create("guess_larmors")?;
        // write the attr data
        attr.write(&[0])?;

        let builder = group.new_dataset_builder();
        let taus_ds = builder.with_data(&self.taus).create("taus")?;
        // create an attr with fixed shape but don't write the data
        let attr = taus_ds.new_attr::<usize>().shape([1]).create("taus")?;
        // write the attr data
        attr.write(&[1])?;

        let builder = file.new_dataset_builder();
        let _ds = builder
            .with_data(&experiment_results)
            // finalize and write the dataset
            .create("results")?;
        return Ok(());
    }
}
