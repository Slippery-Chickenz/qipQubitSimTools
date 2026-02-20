import numpy as np
import numpy.typing as npt

from .base_spin_state import *

# #from .base_spin_state import (upXState, 
#                               downXState, 
#                               upYState, 
#                               downYState, 
#                               upZState, 
#                               downZState)

upXDensity: npt.NDArray[np.complexfloating] = np.outer(upXState.conj(), upXState)
downXDensity: npt.NDArray[np.complexfloating] = np.outer(downXState.conj(), downXState)
upYDensity: npt.NDArray[np.complexfloating] = np.outer(upYState.conj(), upYState)
downYDensity: npt.NDArray[np.complexfloating] = np.outer(downYState.conj(), downYState)
upZDensity: npt.NDArray[np.complexfloating] = np.outer(upZState.conj(), upZState)
downZDensity: npt.NDArray[np.complexfloating] = np.outer(downZState.conj(), downZState)

def getBasisDensity(stateStr: StateStr = "+Z") -> npt.NDArray[np.complexfloating]:
    match stateStr:
        case "+X":
            return upXDensity
        case "-X":
            return downXDensity
        case "+Y":
            return upYDensity
        case "-Y":
            return downYDensity
        case "+Z":
            return upZDensity
        case "-Z":
            return downZDensity

def getDensityMatrix(state: SpinStateType) -> npt.NDArray[np.complexfloating]:
    if isinstance(state, str):
        return getBasisDensity(state)
    elif isinstance(state, SpinState):
        return np.outer(state.state.conj(), state.state)
    return np.outer(state.conj(), state)

class SpinDensity:

    def __init__(self, state: SpinStateType = "+Z") -> None:
        # Density matrix for one spin
        self.matrix: npt.NDArray[np.complexfloating] = getDensityMatrix(state)
        return
