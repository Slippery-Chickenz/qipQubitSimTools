import matplotlib.pyplot as plt

from .base_spin_state import SpinStateType, SpinState

from .base_qubit import Qubit

from ..Plotting.bloch_plot import plotQubitBloch

class SimulationResult:

    def __init__(self, simQubit: Qubit) -> None:

        # Qubit run through the sim
        self.qubit: Qubit = simQubit
        return

    def getProb(self, i: int = -1, state: SpinStateType = "-Z") -> float:
        return self.qubit.getProb(i, state)

    def getState(self, i: int = -1) -> SpinState:
        return self.qubit.getState(i)

    def plotProbability(self, state: SpinStateType = "+Z") -> None:

        fig, axes = plt.subplots(nrows=1, ncols=1, sharex=True)
        fig.set_figwidth(8)
        fig.set_figheight(3)

        axes.plot(self.qubit.times, self.qubit.getProbs(state))
        axes.set_ylim(-0.1, 1.1)
        plt.show()
        return

    def plotBloch(self, i: int | None = None) -> None:
        plotQubitBloch(self.qubit, i)
        return







