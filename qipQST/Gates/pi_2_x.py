import numpy as np
import numpy.typing as npt

from .base_gate import QuantumGate

from ..Pulses.constant import ConstantPulse

class PiO2X(QuantumGate):

    def __init__(self) -> None:
        super().__init__()
        self.appendPulse(ConstantPulse(1/2, 1, 0, 0))
        return
    
    @classmethod
    def getIdealMatrix(cls) -> npt.NDArray[np.complexfloating]:
        return np.array([[np.cos(np.pi/4), -1j * np.sin(np.pi/4)], [-1j * np.sin(np.pi/4), np.cos(np.pi/4)]])

