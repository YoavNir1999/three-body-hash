import three_body_hash
import functions

#    mass    x    y      Vx    Vy
b1s = [8e4, 0.0, 120.0, 20.0, -16.0]
b2s = [7e4, 0.0, -120.0, 0.0, 16.0]
b3s1 = [9e4, -40.0, 0.0, 16.0, 14.5]
b3s2 = [9e4, -40.0, 0.0, 16.1, 14.5]


# setting the time delta and number of iterations
dt = 5e-2
iterations = 1000
demo_file = "/Users/yoavnir/Documents/python/three_body_hash/sample photo.jpg"

# calling rust function
[b1,b2,b3] = three_body_hash.calc_three_body(b1s,b2s,b3s1,dt,iterations)
[b1d,b2d,b3d] = three_body_hash.calc_three_body(b1s,b2s,b3s2,dt,iterations)

#[b1,b2,b3] = three_body_hash.calc_hash_animation(b1s,b2s,9.5e4,demo_file,dt,iterations)

#[b1x,b1y,b2x,b2y] = three_body_hash.fast_hash(b1s,b2s,9.5e4,demo_file,dt,iterations)


#functions.physical_simulation(b1,b2,b3,False)
#print(functions.float_to_hex_concat(b1x,b1y,b2x,b2y))
functions.divergence_simulation(b1,b2,b3,b1d,b2d,b3d,False)