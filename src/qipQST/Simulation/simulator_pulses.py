from matplotlib.pyplot import waitforbuttonpress
import numpy as np
import numpy.typing as npt

from scipy.linalg import expm

from .base_qubit import Qubit
from .base_circuit import QuantumCircuit

class PulseSimulator:
    """
    Simulate a quantum circut constructed of raw ESR pulses
    """

    def __init__(self) -> None:

        # Qubit to simluate pusles on
        self.qubit: Qubit

        # Circuit made of of gates made of pulses to simulate
        self.quantumCircuit: QuantumCircuit

        # List of indicies where the states are recorded and the times at those indcies
        self.sampleIndices: npt.NDArray[np.integer]

        # Bool if the samples and interations have been set
        self.timeStepsSet: bool = False

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

    def setTimeSteps(self, numIterations: int, numSamples: int = 2) -> None:

        assert numSamples > 1, "At least two time points are needed"

        # Set the simualation time values
        self.qubit.times, self.sampleIndices = self.quantumCircuit.setSimulationTimes(numIterations, numSamples)

        self.quantumCircuit.calculateIntegratedFrequencies()

        # Reset the tls for this circuit
        self.qubit.states = np.zeros(shape=(numSamples, 2), dtype="complex")
        self.qubit.states[0] = np.array([1, 0])

        self.timeStepsSet = True
        return


    def simulateCircuit(self, numIterations: int = 500, numSamples: int = 2) -> Qubit:

        if not self.timeStepsSet:
            self.setTimeSteps(numIterations, numSamples)

        # Loop over all the sample times to get the evolution operator between each sample
        for i in range(len(self.sampleIndices) - 1):
            startIndex = self.sampleIndices[i]
            endIndex = self.sampleIndices[i + 1]
            evolutionOperator = self.getEvolutionOperator(startIndex, endIndex)
            self.qubit.states[i + 1] = self.qubit.states[i].dot(evolutionOperator)

        return self.qubit

    def getEvolutionOperator(self, startIndex: int, endIndex: int) -> npt.NDArray[np.complexfloating]:

        assert endIndex < len(self.quantumCircuit.integratedFrequency), "Set end index within the length of the circuit"

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
            evolutionOperator = evolutionOperator.dot(expm(np.pi * 1j * self.quantumCircuit.dt * hamiltonian))

        return evolutionOperator

    def getDetuningTerm(self) -> npt.NDArray[np.complexfloating]:

        # Diagonal term in the interaction frame. Splitting of the spin states based on detuning
        detuningTerm: npt.NDArray[np.complexfloating] = np.eye(2, dtype = "complex")
        detuning: float = self.quantumCircuit.getGuessLarmor() - self.qubit.larmor 
        detuningTerm *= detuning
        return detuningTerm

