import three_body_hash
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation, PillowWriter
from matplotlib.animation import FFMpegWriter
from matplotlib.patches import Rectangle

#    mass    x    y      Vx    Vy
b1 = [8e4, 0.0, 120.0, 20.0, -16.0]
b2 = [7e4, 0.0, -120.0, 0.0, 16.0]
b3 = [9e4, -40.0, 0.0, 16.0, 14.0]

# setting the time delta and number of iterations
dt = 1e-1
iterations = 2000

# calling rust function
[b1,b2,b3] = three_body_hash.calc_three_body(b1,b2,b3,dt,iterations)
#[b1,b2,b3] = three_body_hash.calc_hash_animation(b1,b2,3.0,"/Users/yoavnir/Documents/python/three_body_hash/sample photo.jpg",dt,iterations)

# animation process
fig, ax = plt.subplots(1, 1)
fig.set_size_inches(5,5)
 
def animate(i):
    ax.clear()
    # Get the point from the points list at index i
    # Plot that point using the x and y coordinates
    ax.add_patch(Rectangle((-128.0,-128.0),256.0,256.0, linewidth=1, edgecolor='k', facecolor='none'))

    ax.plot(b1[0][i], b1[1][i], marker='o',color='r',markersize=7)
    ax.plot(b1[0][max(i-20,0):i], b1[1][max(i-20,0):i],color='r')

    # Plot that point using the x and y coordinates
    ax.plot(b2[0][i], b2[1][i], marker='o',color='g',markersize=7)
    ax.plot(b2[0][max(i-20,0):i], b2[1][max(i-20,0):i],color='g')

    # Plot that point using the x and y coordinates
    ax.plot(b3[0][i], b3[1][i], marker='o',color='b',markersize=7)
    ax.plot(b3[0][max(i-20,0):i], b3[1][max(i-20,0):i],color='b')

    ax.set_xlim([-128, 128])
    ax.set_ylim([-128, 128])



ani = FuncAnimation(fig, animate, frames=len(b1[0]),
                    interval=15, repeat=False)
plt.show()
# Save the animation as an animated GIF
#ani.save("simulation.gif", dpi=200,
#         writer=PillowWriter(fps=20))

#ani.save(filename="/Users/yoavnir/Documents/python/three_body_hash/video_simulation.mp4", writer="ffmpeg",fps=30,dpi=150)