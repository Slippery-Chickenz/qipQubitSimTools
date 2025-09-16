import numpy as np
import numpy.typing as npt

from QuantumGate import QuantumGate

from constants import *

class QuantumCircuit:
    """
    Defines a set of quantum gates to run in a set sequence.
    """

    def __init__(self) -> None:

        # List of quantum gates to perform in order
        self.quantumGates: list[QuantumGate] = []

        # Guess Larmor Frequency
        self.guessLarmor: float

        # Integrated frequency values over the entire circuit for the frequency modulation
        self.integratedFrequency: npt.NDArray[np.floating] = np.array([])

        # Times for the iteration over the entire circuit
        self.iterationTimes: npt.NDArray[np.floating]
        self.dt: float # Time between iterations

        return

    def getGuessLarmor(self) -> float:
        return self.guessLarmor

    def getHamiltonian(self, timeIndex: int) -> npt.NDArray[np.complexfloating]:

        t = self.iterationTimes[timeIndex]

        hamiltonian: npt.NDArray[np.complexfloating] = np.zeros((2, 2), dtype="complex")

        currentGate = self.getGate(t)

        amplitude = currentGate.getAmplitude(t)
        frequency = self.integratedFrequency[timeIndex]
        phase = currentGate.getPhase(t)

        hamiltonian += amplitude * (np.cos(frequency + phase) * sX + np.sin(frequency + phase) * sY)

        return np.array([])

    def calculateIntegratedFrequencies(self) -> None:

        # Raw frequencies at each time step
        rawFrequencies = np.array([self.getGate(t).getFrequency(t) for t in self.iterationTimes])

        # Integrate the raw frequencies to get the frequency modulated values
        self.integratedFrequency = np.cumsum(rawFrequencies) * self.dt
        return

    def appendGate(self, newGate: QuantumGate) -> None:
        self.quantumGates.append(newGate)
        return
    def getGate(self, t: float) -> QuantumGate:
        for gate in self.quantumGates:
            if t < gate.getTime():
                return gate
            t -= gate.getTime()
        return self.quantumGates[-1]

    def getTime(self) -> float:
        time = 0
        for gate in self.quantumGates:
            time += gate.getTime()
        return time









