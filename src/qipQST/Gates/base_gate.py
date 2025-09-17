import numpy as np
import numpy.typing as npt
import matplotlib.pyplot as plt

from ..Pulses.base_pulse import Pulse

class QuantumGate:
    """
    Defines a set of pulse points which together create a waveform that the gate applies
    """

    def __init__(self) -> None:

        # List of pulses that constitute this gate
        self.pulses: list[Pulse] = []
        return

    def getAmplitude(self, t: float) -> float:
        pulse, pulseTime = self.getPulse(t)
        return pulse.getAmplitude(pulseTime)
    def getFrequency(self, t: float) -> float:
        pulse, pulseTime = self.getPulse(t)
        return pulse.getFrequency(pulseTime)
    def getPhase(self, t: float) -> float:
        pulse, pulseTime = self.getPulse(t)
        return pulse.getPhase(pulseTime)

    def appendPulse(self, newPulse: Pulse) -> None:
        self.pulses.append(newPulse)
    def getPulse(self, t) -> tuple[Pulse, float]:
        for pulse in self.pulses:
            if t < pulse.getTime():
                return pulse, t
            t -= pulse.getTime()
        return self.pulses[-1], self.pulses[-1].getTime()

    def getTime(self) -> float:
        t = 0
        for pulse in self.pulses:
            t += pulse.getTime()
        return t

    def getIntegratedFrequencies(self, times: npt.NDArray[np.floating]) -> npt.NDArray[np.floating]:

        # Raw frequencies at each time step
        rawFrequencies = np.array([self.getFrequency(t) for t in times])

        # Integrate the raw frequencies to get the frequency modulated values
        integratedFrequency = np.cumsum(rawFrequencies) * (times[1] - times[0])
        return integratedFrequency

    def plotPulses(self) -> None:

        # Time values to plot over
        plotTimes = np.linspace(0, self.getTime(), 500 * len(self.pulses))

        # Amplitude, frequency, and pulse values to plot
        amplitudes = [self.getAmplitude(t) for t in plotTimes]
        integratedFrequency = self.getIntegratedFrequencies(plotTimes)
        frequencies = [self.getFrequency(t) for t in plotTimes]
        pulseValues = [self.getAmplitude(t) * np.cos(integratedFrequency[i] + self.getPhase((t))) for i, t in enumerate(plotTimes)]

        # Create the fig/axes and set the size
        fig, axes = plt.subplots(nrows=3, ncols=1, layout="tight", sharex=True)
        fig.set_figheight(12)
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
