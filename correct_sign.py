"""
This script receives two vector files and correct the sign of the former 
so that the correlation coefficient of two vectors is positive

Usage: python correct_sign.py [corrected vector file path] [reference vector file path] [output file path]

vector file contains single elements in each line
two vectors must have the same size
"""

import sys
import numpy as np

corrected = read_vector(sys.argv[1])
reference = read_vector(sys.argv[2])

if np.corrcoef(corrected, reference)[0, 1] < 0:
    corrected *= -1.0

write_vector(sys.argv[3], corrected)

def read_vector(path):
    f = open(path)
    vec = []
    for line in f:
        vec.append(float(line.strip()))
    f.close()
    return np.array(vec)

def write_vector(path, vector):
    f = open(path)
    f.write('\n'.join([str(x) for x in vector]))
    f.close()
