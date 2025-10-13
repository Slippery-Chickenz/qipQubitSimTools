import numpy as np
import numpy.typing as npt

from ..states import StateStr, BasisStr, getBasisState 

from ..Gates import Hadamard, SPhase

class SpinState:

    def __init__(
        self, 
        state: npt.NDArray[np.complexfloating] | StateStr = "+Z"
    ) -> None:

        if isinstance(state, str):
            state = getBasisState(state)
        # Spin state held
        self.state: npt.NDArray[np.complexfloating] = state
        return

    def getState(self, basis: BasisStr = "Z"):
        if basis == "Z":
            return self.state
        if basis == "X":
            return Hadamard.getIdealGate().dot(self.state)
        return Hadamard.getIdealGate().dot(SPhase.getIdealGate().dot(self.state))

    def getProbability(
        self, 
        state: npt.NDArray[np.complexfloating] | StateStr = "+Z"
    ) -> float:

        if isinstance(state, str):
            state = getBasisState(state)

        braket = np.dot(np.conj(state), self.state)
        return np.real(np.dot(np.conj(braket), braket))

