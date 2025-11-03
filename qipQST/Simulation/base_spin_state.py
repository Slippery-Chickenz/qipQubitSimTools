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

    def getSphericalState(self) -> tuple[float, float]:

        # Alpha and Beta for the state a|0> + b|1>
        a = self.state[0]
        b = self.state[1]

        # Magnitude of the values
        magA = np.sqrt((a * np.conj(a)).real)
        magB = np.sqrt((b * np.conj(b)).real)

        phi = 2 * np.arccos(magA)

        theta = 0
        if (magA > 0.0001 and magB > 0.0001):
            theta = np.log((b * np.conj(a)) / (magA * magB)) / 1j

        return np.real(theta), np.real(phi)


