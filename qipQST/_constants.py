import numpy as np
import numpy.typing as npt

PAULIX: npt.NDArray[np.complexfloating] = np.array([[0,   1], [ 1,  0]])
PAULIY: npt.NDArray[np.complexfloating] = np.array([[0, -1j], [1j,  0]])
PAULIZ: npt.NDArray[np.complexfloating] = np.array([[1,   0], [ 0, -1]])

SX: npt.NDArray[np.complexfloating] = PAULIX / 2
SY: npt.NDArray[np.complexfloating] = PAULIY / 2
SZ: npt.NDArray[np.complexfloating] = PAULIZ / 2
