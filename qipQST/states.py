import numpy as np
import numpy.typing as npt

from typing import Literal

StateStr =  Literal["+X", "-X", "+Y", "-Y", "+Z", "-Z"] 
BasisStr =  Literal["X", "Y", "Z"] 

def getBasisState(stateStr: StateStr = "+Z") -> npt.NDArray[np.complexfloating]:
    match stateStr:
        case "+X":
            return upXState()
        case "-X":
            return downXState()
        case "+Y":
            return upYState()
        case "-Y":
            return downYState()
        case "+Z":
            return upZState()
        case "-Z":
            return downZState()

def upXState() -> npt.NDArray[np.complexfloating]:
    return np.array([1, 1]) / np.sqrt(2)
def downXState() -> npt.NDArray[np.complexfloating]:
    return np.array([1, -1]) / np.sqrt(2)
def upYState() -> npt.NDArray[np.complexfloating]:
    return np.array([1, 1j]) / np.sqrt(2)
def downYState() -> npt.NDArray[np.complexfloating]:
    return np.array([1, -1j]) / np.sqrt(2)
def upZState() -> npt.NDArray[np.complexfloating]:
    return np.array([1, 0])
def downZState() -> npt.NDArray[np.complexfloating]:
    return np.array([0, 1])
