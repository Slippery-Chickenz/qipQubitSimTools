pub mod constant;
pub mod ramp;
pub mod tangent;

pub use constant::Constant;
pub use ramp::Ramp;
pub use tangent::Tangent;

/// Trait to be implemented to be used as a pulse that makes up a gate
pub trait Pulse {
    /// Get the amplitude of the pulse at some time
    fn get_amplitude(&self, t: f64) -> f64;
    /// Get the frequency of the pulse at some time
    fn get_frequency(&self, t: f64) -> f64;
    /// Get the phase of the pulse at some time
    fn get_phase(&self, t: f64) -> f64;
    /// Get the total duration of the pulse
    fn get_duration(&self) -> f64;
}
