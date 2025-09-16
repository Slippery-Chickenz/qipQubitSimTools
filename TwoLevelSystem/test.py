
from PulseSimulator import PulseSimulator
from QuantumCircuit import QuantumCircuit
from ConstantPulse import ConstantPulse
from RampPulse import RampPulse
from QuantumGate import QuantumGate

testGate = QuantumGate()

testRamp = RampPulse(5, (0, 1), (0, 10), (0, 0))
#testRamp.plotPulse()
#testConst = ConstantPulse(5, 1, 10, 0)
testRamp2 = RampPulse(5, (1, 1), (10, 0), (0, 0))

testRamp3 = RampPulse(5, (1, 0), (0, 10), (0, 0))

testGate.appendPulse(testRamp)
#testGate.appendPulse(testConst)
testGate.appendPulse(testRamp2)
testGate.appendPulse(testRamp3)


testGate.plotPulses()
