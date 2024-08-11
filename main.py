import three_body_hash
import functions

#    mass    x    y      Vx    Vy
b1s = [8e4, 0.0, 120.0, 20.0, -16.0]
b2s = [8e4, 0.0, -120.0, -20.0, 16.0]
b3s1 = [9e4, -40.0, 0.0, 16.0, 14.5]

# mass used for divergence simulation
b3s2 = [9e4, -40.0, 0.0, 16.1, 14.5]


# setting the time delta and number of iterations
dt = 5e-2
file_iterations = 100
simulation_iterations = 2000
demo_file = "/Users/yoavnir/Documents/python/three_body_hash/sample photo.jpg"

# calling rust functions

###* regular physics simulations
#[b1,b2,b3] = three_body_hash.calc_three_body(b1s,b2s,b3s1,dt,simulation_iterations)
#[b1d,b2d,b3d] = three_body_hash.calc_three_body(b1s,b2s,b3s2,dt,simulation_iterations)

###* hash simulation
#[b1,b2,b3] = three_body_hash.calc_hash_animation(b1s,b2s,9.5e4,demo_file,dt,file_iterations)

###* fast hashing
#[b1x,b1y,b2x,b2y] = three_body_hash.fast_hash_file(b1s,b2s,9.5e4,demo_file,dt,file_iterations)

###* ploting
#functions.physical_simulation(b1,b2,b3,False)
#print(functions.float_to_hex_concat(b1x,b1y,b2x,b2y))
#functions.divergence_simulation(b1,b2,b3,b1d,b2d,b3d,False)

###* checking uniformality
#hashes = functions.distribution_test(b1s,b2s,9.5e4,10000,30,dt,file_iterations)
#functions.plot_distribution(hashes)
