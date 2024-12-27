import numpy as np
a=np.array([ 3.2e+08,  1.0e+00, -1.0e+00,  8.0e+07])
b=np.array([ 4.0e+07,  1.0e+00, -1.0e+00, -1.6e+08])
print(f"{a} dot {b} is {np.dot(a,b)}")
c = np.array(a, dtype=np.int64)
d = np.array(b, dtype=np.int64)
print(f"{c} dot {d} is {np.dot(c,d)}")
