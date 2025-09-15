import numpy as np
import numpy.typing as npt


class Pulse:

    def __init__(self) -> None:

        # Amplitude and frequency values of the pulse
        self.amplitude: npt.NDArray[np.floating]
        self.frequency: npt.NDArray[np.floating]

        return
