import sys

sys.path.insert(1, ".")

__version__ = "1.0.0"

from .Pulses import (
    RampPulse,
    ConstantPulse,
    TangentPulse,
)

from .Gates import (
    QuantumGate,
    PiO2X,
    PiO2Y,
    IdleGate,
    SPhase,
    Hadamard
)

from .Simulation import (
    QuantumCircuit,
    Qubit,
    PulseSimulator,
    SimulationResult,
    SpinState
)

from .Experiments import (
    ramsey,
    rabi,
    gate_test,
)

from .Plotting import (
    plotBloch,
    plotQubitBloch
)
