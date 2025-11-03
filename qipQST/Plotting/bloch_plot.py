import numpy as np
import matplotlib.pyplot as plt

from ..Simulation.base_qubit import Qubit
from ..Simulation.base_spin_state import SpinState

def plotQubitBloch(qubit: Qubit, i: int | None = None) -> None:
    if i is not None:
        plotBloch(qubit.states[i])

    plotBloch(qubit.states)
    return

def plotBloch(states: list[SpinState] | SpinState) -> None:

    #
    fig = plt.figure()
    ax = fig.add_subplot(1,1,1, projection='3d')
    ax.invert_yaxis()
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

    ax.text(0, 0, 1.1, "$|+z\\rangle$")
    ax.text(0, 1.1, 0, "$|+y\\rangle$")
    ax.text(1.1, 0, 0, "$|+x\\rangle$")

    theta, phi = 0, 0

    if isinstance(states, list):
        sphereStates = [state.getSphericalState() for state in states]
        theta = [sphereState[0] for sphereState in sphereStates]
        phi = [sphereState[1] for sphereState in sphereStates]
    else:
        theta, phi = states.getSphericalState()

    x = np.sin(phi) * np.cos(theta)
    y = np.sin(phi) * np.sin(theta)
    z = np.cos(phi)

    ax.plot(x, y, z)

    # Set an equal aspect ratio
    ax.set_aspect('equal')

    plt.show()
    return
