import sys
import numpy as np
import cvxpy as cp

# 读入
nums = np.fromiter((int(v) for v in sys.stdin.read().split()), dtype=np.int32)
m, n = nums[:2]
N = m * n + 2
c = nums[2:N].reshape(m, n)
b = nums[N:N+n]
a = nums[-m:]

# CVXPY
x = cp.Variable((m, n), nonneg=True)
obj = cp.Minimize(cp.sum(cp.multiply(c, x)))
source_cons = (cp.sum(x[src,:]) <= a[src] for src in range(m))
dest_cons = (cp.sum(x[:, dst]) >= b[dst] for dst in range(n))
problem = cp.Problem(obj, [*source_cons, *dest_cons])
print(f"{problem.solve():.2f}")
