import numpy as np
import numpy.typing as npt

from scipy.linalg import expm

from .base_qubit import Qubit
from .base_qubit_array import QubitArray
from .base_circuit import QuantumCircuit
from .simulation_result import SimulationResult
from .base_spin_state import SpinStateType

from .._constants import *

class PulseSimulator:
    """
    Simulate a quantum circut constructed of raw ESR pulses
    """

    def __init__(self) -> None:

        # Qubit Array to simulate on
        self.qubitArray: QubitArray = QubitArray()

        # Circuit made of of gates made of pulses to simulate
        self.circuit: QuantumCircuit = QuantumCircuit(0)

        # List of indicies where the states are recorded and the times at those indcies
        self.sampleIndices: npt.NDArray[np.integer]

        # Bool if the samples and interations have been set
        self.timeStepsSet: bool = False

        return

    def setCircuit(self, newCircuit: QuantumCircuit) -> None:
        """
        Set the circuit to simulate
        """
        self.circuit = newCircuit
        return

    def setTimeSteps(self, numIterations: int, numSamples: int) -> None:

        if numSamples <= 1:
            raise ValueError("At least two time points are needed")

        # Set the simualation time values
        sampleTimes, self.sampleIndices = (self.circuit
                                           .setSimulationTimes(numIterations, numSamples))

        self.circuit.calculateIntegratedFrequencies()

        # Reset the qubit state for this circuit
        self.qubitArray.initializeStateSamples(sampleTimes)
        return

    def prepareSimulation(self, numIterations: int, numSamples: int) -> None:

        if not self.circuit.simulationTimesAreSet():
            self.qubitArray.clearQubits()
            self.setTimeSteps(numIterations, numSamples)

        # Check that the qubit array has the correct number of qubits
        if self.circuit.getNumQubits() != self.qubitArray.getNumQubits():
            self.qubitArray.clearQubits()
            for _ in range(self.circuit.getNumQubits()):
                self.qubitArray.addQubit(Qubit(0))
        return

    def simulateCircuit(
        self, 
        numIterations: int = 500, 
        numSamples: int = 2,
        sampleAfterGate: bool = False,
        startingState: SpinStateType = "+Z",
    ) -> SimulationResult:

        if self.circuit.getNumGates() == 0:
            raise ValueError("Current set circuit has no gates to be run")

        # for i in range(self.circuit.getNumQubits() - self.qubitArray.getNumQubits()):
        #     self.qubitArray.addQubit(Qubit(0))

        if sampleAfterGate:
            numSamples = self.circuit.getNumGates() + 1

        self.prepareSimulation(numIterations, numSamples)

        # if not self.timeStepsSet:
        #     self.setTimeSteps(numIterations, numSamples)

        # Set the starting state
        self.qubitArray.setStates(startingState, 0)
        
        # Loop over all the sample times to get the evolution operator between each sample
        for i in range(len(self.sampleIndices) - 1):
            startIndex = self.sampleIndices[i]
            endIndex = self.sampleIndices[i + 1]
            evolutionOperator = self.getEvolutionOperator(startIndex, endIndex)

            self.qubitArray.evolveState(evolutionOperator, i)

        return SimulationResult(self.qubitArray)

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
        detuning: float = self.qubitArray.getLarmor(0) - self.circuit.getGuessLarmor()
        detuningTerm *= -detuning * 2 * np.pi
        return detuningTerm

    def getDetuning(self) -> float:
        return self.qubitArray.getLarmor(0)- self.circuit.getGuessLarmor()

