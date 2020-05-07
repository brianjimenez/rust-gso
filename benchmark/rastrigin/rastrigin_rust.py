#!/usr/bin/env python3
# coding: utf-8

"""
https://en.wikipedia.org/wiki/Rastrigin_function

Non-convex function for testing optimization algorithms.
"""

from matplotlib import cm
from mpl_toolkits.mplot3d import Axes3D
import math
import matplotlib.pyplot as plt
import numpy as np
import os

def rastrigin(*X, **kwargs):
    A = kwargs.get('A', 10)
    return A + sum([(x**2 - A * np.cos(2 * math.pi * x)) for x in X])


def read_points(file_name):
    xs1 = []
    ys1 = []
    xs2 = []
    ys2 = []
    finished = False
    with open(file_name) as input_handler:
        for line in input_handler:
            finished = finished or line.startswith('Step 100')
            if line.startswith('Step'):
                continue
            line = line.rstrip(os.linesep)
            start = line.index('[')
            end = line.index(']')
            x, y = line[start+1:end-2].strip().split()
            if not finished:
                xs1.append(float(x[:-2]))
                ys1.append(float(y))
            else:
                xs2.append(float(x[:-2]))
                ys2.append(float(y))
    return xs1, ys1, xs2, ys2


if __name__ == '__main__':

    xs1, ys1, xs2, ys2 = read_points('data_rust.txt')

    # Initial
    X = np.linspace(-4, 4, 200)    
    Y = np.linspace(-4, 4, 200)    
    X, Y = np.meshgrid(X, Y)
    Z = rastrigin(X, Y, A=10)

    fig = plt.figure()
    ax = fig.gca()
    ax.plot(xs1, ys1, 'bx')
    #ax.plot_surface(X, Y, Z, rstride=1, cstride=1, cmap=cm.plasma, linewidth=0, antialiased=False)
    cp = ax.contourf(X, Y, Z)
    fig.colorbar(cp) # Add a colorbar to a plot
    #ax.view_init(elev=-90., azim=90)
    plt.savefig('rastrigin_rust_1.png')

    fig = plt.figure()
    ax = fig.gca()
    ax.plot(xs2, ys2, 'bx')
    cp = ax.contourf(X, Y, Z)
    fig.colorbar(cp)
    plt.savefig('rastrigin_rust_2.png')
