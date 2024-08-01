 Simulating the three body problem with verlet integration using rust and python and using the simulation to generate a hash function 

first let's look at the simulation without the hash function:

<img src="https://github.com/YoavNir1999/three-body-hash/blob/main/simulation_example.gif" width="400" height="400"/>

as we know, three body gravitational systems are chaotic and therefore every small change in the initial condition results in a very large difference in the end result, making it a good base for a hash function.

now let's look at the hash solution: