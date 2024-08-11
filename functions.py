import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
from matplotlib.animation import FFMpegWriter
from matplotlib.patches import Rectangle
import struct
import numpy as np
import three_body_hash

def physical_simulation(b1,b2,b3,save):
    # animation process
    fig, ax = plt.subplots(1, 1)
    fig.set_size_inches(5,5)
    
    def animate(i):
        ax.clear()
        # Get the point from the points list at index i
        # Plot that point using the x and y coordinates
        ax.add_patch(Rectangle((-128.0,-128.0),256.0,256.0, linewidth=1, edgecolor='k', facecolor='none'))

        ax.plot(b1[0][i], b1[1][i], marker='o',color='r',markersize=7,label="b1")
        ax.plot(b1[0][max(i-20,0):i], b1[1][max(i-20,0):i],color='r')

        # Plot that point using the x and y coordinates
        ax.plot(b2[0][i], b2[1][i], marker='o',color='g',markersize=7,label="b2")
        ax.plot(b2[0][max(i-20,0):i], b2[1][max(i-20,0):i],color='g')

        # Plot that point using the x and y coordinates
        ax.plot(b3[0][i], b3[1][i], marker='o',color='b',markersize=7,label="b3")
        ax.plot(b3[0][max(i-20,0):i], b3[1][max(i-20,0):i],color='b')

        ax.set_xlim([-128, 128])
        ax.set_ylim([-128, 128])



    ani = FuncAnimation(fig, animate, frames=len(b1[0]),
                        interval=10, repeat=False)
    if save:
        ani.save(filename="/Users/yoavnir/Documents/python/three_body_hash/video_simulation.mp4", writer="ffmpeg",fps=30,dpi=150)
    else:
        plt.show()
    return

def calc_hash(b1,b2):
    return float_to_hex_concat(b1[0][-1],b1[1][-1],b2[0][-1],b2[1][-1])

def float_to_hex_concat(f1, f2, f3, f4):
    # Convert each float to its hexadecimal representation
    hex_f1 = struct.pack('!f', f1).hex()
    hex_f2 = struct.pack('!f', f2).hex()
    hex_f3 = struct.pack('!f', f3).hex()
    hex_f4 = struct.pack('!f', f4).hex()
    
    # Concatenate the hexadecimal representations
    hex_concat = hex_f1 + hex_f2 + hex_f3 + hex_f4
    
    return hex_concat

def divergence_simulation(b1,b2,b3,b1d,b2d,b3d,save):
    # animation process
    fig, ax = plt.subplots(1, 1)
    fig.set_size_inches(7,7)
    alpha = 0.4
    
    def animate(i):
        ax.clear()
        # Get the point from the points list at index i
        # Plot that point using the x and y coordinates
        ax.add_patch(Rectangle((-128.0,-128.0),256.0,256.0, linewidth=1, edgecolor='k', facecolor='none'))

        ax.plot(b1[0][i], b1[1][i], marker='o',color='r',markersize=7,label="original b1")
        ax.plot(b1[0][max(i-20,0):i], b1[1][max(i-20,0):i],color='r')

        ax.plot(b1d[0][i], b1d[1][i], marker='o',color='r',markersize=7,alpha=alpha,label="diverging b1")
        ax.plot(b1d[0][max(i-20,0):i], b1d[1][max(i-20,0):i],color='r',alpha=alpha)

        ax.plot(b2[0][i], b2[1][i], marker='o',color='g',markersize=7,label="original b2")
        ax.plot(b2[0][max(i-20,0):i], b2[1][max(i-20,0):i],color='g')

        ax.plot(b2d[0][i], b2d[1][i], marker='o',color='g',markersize=7,alpha=alpha,label="diverging b2")
        ax.plot(b2d[0][max(i-20,0):i], b2d[1][max(i-20,0):i],color='g',alpha=alpha)

        ax.plot(b3[0][i], b3[1][i], marker='o',color='b',markersize=7,label="original b3")
        ax.plot(b3[0][max(i-20,0):i], b3[1][max(i-20,0):i],color='b')
        
        ax.plot(b3d[0][i], b3d[1][i], marker='o',color='b',markersize=7,alpha=alpha,label="diverging b3")
        ax.plot(b3d[0][max(i-20,0):i], b3d[1][max(i-20,0):i],color='b',alpha=alpha)

        ax.set_xlim([-128, 128])
        ax.set_ylim([-128, 128])
        #ax.legend(loc="upper right",ncol=3,fontsize=6)



    ani = FuncAnimation(fig, animate, frames=len(b1[0]),
                        interval=10, repeat=False)
    if save:
        ani.save(filename="/Users/yoavnir/Documents/python/three_body_hash/video_simulation.mp4", writer="ffmpeg",fps=30,dpi=150)
    else:
        plt.show()
    return

def generate_uniform_array(n):
    # Generate n uniformly distributed numbers between 0 and 255
    uniform_dist = np.random.uniform(low=0, high=256, size=n)
    
    # Cast to integers
    uniform_dist_int = uniform_dist.astype(int)
    
    return uniform_dist_int

def plot_distribution(hashes):
    # animation process
    fig, ((b1x,b1y),(b2x,b2y)) = plt.subplots(2, 2)
    fig.set_size_inches(7,7)

    b1x.hist([h[0] for h in hashes],color="r")
    b1x.set_title("mass a in x plane")
    
    b1y.hist([h[1] for h in hashes],color="r")
    b1y.set_title("mass a in y plane")

    b2x.hist([h[2] for h in hashes],color="g")
    b2x.set_title("mass b in x plane")
    
    b2y.hist([h[3] for h in hashes],color="g")
    b2y.set_title("mass b in y plane")

    #ax.scatter([h[0] for h in hashes],[h[1] for h in hashes],color="r")
    #ax.scatter([h[2] for h in hashes],[h[3] for h in hashes],color="g")

    plt.show()
    return

def distribution_test(b1,b2,b3mass,number_of_tests,length_of_test_input,dt,file_iterations):
    rand_arrays = [generate_uniform_array(length_of_test_input) for i in range(number_of_tests)]

    hashes = []

    for arr in rand_arrays:
        hashes += [three_body_hash.fast_hash_data(b1,b2,b3mass,arr,dt,file_iterations)]

    return hashes
