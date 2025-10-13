import numpy as np
import numpy.typing as npt

from .base_gate import QuantumGate

class SPhase(QuantumGate):

    def __init__(self) -> None:
        super().__init__()
        return

    @staticmethod
    def getIdealGate() -> npt.NDArray[np.complexfloating]:
        return np.array([[1, 0], [0, 1j]])

