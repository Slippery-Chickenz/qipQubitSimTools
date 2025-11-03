import numpy as np
import numpy.typing as npt

from typing import Literal

pauliX: npt.NDArray[np.complexfloating] = np.array([[0,   1], [ 1,  0]])
pauliY: npt.NDArray[np.complexfloating] = np.array([[0, -1j], [1j,  0]])
pauliZ: npt.NDArray[np.complexfloating] = np.array([[1,   0], [ 0, -1]])

sX: npt.NDArray[np.complexfloating] = pauliX / 2
sY: npt.NDArray[np.complexfloating] = pauliY / 2
sZ: npt.NDArray[np.complexfloating] = pauliZ / 2

