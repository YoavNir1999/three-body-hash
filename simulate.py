import three_body_hash
import functions

#    mass    x    y      Vx    Vy
b1 = [8e4, 0.0, 120.0, 20.0, -16.0]
b2 = [7e4, 0.0, -120.0, 0.0, 16.0]
b3 = [9e4, -40.0, 0.0, 16.0, 14.0]

# setting the time delta and number of iterations
dt = 5e-2
iterations = 100
demo_file = "/Users/yoavnir/Documents/python/three_body_hash/sample photo.jpg"

# calling rust function
#[b1,b2,b3] = three_body_hash.calc_three_body(b1,b2,b3,dt,iterations)
[b1,b2,b3] = three_body_hash.calc_hash_animation(b1,b2,8e4,demo_file,dt,iterations)

functions.physical_simulation(b1,b2,b3,False)
print(functions.hash_output(b1,b2))