from sys import get_int_max_str_digits
from qipQST import Gates as qipGates

def testGates() -> None:

    # Get a map of the name of the gate class and the gate itself
    gateMap = dict([(name, cls) for name, cls in qipGates.__dict__.items() if isinstance(cls, type)])
    gateMap.pop("QuantumGate")

    for gateName, gate in gateMap.items():
        print("{}: ".format(gateName), end="")
        error = gate().getMatrix() - gate.getIdealMatrix()
        print("{}".format(error))

    return
