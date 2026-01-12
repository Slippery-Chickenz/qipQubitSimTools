import numpy as np
import numpy.typing as npt

from .base_qubit import Qubit
from .base_spin_state import SpinStateType

class QubitArray:

    def __init__(self) -> None:

        # List of qubits to manage
        self.qubits: list[Qubit] = []

        # Times at which each qubit has a state sample saved
        self.sampleTimes: npt.NDArray[np.floating]
        return

    def addQubit(self, newQubit: Qubit) -> None:
        self.qubits.append(newQubit)
        self.qubits[-1].initializeStateSamples(len(self.sampleTimes))
        return
    def clearQubits(self) -> None:
        self.qubits = []

    def getQubit(self, qubitNum: int) -> Qubit:
        return self.qubits[qubitNum]

    def getNumQubits(self) -> int:
        return len(self.qubits)

    def getLarmor(self, qubitNum: int) -> float:
        return self.qubits[qubitNum].getLarmor()

    def initializeStateSamples(self, numSamples: npt.NDArray[np.floating]) -> None:
        self.sampleTimes = numSamples
        # for q in self.qubits:
        #     q.initializeStateSamples(numSamples)
        return
    def getTimes(self) -> npt.NDArray[np.floating]:
        return self.sampleTimes

    def setStates(self, newState: SpinStateType, index: int) -> None:
        for q in self.qubits:
            q.setState(newState, index)
        return

    def evolveState(self, evolutionOperator: npt.NDArray[np.complexfloating], index: int) -> None:
        for q in self.qubits:
            q.evolveState(evolutionOperator, index)
        return

