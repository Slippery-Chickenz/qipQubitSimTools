import numpy as np
import numpy.typing as npt

from .base_spin_state import SpinState, SpinStateType

class Qubit:

    def __init__(self, larmor: float) -> None:
       
        # Current state of the system
        self.states: list[SpinState] = []

        # # Current time of the system
        # self.times: npt.NDArray[np.floating]
        
        # Current resonance of the system 
        # (energy difference between the two states, also known as the Larmor frequency)
        self.larmor: float = larmor

        return

    def getLarmor(self) -> float:
        return self.larmor

    # def getTimes(self) -> npt.NDArray[np.floating]:
    #     return self.times

    def initializeStateSamples(self, numSamples: int) -> None: #times: npt.NDArray[np.floating]) -> None:
        # self.times = times
        self.states = [SpinState("+Z") for _ in range(numSamples)]
        return

    def getState(self, i: int = -1) -> SpinState:
        return self.states[i]
    def setState(self, newState: SpinStateType, index: int) -> None:
        self.states[index].setState(newState)
        return
    
    def evolveState(self, evolutionOperator: npt.NDArray[np.complexfloating], index: int) -> None:
        self.states[index + 1] = SpinState(self.states[index].evolveState(evolutionOperator))
        return

    def getProb(self, index: int = -1, state: SpinStateType = "-Z") -> float:
        return self.states[index].getProbability(state = state)
    def getProbs(self, state: SpinStateType = "-Z") -> npt.NDArray[np.floating]:
        return np.array([qubitState.getProbability(state = state) for qubitState in self.states])









