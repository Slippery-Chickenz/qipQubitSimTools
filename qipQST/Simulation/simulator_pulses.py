import numpy as np
import numpy.typing as npt

from scipy.linalg import expm

from .base_qubit import Qubit
from .base_circuit import QuantumCircuit
from .simulation_result import SimulationResult
from .base_spin_state import SpinState, SpinStateType

from .._constants import *

class PulseSimulator:
    """
    Simulate a quantum circut constructed of raw ESR pulses
    """

    def __init__(self) -> None:

        # Qubit to simluate pusles on
        # self.circuit.qubit: Qubit

        # Circuit made of of gates made of pulses to simulate
        self.circuit: QuantumCircuit

        # List of indicies where the states are recorded and the times at those indcies
        self.sampleIndices: npt.NDArray[np.integer]

        # Bool if the samples and interations have been set
        self.timeStepsSet: bool = False

        return

    # def setQubit(self, newQubit: Qubit) -> None:
    #     # self.circuit.qubit = newQubit
    #     return;

    def setCircuit(self, newCircuit: QuantumCircuit) -> None:
        """
        Set the circuit to simulate
        """
        self.circuit = newCircuit
        self.timeStepsSet = False
        return

    def setTimeSteps(self, numIterations: int, numSamples: int = 2) -> None:

        assert numSamples > 1, "At least two time points are needed"

        # Set the simualation time values
        sampleTimes, self.sampleIndices = (self.circuit
                                           .setSimulationTimes(numIterations, numSamples))

        self.circuit.calculateIntegratedFrequencies()

        # Reset the qubit state for this circuit
        self.circuit.qubit.initializeStates(sampleTimes)
        self.timeStepsSet = True
        return

    def simulateCircuit(
        self, 
        numIterations: int = 500, 
        numSamples: int = 2,
        sampleAfterGate: bool = False,
        startingState: SpinStateType = "+Z",
    ) -> SimulationResult:

        if sampleAfterGate:
            numSamples = self.circuit.getNumGates() + 1

        if not self.timeStepsSet:
            self.setTimeSteps(numIterations, numSamples)

        # Set the starting state
        self.circuit.qubit.states[0].setState(startingState)
        
        # Loop over all the sample times to get the evolution operator between each sample
        for i in range(len(self.sampleIndices) - 1):
            startIndex = self.sampleIndices[i]
            endIndex = self.sampleIndices[i + 1]
            evolutionOperator = self.getEvolutionOperator(startIndex, endIndex)

            self.circuit.qubit.states[i + 1] = SpinState(evolutionOperator @ 
                                                 self.circuit.qubit.states[i].state)

        return SimulationResult(self.circuit.qubit)

    def getEvolutionOperator(
        self, 
        startIndex: int, 
        endIndex: int
    ) -> npt.NDArray[np.complexfloating]:

        if endIndex >= len(self.circuit.integratedFrequency):
            raise ValueError("Set end index within the length of the circuit")

        # Evolution operator, Hamiltonian, and detuning term
        evolutionOperator: npt.NDArray[np.complexfloating]
        hamiltonian: npt.NDArray[np.complexfloating]
        detuningTerm: npt.NDArray[np.complexfloating]

        # Evolution operator for the given circuit
        evolutionOperator = np.eye(2, dtype = "complex")

        # Diagonal term in the interaction frame. 
        # Splitting of the spin states based on detuning
        detuningTerm = self.getDetuningTerm()

        # Loop over the entire time of the circuit
        for i in range(startIndex, endIndex):
            hamiltonian = self.circuit.getHamiltonian(i) + detuningTerm
            evolutionOperator = (evolutionOperator.dot(expm(-1j 
                                 *self.circuit.dt 
                                 *hamiltonian)))

        return evolutionOperator

    def getDetuningTerm(self) -> npt.NDArray[np.complexfloating]:

        # Diagonal term in the interaction frame
        # Splitting of the spin states based on detuning
        detuningTerm: npt.NDArray[np.complexfloating] = SZ.copy()
        detuning: float = self.circuit.qubit.getLarmor() - self.circuit.getGuessLarmor()
        detuningTerm *= -detuning * 2 * np.pi
        return detuningTerm

    def getDetuning(self) -> float:
        return self.circuit.qubit.larmor - self.circuit.getGuessLarmor()

