import numpy as np

from .base_gate import QuantumGate

from ..Pulses.constant import ConstantPulse

class PiO2Y(QuantumGate):

    def __init__(self) -> None:
        super().__init__()
        self.appendPulse(ConstantPulse(np.pi / 2, 1, 0, -np.pi/2))
        return

