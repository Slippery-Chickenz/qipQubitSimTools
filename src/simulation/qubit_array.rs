use std::f64::consts::PI;
use std::rc::Rc;

use super::simulation_times::UninitializedTimesError;
use crate::simulation::{LarmorFrequency, SimulationTimes};

use ndarray::linalg::kron;
use ndarray::{Array2, array};
use num_complex::Complex64;

/// Qubit array to be used in a simulation. Holds the number of qubits (currently only supports 1)
/// the starting density matrix of the qubits, the larmor of each qubit and the guess at what the
/// larmor is for each qubit
pub struct QubitArray {
    /// Number of qubits in the array (currently only supports 1)
    num_qubts: u32,
    /// Starting density matrix of the qubits
    density_matrix: Array2<Complex64>,
    /// Larmor value of the qubit
    larmor: LarmorFrequency,
    /// Lifetime for the non-unitary continuous dephasing effect
    dephasing_lifetime: f64,
    /// Guess at the larmor value for the qubits
    guess_larmor: f64,
    /// Simulation times for the simulation
    simulation_times: Option<Rc<SimulationTimes>>,
}

impl QubitArray {
    /// Get a QubitArray object with a given number of qubits with a certain larmor and guess
    /// larmor. This sets the starting density matrix to be in the +z state e.g. (1, 0)
    pub fn new(
        num_qubits: u32,
        larmor: LarmorFrequency,
        dephasing_lifetime: f64,
        guess_larmor: f64,
    ) -> QubitArray {
        // Set the density matrix as a kronecker product of the +z state for each qubit
        let mut density_matrix: Array2<Complex64> = Array2::<Complex64>::zeros((2, 2));
        density_matrix[[0, 0]] = Complex64::new(1., 0.);
        let temp_matrix: Array2<Complex64> = density_matrix.clone();
        for _i in 0..num_qubits - 1 {
            density_matrix = kron(&density_matrix, &temp_matrix);
        }

        return QubitArray {
            num_qubts: num_qubits,
            density_matrix: density_matrix,
            larmor: larmor,
            dephasing_lifetime: dephasing_lifetime,
            guess_larmor: guess_larmor,
            simulation_times: None,
        };
    }
    /// Set the simulation times for the qubit array
    pub fn set_simulation_times(&mut self, simulation_times: Rc<SimulationTimes>) -> () {
        self.simulation_times = Some(Rc::clone(&simulation_times));
        self.larmor.set_simulation_times(simulation_times);
        return;
    }
    /// Get the density_matrix that represents the starting state for the qubits
    pub fn get_density_matrix(&self) -> &Array2<Complex64> {
        return &self.density_matrix;
    }
    /// Get the number of qubits (currently only 1)
    pub fn get_num_qubits(&self) -> u32 {
        return self.num_qubts;
    }
    /// Get the detuning Hamiltonian for the qubit array. Just a 2x2 array with the detuning value
    /// (guess - larmor) for each time step in the simulation times
    pub fn get_detuning_hamiltonian(&self, time_index: usize) -> Array2<Complex64> {
        // Detuning between guess and qubit. Factor of pi is to convert to angular frequency
        // combined with 1/2 factor from S_z gate
        let detuning: f64 =
            (self.larmor.get_larmor_frequency(time_index) - self.guess_larmor) * -PI;

        let detuning_hamiltonian: Array2<Complex64> =
            Array2::<Complex64>::from_shape_fn((2, 2), |(i, j)| {
                Complex64::new(
                    f64::from(-(i32::try_from(i + j).unwrap() - 1)) * detuning,
                    0.,
                )
            });
        return detuning_hamiltonian;
    }
    pub fn get_channel_coefficients(&mut self) -> Array2<f64> {
        return array![
            [
                1.,
                (-self
                    .simulation_times
                    .as_ref()
                    .ok_or(UninitializedTimesError)
                    .unwrap()
                    .get_dt()
                    * self.dephasing_lifetime)
                    .exp()
            ],
            [
                (-self
                    .simulation_times
                    .as_ref()
                    .ok_or(UninitializedTimesError)
                    .unwrap()
                    .get_dt()
                    * self.dephasing_lifetime)
                    .exp(),
                1.
            ]
        ];
    }
}
