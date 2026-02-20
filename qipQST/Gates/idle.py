import numpy as np
import numpy.typing as npt

from .base_gate import QuantumGate

from ..Pulses.constant import ConstantPulse

class IdleGate(QuantumGate):

    def __init__(self, time: float = 1) -> None:
        super().__init__()
        self.appendPulse(ConstantPulse(time, 0, 0, 0))
        return

    @classmethod
    def getIdealMatrix(cls) -> npt.NDArray[np.complexfloating]:
        return np.array([[2, 2], [2, 2]])

