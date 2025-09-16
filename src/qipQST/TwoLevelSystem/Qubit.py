import numpy as np

import numpy.typing as npt

class Qubit:

    def __init__(self) -> None:
       
        # Current state of the system
        self.states: npt.NDArray[np.complexfloating]

        # Current time of the system
        self.times: npt.NDArray[np.floating]
        
        # Current resonance of the system 
        # (energy difference between the two states, also known as the Larmor frequency)
        self.larmor: float = 0

        return
