import numpy as np
import numpy.typing as npt

import matplotlib.pyplot as plt

from ..states import StateStr

from .base_qubit import Qubit
from .base_spin_state import SpinState

class SimulationResult:

    def __init__(self, simQubit: Qubit) -> None:

        # Qubit run through the sim
        self.qubit: Qubit = simQubit

        return

    def plotProbability(self, state: npt.NDArray[np.complexfloating] | StateStr = "+Z") -> None:


        fig, axes = plt.subplots(nrows=1, ncols=1, sharex=True)
        fig.set_figwidth(8)
        fig.set_figheight(3)

        axes.plot(self.qubit.times, self.qubit.getProbs(state))
        axes.set_ylim(-0.1, 1.1)
        plt.show()
        return

    def plotBloch(self, i: int | None = None) -> None:

        #
        fig = plt.figure()
        ax = fig.add_subplot(1,1,1, projection='3d')
        ax.set_axis_off()

        # Make data
        u = np.linspace(0, 2 * np.pi, 100)
        v = np.linspace(0, np.pi, 100)
        x = np.outer(np.cos(u), np.sin(v))
        y = np.outer(np.sin(u), np.sin(v))
        z = np.outer(np.ones(np.size(u)), np.cos(v))

        # Plot a basic wireframe.
        ax.plot_wireframe(x, y, z, rstride=10, cstride=10, color='grey', alpha = 0.15)

        # Plot the surface
        ax.plot_surface(x, y, z, alpha=0.05, color = 'grey')

        # Prepare arrays x, y, z
        wireAngles = np.linspace(0, 2 * np.pi, 100)

        x = np.cos(wireAngles)
        y = np.sin(wireAngles)
        z = np.zeros(np.shape(x))
        ax.plot(x, y, z, color = 'dimgray', alpha = 0.5)

        x = np.cos(wireAngles)
        y = np.zeros(np.shape(x))
        z = np.sin(wireAngles)
        ax.plot(x, y, z, color = 'dimgray', alpha = 0.3)

        x = np.zeros(np.shape(y))
        y = np.cos(wireAngles)
        z = np.sin(wireAngles)
        ax.plot(x, y, z, color = 'dimgray', alpha = 0.5)


        ax.plot([-1, 1], [0, 0], [0, 0], color = 'dimgray', alpha = 0.4)
        ax.plot([0, 0], [-1, 1], [0, 0], color = 'dimgray', alpha = 0.4)
        ax.plot([0, 0], [0, 0], [-1, 1], color = 'dimgray', alpha = 0.4)

        theta, phi = 0, 0

        if i is not None:
            theta, phi = self.getSphericalState(self.qubit.states[i])
        else:
            sphereStates = [self.getSphericalState(state) for state in self.qubit.states]
            theta = [sphereState[0] for sphereState in sphereStates]
            phi = [sphereState[1] for sphereState in sphereStates]

        x = np.sin(phi) * np.cos(theta)
        y = np.sin(phi) * np.sin(theta)
        z = np.cos(phi)

        ax.plot(x, y, z)

        #ax.scatter(x, y, z)
        #ax.plot([0, x], [0, y], [0, z])

        # Set an equal aspect ratio
        ax.set_aspect('equal')

        plt.show()
        return

    def getSphericalState(self, state: SpinState) -> tuple[float, float]:

        # Alpha and Beta for the state a|0> + b|1>
        a = state.state[0]
        b = state.state[1]

        # Magnitude of the values
        magA = np.sqrt((a * np.conj(a)).real)
        magB = np.sqrt((b * np.conj(b)).real)

        phi = 2 * np.arccos(magA)

        theta = 0
        if (magA > 0.0001 and magB > 0.0001):
            theta = np.log((b * np.conj(a)) / (magA * magB)) / 1j

        return np.real(theta), np.real(phi)








