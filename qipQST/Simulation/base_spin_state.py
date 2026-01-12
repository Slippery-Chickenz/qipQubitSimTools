import numpy as np
import numpy.typing as npt

from ..Gates import Hadamard, SPhase

from typing import Literal, TypeAlias

upXState: npt.NDArray[np.complexfloating] = np.array([1, 1]) / np.sqrt(2)
downXState: npt.NDArray[np.complexfloating] = np.array([1, -1]) / np.sqrt(2)
upYState: npt.NDArray[np.complexfloating] = np.array([1, 1j]) / np.sqrt(2)
downYState: npt.NDArray[np.complexfloating] = np.array([1, -1j]) / np.sqrt(2)
upZState: npt.NDArray[np.complexfloating] = np.array([1, 0])
downZState: npt.NDArray[np.complexfloating] = np.array([0, 1])

StateStr =  Literal["+X", "-X", "+Y", "-Y", "+Z", "-Z"] 
BasisStr =  Literal["X", "Y", "Z"] 

SpinStateType: TypeAlias = npt.NDArray[np.complexfloating] | StateStr | "SpinState"

def getBasisState(stateStr: StateStr = "+Z") -> npt.NDArray[np.complexfloating]:
    match stateStr:
        case "+X":
            return upXState
        case "-X":
            return downXState
        case "+Y":
            return upYState
        case "-Y":
            return downYState
        case "+Z":
            return upZState
        case "-Z":
            return downZState

def getStateArray(state: SpinStateType) -> npt.NDArray[np.complexfloating]:
    if isinstance(state, str):
        return getBasisState(state)
    elif isinstance(state, SpinState):
        return state.state
    return state

class SpinState:

    def __init__(
        self, 
        state: SpinStateType = "+Z"
    ) -> None:

        # Spin state held
        self.state: npt.NDArray[np.complexfloating] = getStateArray(state)
        return

    def getState(self, basis: BasisStr = "Z") -> npt.NDArray[np.complexfloating]:
        if basis == "Z":
            return self.state
        if basis == "X":
            return Hadamard.getIdealMatrix().dot(self.state)
        return Hadamard.getIdealMatrix().dot(SPhase.getIdealGate().dot(self.state))
    
    def setState(self, newState: SpinStateType) -> None:
        self.state = getStateArray(newState)
        return
    
    def evolveState(self, evolutionOperator: npt.NDArray[np.complexfloating]) -> npt.NDArray[np.complexfloating]:
        return evolutionOperator @ self.state

    def getProbability(
        self, 
        state: SpinStateType = "+Z"
    ) -> float:

        state = getStateArray(state)
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


