import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
from matplotlib.animation import FFMpegWriter
from matplotlib.patches import Rectangle
import struct


def physical_simulation(b1,b2,b3,save):
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
                        interval=10, repeat=False)
    if save:
        ani.save(filename="/Users/yoavnir/Documents/python/three_body_hash/video_simulation.mp4", writer="ffmpeg",fps=30,dpi=150)
    else:
        plt.show()
    return

def chaotic_hash(b1,b2):
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