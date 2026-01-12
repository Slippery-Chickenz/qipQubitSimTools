import numpy as np
import numpy.typing as npt

from tqdm import tqdm

from ..Simulation.base_circuit import QuantumCircuit
from ..Simulation.simulator_pulses import PulseSimulator
from ..Simulation.base_spin_state import SpinStateType

from ..Gates import QuantumGate

def sweepGuess(
    guessLarmors: list[float], 
    gate: QuantumGate,
    startingState: SpinStateType = "+Z",
    probabilityState: SpinStateType = "+Z",
    numIterations: int = 1000, 
) -> npt.NDArray[np.floating]:

    # Make a simulator object to run the circuit
    simulator = PulseSimulator()

    # List to hold the output probabilities
    probabilities = np.zeros(len(guessLarmors))

    # Loop over the guess values
    for i in tqdm(range(len(guessLarmors))):
        larmor = guessLarmors[i]

        # Make the circuit with this guess
        gateTestCircuit = QuantumCircuit(larmor)

        # Add the gate to test
        gateTestCircuit.appendGate(gate)

        # Set the qubit to run on and the circuit
        simulator.setCircuit(gateTestCircuit)
        
        # Run the sim and get the results
        results = simulator.simulateCircuit(numIterations, 2, startingState=startingState)
        probabilities[i] = results.getProb(state=probabilityState)

    return probabilities



