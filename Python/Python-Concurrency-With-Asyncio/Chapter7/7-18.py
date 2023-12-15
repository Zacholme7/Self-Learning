# 7.18: Mean of a large matrix with NumPy
import numpy as np
import time

data_points = 40000000
rows = 50
columns = int(data_points / rows)

matrix = np.arange(data_points).reshape(rows, columns)

s = time.time()

res = np.mean(matrix, axis = 1)

e = time.time()

print(e - s)
