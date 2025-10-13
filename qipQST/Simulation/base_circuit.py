import numpy as np
import numpy.typing as npt

import matplotlib.pyplot as plt

from ..Gates.base_gate import QuantumGate

from .._constants import *

class QuantumCircuit:
    """
    A set of quantum gates to run in a set sequence.
    """

    def __init__(self, guessLarmor: float) -> None:

        # List of quantum gates to perform in order
        self.gates: list[QuantumGate] = []

        # Guess Larmor Frequency
        self.guessLarmor: float = guessLarmor

        # Integrated frequency values over the entire circuit for the frequency modulation
        self.integratedFrequency: npt.NDArray[np.floating] = np.array([])

        # Times for the iteration over the entire circuit
        self.iterationTimes: npt.NDArray[np.floating] = np.array([])
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

        hamiltonian += (-amplitude * 
                       (np.cos(2 * np.pi * frequency + phase) * sX + 
                        np.sin(2 * np.pi * frequency + phase) * sY))
        return hamiltonian

    def setSimulationTimes(
        self, 
        numIterations: int, 
        numSamples: int
    ) -> tuple[npt.NDArray[np.floating], npt.NDArray[np.integer]]:

        # Add one because the number of iterations is the number of steps not the number of times
        numIterations += 1

        # Time stamps where each sample will be taken
        sampleTimes = np.linspace(0, self.getTime(), numSamples)
        sampleIndices = np.zeros(numSamples, dtype=np.int32)

        self.iterationTimes = np.linspace(0, sampleTimes[1], numIterations)
        self.dt = self.iterationTimes[1]

        for i in range(numSamples - 2):
            nextTimes = np.linspace(sampleTimes[i + 1], sampleTimes[i + 2], numIterations)
            sampleIndices[i + 1] = len(self.iterationTimes) - 1
            self.iterationTimes = np.concat((self.iterationTimes, nextTimes[1:]))
            
        sampleIndices[-1] = len(self.iterationTimes) - 1
        return sampleTimes, sampleIndices

    def calculateIntegratedFrequencies(self) -> None:

        # Raw frequencies at each time step
        rawFrequencies = np.array([self.getGate(t).getFrequency(t) - self.guessLarmor for t in self.iterationTimes])

        # Integrate the raw frequencies to get the frequency modulated values
        self.integratedFrequency = np.cumsum(rawFrequencies) * self.dt
        return

    def appendGate(self, newGate: QuantumGate) -> None:
        self.gates.append(newGate)
        return
    def getGate(self, t: float) -> QuantumGate:
        for gate in self.gates:
            if t < gate.getTime():
                return gate
            t -= gate.getTime()
        return self.gates[-1]

    def getTime(self) -> float:
        time = 0
        for gate in self.gates:
            time += gate.getTime()
        return time

    def plotCircuitWaveform(self) -> None:

        if len(self.iterationTimes) == 0:
            self.setSimulationTimes(500, len(self.gates) + 1)
            self.calculateIntegratedFrequencies()

        # Amplitude, frequency, and pulse values to plot
        amplitudes = [self.getGate(t).getAmplitude(t) for t in self.iterationTimes]
        frequencies = [self.getGate(t).getFrequency(t) for t in self.iterationTimes]
        pulseValues = [self.getHamiltonian(i)[0][1].real for i in range(len(self.iterationTimes))]

        # Create the fig/axes and set the size
        fig, axes = plt.subplots(nrows=3, ncols=1, layout="tight", sharex=True)
        fig.set_figheight(6)
        fig.set_figwidth(6)
        fig.supxlabel("Time")

        axes[0].plot(self.iterationTimes, amplitudes)
        axes[0].set_ylabel("Amplitude")

        axes[1].plot(self.iterationTimes, frequencies)
        axes[1].set_ylabel("Frequency")

        axes[2].plot(self.iterationTimes, pulseValues)
        axes[2].set_ylabel("Pulse Voltage")
        
        plt.show()
        return









