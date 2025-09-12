from warnings import warn
import numpy as np

import numpy.typing as npt
from typing import Literal, Annotated


class TLS:

    def __init__(self) -> None:
       
        # Current state of the system
        self.state: npt.NDArray[np.complexfloating] = np.array([])

        # Current time of the system
        self.t: float = 0
        
        # Current resonance of the system (energy difference between the two states, also known as the Larmor frequency)
        self.larmor: float = 0

        return

    def setState(self, newState: Annotated[npt.NDArray[np.complexfloating], Literal[2]]) -> None:
        self.state = newState
        return

    def setTime(self, newTime: float) -> None:
        self.t = newTime
        return

    def setLarmor(self, newLarmor: float) -> None:
        self.larmor = newLarmor
        return
