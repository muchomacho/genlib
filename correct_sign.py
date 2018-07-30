"""
This script receives two vector files and correct the sign of the former 
so that the correlation coefficient of two vectors is positive

Usage: python correct_sign.py [corrected vector file path] [reference vector file path] [output file path]

vector file contains single elements in each line
two vectors must have the same size
"""

import sys
import numpy as np

def read_vector(path):
    f = open(path, 'r')
    vec = []
    for line in f:
        vec.append(float(line.strip()))
    f.close()
    return np.array(vec)

def write_vector(path, vector):
    f = open(path, 'w')
    f.write('\n'.join([str(x) for x in vector]))
    f.close()

corrected = read_vector(sys.argv[1])
reference = read_vector(sys.argv[2])
if corrected.shape[0] != reference.shape[0]:
    print("sizes of two vectors are not equal")
    sys.exit(1)
valid_indices = np.logical_and(np.isfinite(corrected), np.isfinite(reference))
corrected = corrected[valid_indices]
reference = reference[valid_indices]

corr = np.corrcoef(corrected, reference)[0, 1]
print(sys.argv[1], sys.argv[2])
print("correlation coefficient = ", corr)

if corr < 0:
    corrected *= -1.0
write_vector(sys.argv[3], corrected)
