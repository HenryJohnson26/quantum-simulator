## Extended Stabilizer Simulator
purpose:
This is an exploratory project into advancements in classical simulations of quantum information.
It is largely based on the work of Bravyi et al. 2019 in which they proposed an efficent method
of simulating the evolution of quantum information in a quantum computer under the gate model.

# Previous work
My work began with a statevector - keeping track of all possible outcome and their magnitudes,
applying gates to the state an measuring with some random weighted by the magnitude of each outome.
This is a completely accurate representation of gate-model quantum computation however it requires
non-polyomial time and space in the number of qubits. This work can be found in src/statevector.
Polynomial time and space methods have been found to be effective on clifford-only gates (single 
qubit, non-entangling) and so my work continued in src/tableau in which a tableau representation
of stabilizers is used to simulate the state in polynomial time.
The goal of extending this clifford-only stabilizer is what brought on the work from Bravyi 2019
and is found in src/rank-decomp.
This method is bounded error nondeterministic polynomial and has time complexity of O(2^{0.23n})
which is substantial enough to simulate intermediate scale quantum computers with some error but
better validation of real quantum systems will require less expensive simulation.


# Areas of research
There is active research in gate rescheduling (sometimes referred to as quantum compiling) in 
which by delaying, grouping and potentially eliminating non-clifford gates, one can achieve less
expensive simulation due to the method of the stabilizer rank decomposition simulation method in 
Bravyi 2019. This happens because the extended-stabilizer method performs clifford only gates in
polynomial time until a non-clifford gate is reached in which the lifting lemma is applied and a
non-deterministic state of stabilizers is tracked.
The field leaves room for research into things like:
further push-back of non-clifford gates
gate rescheduling using the ZX-calculus (ZX-calculus typically results in more proficient circuit
opt)
grouping non-cliffords then measuring intermittently (sometimes called gadgeting)
parallelization and subsequent gpu acceleration of clifford-gate operations and/or non-clifford
state promotions

# Impact
Currently, simulators are important for validation of intermediate-scale quantum devices and for
ecucational purposes as well. Most quantum algorithms that people expect to be usefult post quantum
break have low non-clifford density. This means in order to validate algorithms and quantum systems
at useful scales, we need effective simulation for low non-clifford circuits. Potentially the most
exciting candidate in this field is the extended-stabilizer method and is widely used in 
applications like qiskit-aer. With or without significant results, the research areas identified
will likely impact the future of quantum simulation and computing as a whole.