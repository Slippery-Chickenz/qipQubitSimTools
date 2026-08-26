use crate::simulation::Circuit;

use hdf5::{Group, Result};
use ndarray::{Array1, Array2};

#[derive(Debug)]
pub struct WaveformSaver {
    /// Frequency throughout the waveform
    frequency: Array1<f64>,
    /// Amplitude throughout the waveform
    amplitude: Array1<f64>,
    /// Phase throughout the waveform
    phase: Array1<f64>,
    /// I&Q throughout the waveform
    pulse_iq: Array2<f64>,
}

impl WaveformSaver {
    pub fn from_circuit(circuit: Circuit) -> WaveformSaver {
        // Get the waveform data to save
        let (frequency_data, amplitude_data, phase_data, pulse_data): (
            Array1<f64>,
            Array1<f64>,
            Array1<f64>,
            Array2<f64>,
        ) = circuit.get_circuit_data();
        return WaveformSaver {
            frequency: frequency_data,
            amplitude: amplitude_data,
            phase: phase_data,
            pulse_iq: pulse_data,
        };
    }
    /// Save the waveform to a given hdf5 group
    pub fn save(&self, group: &Group) -> Result<()> {
        // Make a builder and put the results data set into the file
        let builder = group.new_dataset_builder();
        let _ds = builder.with_data(&self.frequency).create("frequency")?;
        let builder = group.new_dataset_builder();
        let _ds = builder.with_data(&self.amplitude).create("amplitude")?;
        let builder = group.new_dataset_builder();
        let _ds = builder.with_data(&self.phase).create("phase")?;
        let builder = group.new_dataset_builder();
        let _ds = builder.with_data(&self.pulse_iq).create("pulse_iq")?;
        return Ok(());
    }
}
