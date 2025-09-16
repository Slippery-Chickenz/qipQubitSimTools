import numpy as np
import numpy.typing as npt

from scipy.linalg import expm

from .Qubit import Qubit
from .QuantumCircuit import QuantumCircuit

class PulseSimulator:
    """
    Simulate a quantum circut constructed of raw ESR pulses
    """

    def __init__(self) -> None:

        # Qubit to simluate pusles on
        self.qubit: Qubit

        # Circuit made of of gates made of pulses to simulate
        self.quantumCircuit: QuantumCircuit = QuantumCircuit()

        # List of indicies where the states are recorded and the times at those indcies
        self.sampleIndices: npt.NDArray[np.integer]

        return

    def setQubit(self, newQubit: Qubit) -> None:
        self.qubit = newQubit
        return;

    def setCircuit(self, newCircuit: QuantumCircuit) -> None:
        """
        Set the circuit to simulate
        """
        self.quantumCircuit = newCircuit
        return

    def setSimulationTimes(self, numIterations: int, numSamples: int) -> None:

        # Add one because the number of iterations is the number of steps not the number of times
        numIterations += 1

        # Time stamps where each sample will be taken
        self.qubit.times = np.linspace(0, self.quantumCircuit.getTime(), numSamples)

        self.sampleIndices = np.zeros(numSamples, dtype=np.int32)

        self.quantumCircuit.iterationTimes = np.linspace(0, self.qubit.times[1], numIterations)
        self.quantumCircuit.dt = self.quantumCircuit.iterationTimes[1]

        for i in range(numSamples - 2):
            nextTimes: npt.NDArray[np.floating] = np.linspace(self.qubit.times[i + 1], self.qubit.times[i + 2], numIterations)
            self.sampleIndices[i + 1] = len(self.quantumCircuit.iterationTimes) - 1
            self.quantumCircuit.iterationTimes = np.concat((self.quantumCircuit.iterationTimes, nextTimes[1:]))
            
        self.sampleIndices[-1] = len(self.quantumCircuit.iterationTimes) - 1
        return

    def simulateCircuit(self, numIterations: int, numSamples: int = 2) -> Qubit:

        assert numSamples > 1, "At least two time points are needed"

        # Set the simualation time values
        self.setSimulationTimes(numIterations, numSamples)

        # Reset the tls for this circuit
        self.qubit.states = np.zeros(shape=(numSamples, 2), dtype="complex")
        self.qubit.states[0] = np.array([1, 0])

        # Loop over all the sample times to get the evolution operator between each sample
        for i in range(len(self.sampleIndices) - 1):
            startIndex = self.sampleIndices[i]
            endIndex = self.sampleIndices[i + 1]
            evolutionOperator = self.getEvolutionOperator(startIndex, endIndex)
            self.qubit.states[i + 1] = self.qubit.states[i].dot(evolutionOperator)

        return self.qubit

    def getEvolutionOperator(self, startIndex: int, endIndex: int) -> npt.NDArray[np.complexfloating]:

        # Evolution operator, Hamiltonian, and detuning term
        evolutionOperator: npt.NDArray[np.complexfloating]
        hamiltonian: npt.NDArray[np.complexfloating]
        detuningTerm: npt.NDArray[np.complexfloating]

        # Evolution operator for the given circuit
        evolutionOperator = np.eye(2, dtype = "complex")

        # Diagonal term in the interaction frame. Splitting of the spin states based on detuning
        detuningTerm = self.getDetuningTerm()

        # Loop over the entire time of the circuit
        for i in range(startIndex, endIndex):
            hamiltonian = self.quantumCircuit.getHamiltonian(i) + detuningTerm
            evolutionOperator = evolutionOperator.dot(expm(1j * self.quantumCircuit.dt * hamiltonian))

        return evolutionOperator

    def getDetuningTerm(self) -> npt.NDArray[np.complexfloating]:

        # Diagonal term in the interaction frame. Splitting of the spin states based on detuning
        detuningTerm: npt.NDArray[np.complexfloating] = np.eye(2, dtype = "complex")
        detuning: float = self.qubit.larmor - self.quantumCircuit.getGuessLarmor()
        detuningTerm *= detuning
        return detuningTerm

