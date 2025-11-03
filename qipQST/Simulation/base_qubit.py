import numpy as np
import numpy.typing as npt

from ..states import StateStr

from .base_spin_state import SpinState

class Qubit:

    def __init__(self, larmor: float) -> None:
       
        # Current state of the system
        self.states: list[SpinState]

        # Current time of the system
        self.times: npt.NDArray[np.floating]
        
        # Current resonance of the system 
        # (energy difference between the two states, also known as the Larmor frequency)
        self.larmor: float = larmor

        return

    def initializeStates(self, times: npt.NDArray[np.floating]) -> None:

        self.times = times
        self.states = [SpinState("+Z") for _ in range(len(self.times))]
        return

    def getProb(self, index: int = -1, state: npt.NDArray[np.complexfloating] | StateStr = "-Z") -> float:
        return self.states[index].getProbability(state = state)
    def getProbs(self, state: npt.NDArray[np.complexfloating] | StateStr = "-Z") -> npt.NDArray[np.floating]:
        return np.array([qubitState.getProbability(state) for qubitState in self.states])









