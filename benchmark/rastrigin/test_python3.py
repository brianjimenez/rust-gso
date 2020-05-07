#!/usr/bin/env python3
# coding: utf-8

from lightdock.gso.parameters import GSOParameters
from lightdock.gso.searchspace.benchmark_ofunctions import J2
from lightdock.gso.algorithm import GSOBuilder
from lightdock.gso.boundaries import Boundary, BoundingBox
from lightdock.mathutil.lrandom import MTGenerator


gso_parameters = GSOParameters()
objective_function = J2()
bounding_box = BoundingBox([Boundary(-4.0, 4.0), Boundary(-4.0, 4.0)])
number_of_glowworms = 200
random_number_generator = MTGenerator(324324324)
builder = GSOBuilder()
gso = builder.create(number_of_glowworms, 
                     random_number_generator, 
                     gso_parameters, 
                     objective_function, 
                     bounding_box)        

gso.swarm.save(0, '.', file_name='gso_0.out')
for glowworm in gso.swarm.glowworms:
    print(glowworm.landscape_positions[0])

print('Step 1')

gso.run(100)

print('Step 100')
for glowworm in gso.swarm.glowworms:
    print(glowworm.landscape_positions[0])