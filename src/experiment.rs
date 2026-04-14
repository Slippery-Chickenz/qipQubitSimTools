use crate::{
    circuit::Circuit,
    gate::{Idle, PiO2X, PiO2Y},
    qubit_array::QubitArray,
    simulator::Simulator,
};

use ndarray::{Array2, array};

#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, H5Type, Result};

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
                let mut circuit: Circuit =
                    Circuit::from_vec(vec![PiO2X::new(), Idle::new(*tau)]);
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
        let file = File::create("ramsey_results.h5")?; // open for writing

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
