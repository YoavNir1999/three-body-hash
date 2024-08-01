import three_body_hash
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation, PillowWriter
from matplotlib.animation import FFMpegWriter
from matplotlib.patches import Rectangle

#    mass  x    y    Vx    Vy
b1 = [1.8, 0.0, 3.0, 0.3, -0.5]
b2 = [1.7, 0.0, -3.0, 0.3, 0.5]
b3 = [1.6, 2.5, 0.0, -0.3, 0.2]

# setting the time delta and number of iterations
dt = 1e-1
iterations = 2000

# calling rust function
[b1,b2,b3] = three_body_hash.calc_three_body(b1,b2,b3,dt,iterations)
#[b1,b2,b3] = three_body_hash.calc_hash_animation(b1,b2,1.4,"/Users/yoavnir/Documents/python/three_body_hash/sample photo.jpg",dt,iterations)

# animation process
fig, ax = plt.subplots(1, 1)
fig.set_size_inches(5,5)
 
def animate(i):
    ax.clear()
    # Get the point from the points list at index i
    # Plot that point using the x and y coordinates
    ax.add_patch(Rectangle((-4.0,-4.0),8.0,8.0, linewidth=1, edgecolor='k', facecolor='none'))

    ax.plot(b1[0][i], b1[1][i], marker='o',color='r')
    ax.plot(b1[0][max(i-20,0):i], b1[1][max(i-20,0):i],color='r')

    # Plot that point using the x and y coordinates
    ax.plot(b2[0][i], b2[1][i], marker='o',color='g')
    ax.plot(b2[0][max(i-20,0):i], b2[1][max(i-20,0):i],color='g')

    # Plot that point using the x and y coordinates
    ax.plot(b3[0][i], b3[1][i], marker='o',color='b')
    ax.plot(b3[0][max(i-20,0):i], b3[1][max(i-20,0):i],color='b')

    ax.set_xlim([-5, 5])
    ax.set_ylim([-5, 5])


ani = FuncAnimation(fig, animate, frames=len(b1[0]),
                    interval=10, repeat=False)
plt.show()
# Save the animation as an animated GIF
ani.save("simulation.gif", dpi=200,
         writer=PillowWriter(fps=20))

#ani.save(filename="/Users/yoavnir/Documents/python/three_body_hash/video_simulation.mp4", writer="ffmpeg",fps=30,dpi=150)