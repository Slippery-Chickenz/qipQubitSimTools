import numpy as np
import numpy.typing as npt

from ..Simulation.base_circuit import QuantumCircuit
from ..Simulation.simulator_pulses import PulseSimulator
from ..Simulation.base_qubit import Qubit

from ..Gates import IdleGate, PiO2X, PiO2Y

from tqdm import tqdm

from typing import Literal

def sweepGuess(guessLarmors: list[float], tau: float, numIterations: int = 1000, secondGate: Literal["X", "Y"] = "Y") -> npt.NDArray[np.floating]:

    # Make a simulator object to run the circuit
    simulator = PulseSimulator()

    # List to hold the output probabilities
    probabilities = np.zeros(len(guessLarmors))


    # Loop over the guess values and run the circuit at that value and tau
    for i in tqdm(range(len(guessLarmors))):
        larmor = guessLarmors[i]

        # Make the circuit with this guess
        ramseyCircuit = QuantumCircuit(larmor)

        # Add the gates for Ramsey
        ramseyCircuit.appendGate(PiO2X())
        ramseyCircuit.appendGate(IdleGate(tau))
        if secondGate == "X":
            ramseyCircuit.appendGate(PiO2X())
        else:
            ramseyCircuit.appendGate(PiO2Y())

        # Set the qubit to run on and the circuit
        simulator.setQubit(Qubit(0))
        simulator.setCircuit(ramseyCircuit)
        
        # Run the sim and get the results
        results = simulator.simulateCircuit(numIterations, 2)
        probabilities[i] = results.qubit.getProb()

    return probabilities

def sweepTau(taus: list[float], guessLarmor, numIterations: int = 1000) -> npt.NDArray[np.floating]:

    # Make a simulator object to run the circuit
    simulator = PulseSimulator()

    # List to hold the output probabilities
    probabilities = np.zeros(len(taus))

    # Loop over the guess values and run the circuit at that value and tau
    for i in tqdm(range(len(taus))):
        tau = taus[i]

        # Make the circuit with this guess
        ramseyCircuit = QuantumCircuit(guessLarmor)

        # Add the gates for Ramsey
        ramseyCircuit.appendGate(PiO2X())
        ramseyCircuit.appendGate(IdleGate(tau))
        ramseyCircuit.appendGate(PiO2Y())

        # Set the qubit to run on and the circuit
        simulator.setQubit(Qubit(0))
        simulator.setCircuit(ramseyCircuit)
        
        # Run the sim and get the results
        results = simulator.simulateCircuit(numIterations, 2)
        probabilities[i] = results.qubit.getProb()

    return probabilities

def sweepGuessAndTau(taus: list[float], guessLarmors: list[float], numIterations: int = 1000, secondGate: Literal["X", "Y"] = "X") -> npt.NDArray[np.floating]:

    # Make a simulator object to run the circuit
    simulator = PulseSimulator()

    # List to hold the output probabilities
    probabilities = np.zeros((len(taus), len(guessLarmors)))

    # Loop over the guess values and run the circuit at that value and tau
    for i in tqdm(range(len(taus))):
        for j in range(len(guessLarmors)):
            tau = taus[i]
            larmor = guessLarmors[j]

            # Make the circuit with this guess
            ramseyCircuit = QuantumCircuit(larmor)

            # Add the gates for Ramsey
            ramseyCircuit.appendGate(PiO2X())
            ramseyCircuit.appendGate(IdleGate(tau))
            if secondGate == "X":
                ramseyCircuit.appendGate(PiO2X())
            else:
                ramseyCircuit.appendGate(PiO2Y())

            # Set the qubit to run on and the circuit
            simulator.setQubit(Qubit(0))
            simulator.setCircuit(ramseyCircuit)
            
            # Run the sim and get the results
            results = simulator.simulateCircuit(numIterations, 2)
            probabilities[i][j] = results.qubit.getProb()

    return probabilities.transpose()








