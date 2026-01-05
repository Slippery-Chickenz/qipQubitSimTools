from typing import override
from .base_pulse import Pulse

class ConstantPulse(Pulse):
    
    def __init__(self, time: float, amplitude: float, frequency: float, phase: float) -> None:

        # Set the time of the pulse in the base class
        super().__init__(time)

        # Set the amplitude, frequency and phase
        self.amplitude: float = amplitude
        self.frequency: float = frequency
        self.phase: float = phase
        return

    @override
    def getAmplitude(self, t: float) -> float:
        return self.amplitude

    @override
    def getFrequency(self, t: float) -> float:
        return self.frequency

    @override
    def getPhase(self, t: float) -> float:
        return self.phase
