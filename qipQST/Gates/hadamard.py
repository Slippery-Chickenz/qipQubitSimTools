import numpy as np
import numpy.typing as npt

from .base_gate import QuantumGate

class Hadamard(QuantumGate):

    def __init__(self) -> None:
        super().__init__()
        return

    @staticmethod
    def getIdealGate() -> npt.NDArray[np.complexfloating]:
        return np.array([[1, 1], [1, -1]]) / np.sqrt(2)

