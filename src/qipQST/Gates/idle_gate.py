from .base_gate import QuantumGate

from ..Pulses.constant import ConstantPulse

class IdleGate(QuantumGate):

    def __init__(self, time: float) -> None:
        super().__init__()
        self.appendPulse(ConstantPulse(time, 0, 0, 0))
        return

