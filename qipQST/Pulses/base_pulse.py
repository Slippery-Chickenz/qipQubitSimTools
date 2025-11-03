import numpy as np
import numpy.typing as npt

import matplotlib.pyplot as plt

from .._constants import sX, sY

class Pulse:

    def __init__(self, time: float) -> None:

        # Total time the pulse takes
        self.time: float = time
        return

    def getAmplitude(self, t: float) -> float:
        raise NotImplementedError("getAmplitude is not implemented")

    def getFrequency(self, t: float) -> float:
        raise NotImplementedError("getFrequency is not implemented")

    def getPhase(self, t: float) -> float:
        raise NotImplementedError("getPhase is not implemented")

    def getTime(self) -> float:
        return self.time

    def getIntegratedFrequencies(
        self, 
        times: npt.NDArray[np.floating]
    ) -> npt.NDArray[np.floating]:

        # Raw frequencies at each time step
        rawFrequencies = np.array([self.getFrequency(t) for t in times])

        # Integrate the raw frequencies to get the frequency modulated values
        integratedFrequency = np.cumsum(rawFrequencies) * (times[1] - times[0])
        return integratedFrequency

    @staticmethod
    def hamiltonian(amplitude: float, frequency: float, phase:float) -> float:
        """
        Hamiltonian for the pulse in the rotating frame.
        """
        return (amplitude * 
               (np.cos(frequency + phase) * sX - 
                np.sin(frequency + phase) * sY))

    def plotPulse(self) -> None:

        # Time values to plot over
        plotTimes = np.linspace(0, self.time, 500)

        # Amplitude, frequency, and pulse values to plot
        amplitudes = [self.getAmplitude(t) for t in plotTimes]
        integratedFrequency = self.getIntegratedFrequencies(plotTimes)
        frequencies = [self.getFrequency(t) for t in plotTimes]
        pulseValues = [self.getAmplitude(t) * np.cos(integratedFrequency[i] + self.getPhase((t))) for i, t in enumerate(plotTimes)]

        # Create the fig/axes and set the size
        fig, axes = plt.subplots(nrows=3, ncols=1, layout="tight", sharex=True)
        fig.set_figheight(8)
        fig.set_figwidth(6)
        fig.supxlabel("Time")

        axes[0].plot(plotTimes, amplitudes)
        axes[0].set_ylabel("Amplitude")

        axes[1].plot(plotTimes, frequencies)
        axes[1].set_ylabel("Frequency")

        axes[2].plot(plotTimes, pulseValues)
        axes[2].set_ylabel("Pulse Voltage")
        
        plt.show()
        return
