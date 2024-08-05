 Simulating the 2D three body problem in a box with verlet integration using rust and python and using the simulation to generate a hash function:
 
 This program is implemented using both rust and python. In order to run it you will need the rust compiler, python interpreter and pyo3, maturin and matplotlib libraries.
 First compile the rust code to a python module with the command "maturin develop --release", make sure you are using a venv enviroment. Then you can run the python code with the examples or call the built in functions on your own data.

 The program currently supports solving and visualizing the 2D three body problem in a box, simulating the divergence of two systems with different initial conditions in this problem and using the problem as a hash function with the abilty to either quickly generate an output or generate an animation of the problem (very slow). The programs code currently contains an example for each use case.

Let's review how the program works.

First we will look at the simulation without the hash function:

<img src="https://github.com/YoavNir1999/three-body-hash/blob/main/simulation_example.gif" width="400" height="400"/>

As we know, three body gravitational systems are chaotic and therefore every small change in the initial condition results in a very large difference in the end result, making it a good base for a hash function.

Let's see how quickly two solutions diverge, here we start with the same starting conditions except the blue ball starts with the speed of 16m/s in the x direction in the opaque simulation and with the speed of 16.1m/s in the x direction in the partially transparent simulation:

<img src="https://github.com/YoavNir1999/three-body-hash/blob/main/divergence simulation.gif" width="400" height="400"/>

Now let's look at the hash solution:
Say you have a vector of bytes V, for example a file seprated to a vector of bytes. We assume the length of the vector is at least 4 bytes. Now for the initial conditions of the system the masses a and b always begin at the same position with the same velocity vectors and for the mass c we pick the position vector to be (V[0],V[1]) and the velocity vector to be (V[2],V[3]) all converted to floating point numbers. The program then iterates over the vector of bytes and computes a fixed predetermined number of iterations of the verlet integral, after the iterations are complete we change either the position or the velocity of c in one axis to be the value of V[n] converted to floating point where n is the iteration number, the field we change is determined by the remainder of n/4.
After the algorithm finishes running the output vector is (position of a, velocity of a, position of b, velocity of b) converted to hexa, we discard all information about c making the method even harder to reverse.
Let's look a visualization of the process where the blue ball represents mass c:
<img src="https://github.com/YoavNir1999/three-body-hash/blob/main/hash animation.gif" width="400" height="400"/>

Finally, the fast hash function only returns the output and discards information about the system along the way, it runs in linear time O(n) where n is the length of the input vector and returns an output such as "c08bbcd3c272c940425242a8c20e9202" which represents parameters about masses a and b as explained earlier.