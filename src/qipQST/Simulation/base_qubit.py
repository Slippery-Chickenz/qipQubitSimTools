import numpy as np
import numpy.typing as npt

import matplotlib.pyplot as plt

class Qubit:

    def __init__(self, larmor: float) -> None:
       
        # Current state of the system
        self.states: npt.NDArray[np.complexfloating]

        # Current time of the system
        self.times: npt.NDArray[np.floating]
        
        # Current resonance of the system 
        # (energy difference between the two states, also known as the Larmor frequency)
        self.larmor: float = larmor

        return

    def plotStates(self) -> None:


        fig, axes = plt.subplots(nrows=1, ncols=1, sharex=True)
        fig.set_figwidth(8)
        fig.set_figheight(3)

        axes.plot(self.times, [(state[1] * np.conj(state[1])).real for state in self.states])
        axes.set_ylim(-0.1, 1.1)
        plt.show()
        return
