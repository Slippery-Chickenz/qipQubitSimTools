pub mod atm_gate;
pub mod constant;
pub mod idle;
pub mod pi;
pub mod pi_o_2_x;
pub mod pi_o_2_y;
pub mod pi_x;
pub mod pi_y;

pub use atm_gate::ATMGate;
pub use constant::Constant;
pub use idle::Idle;
pub use pi_o_2_x::PiO2X;
pub use pi_o_2_y::PiO2Y;
pub use pi_x::PiX;
pub use pi_y::PiY;

/// Trait to guarentee that a gate can be converted into a string
pub trait CheckGateName {
    fn check_name(name: &str) -> bool;
}

/// Trait that must be implemented for any gate that is to be used within a quantum circuit.
/// Contains functions to get the amplitude, frequency, and phase of the gate at any time within
/// the duration of the gate. And one to get the duration of the gate.
pub trait Gate {
    fn get_amplitude(&self, t: f64) -> f64;
    fn get_frequency(&self, t: f64) -> f64;
    fn get_phase(&self, t: f64) -> f64;
    fn get_duration(&self) -> f64;
}

/// Macro to make the check_name function and implement the CheckGateName trait for an arbitrary
/// struct. By default it just makes the string name the name of the struct
#[macro_export]
macro_rules! default_name {
    ($n:ident) => {
        impl CheckGateName for $n {
            fn check_name(name: &str) -> bool {
                return name == stringify!($n);
            }
        }
    };
}
