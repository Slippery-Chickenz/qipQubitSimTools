import numpy as np
import numpy.typing as npt

from .base_gate import QuantumGate

from ..Pulses.constant import ConstantPulse

class Hadamard(QuantumGate):

    def __init__(self) -> None:
        super().__init__()
        self.appendPulse(ConstantPulse(1, 1, 0, 0))
        self.appendPulse(ConstantPulse(1/2, 1, 0, -np.pi / 2))
        return

    @classmethod
    def getIdealMatrix(cls) -> npt.NDArray[np.complexfloating]:
        return np.array([[1, 1], [1, -1]]) / np.sqrt(2)

