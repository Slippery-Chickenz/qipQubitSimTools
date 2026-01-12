import numpy as np
import numpy.typing as npt
import matplotlib.pyplot as plt

from .base_spin_state import SpinStateType, SpinState

# from .base_qubit import Qubit
from .base_qubit_array import QubitArray

from ..Plotting.bloch_plot import plotQubitBloch

class SimulationResult:

    def __init__(self, qubitArray: QubitArray) -> None:

        # Qubits run through the sim
        self.qubitArray: QubitArray = qubitArray
        # self.qubit: Qubit = simQubit
        return

    def getProb(self, i: int = -1, state: SpinStateType = "-Z", qubitNum: int = 0) -> float:
        return self.qubitArray.getQubit(qubitNum).getProb(i, state)
    def getProbs(self, state: SpinStateType = "-Z", qubitNum: int = 0) -> npt.NDArray[np.floating]:
        return self.qubitArray.getQubit(qubitNum).getProbs(state)

    def getTimes(self) -> npt.NDArray[np.floating]:
        return self.qubitArray.getTimes()

    def getState(self, i: int = -1, qubitNum: int = 0) -> SpinState:
        return self.qubitArray.getQubit(qubitNum).getState(i)

    def plotProbability(self, state: SpinStateType = "+Z") -> None:

        fig, axes = plt.subplots(nrows=1, ncols=1, sharex=True)
        fig.set_figwidth(8)
        fig.set_figheight(3)

        axes.plot(self.qubitArray.getTimes(), self.qubitArray.getQubit(0).getProbs(state))
        axes.set_ylim(-0.1, 1.1)
        plt.show()
        return

    def plotBloch(self, i: int | None = None) -> None:
        plotQubitBloch(self.qubitArray.getQubit(0), i)
        return







