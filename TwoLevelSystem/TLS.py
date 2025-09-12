import numpy as np
import numpy.typing as npt


class TLS:

    def __init__(self) -> None:
       
        # Current state of the system
        self.state: npt.NDArray[np.complexfloating] = np.array([])

        # Current time of the system
        self.t: float = 0
        
        # Current resonance of the system (energy difference between the two states, also known as the Larmor frequency)
        self.larmor: float = 0

        return
