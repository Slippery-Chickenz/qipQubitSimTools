import numpy as np
import numpy.typing as npt

from ..Simulation.base_circuit import QuantumCircuit
from ..Simulation.simulator_pulses import PulseSimulator
from ..Simulation.base_qubit import Qubit

from ..Gates import QuantumGate
from ..Pulses import ConstantPulse

from tqdm import tqdm

def rabiCycle(
    guessLarmor: float, 
    time: float, 
    amplitude: float = 1, 
    frequency: float = 0, 
    numIterations: int = 1000, 
) -> tuple[npt.NDArray[np.floating], npt.NDArray[np.floating]]:

    # Make a simulator object to run the circuit
    simulator = PulseSimulator()

    pulse = ConstantPulse(time, amplitude, frequency, 0)
    constantPulseGate = QuantumGate()
    constantPulseGate.appendPulse(pulse)

    # Make the circuit with this guess
    rabiCircuit = QuantumCircuit(guessLarmor)

    # Add a single constant pulse
    rabiCircuit.appendGate(constantPulseGate)

    # Set the qubit to run on and the circuit
    simulator.setCircuit(rabiCircuit)
    
    # Run the sim and get the results
    results = simulator.simulateCircuit(numIterations, numIterations)

    results.plotBloch()

    return results.getTimes(), results.getProbs()

def sweepGuess(
    guessLarmors: list[float], 
    time: float, 
    amplitude: float = 1, 
    frequency: float = 0, 
    numIterations: int = 1000, 
) -> npt.NDArray[np.floating]:

    # Make a simulator object to run the circuit
    simulator = PulseSimulator()

    # List to hold the output probabilities
    probabilities = np.zeros(len(guessLarmors))

    pulse = ConstantPulse(time, amplitude, frequency, 0)
    constantPulseGate = QuantumGate()
    constantPulseGate.appendPulse(pulse)

    # Loop over the guess values and run the circuit at that value and tau
    for i in tqdm(range(len(guessLarmors))):
        larmor = guessLarmors[i]

        # Make the circuit with this guess
        rabiCircuit = QuantumCircuit(larmor)

        # Add the gates for Ramsey
        rabiCircuit.appendGate(constantPulseGate)

        # Set the qubit to run on and the circuit
        simulator.setCircuit(rabiCircuit)
        
        # Run the sim and get the results
        results = simulator.simulateCircuit(numIterations, 2)
        probabilities[i] = results.getProb()

    return probabilities

def sweepGuessAndTime(
    times: list[float], 
    guessLarmors: list[float], 
    amplitude: float,
    frequency: float = 0,
    numIterations: int = 1000,
) -> npt.NDArray[np.floating]:

    pass

    # Make a simulator object to run the circuit
    simulator = PulseSimulator()

    # List to hold the output probabilities
    probabilities = np.zeros((len(times), len(guessLarmors)))

    # Loop over the guess values and run the circuit at that value and tau
    for i in tqdm(range(len(times))):
        for j in range(len(guessLarmors)):
            pulseTime = times[i]
            larmor = guessLarmors[j]

            pulse = ConstantPulse(pulseTime, amplitude, frequency, 0)
            constantPulseGate = QuantumGate()
            constantPulseGate.appendPulse(pulse)

            # Make the circuit with this guess
            rabiCircuit = QuantumCircuit(larmor)

            # Add a single constant pulse
            rabiCircuit.appendGate(constantPulseGate)

            # Set the qubit to run on and the circuit
            simulator.setCircuit(rabiCircuit)
            
            # Run the sim and get the results
            results = simulator.simulateCircuit(numIterations, 2)

            probabilities[i][j] = results.getProb()

    return probabilities.transpose()





