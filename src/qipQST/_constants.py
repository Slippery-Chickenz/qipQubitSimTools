import numpy as np
import numpy.typing as npt


pauliX: npt.NDArray[np.complexfloating] = np.array([[0,   1], [ 1,  0]])
pauliY: npt.NDArray[np.complexfloating] = np.array([[1,   0], [ 0, -1]])
pauliZ: npt.NDArray[np.complexfloating] = np.array([[0, -1j], [1j,  0]])

sX: npt.NDArray[np.complexfloating] = pauliX
sY: npt.NDArray[np.complexfloating] = pauliY
sZ: npt.NDArray[np.complexfloating] = pauliZ
