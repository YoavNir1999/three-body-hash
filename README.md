 Simulating the three body problem with verlet integration using rust and python and using the simulation to generate a hash function 

first let's look at the simulation without the hash function:
![](https://github.com/YoavNir1999/three-body-hash/blob/main/simulation.gif)

<img src="https://github.com/YoavNir1999/three-body-hash/blob/main/simulation.gif" width="250" height="250"/>

as we know, three body gravitational systems are chaotic and therefore every small change in the initial condition results in a very large difference in the end result, making it a good base for a hash function.
in addition, there are multiple initial conditions that end in the same result making the output irreversal, which is great in our case.

now let's look at the hash solution: