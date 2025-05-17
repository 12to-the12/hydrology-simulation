import time
import nimporter
import sim

from time import perf_counter, perf_counter_ns


def fib(n):
    if n == 0:
        return 0
    elif n < 3:
        return 1
    return fib(n - 1) + fib(n - 2)
    
def time_difference(function):
    start = perf_counter_ns()
    for i in range(0,10):print(function(i))
    end = perf_counter_ns()
    elapsed_ns = end-start
    elapsed_ms = elapsed_ns*1e-6
    print(f"Elapsed: {elapsed_ms:.2f}ms")
    return elapsed_ms

def benchmark():
    a=0
    b=0
    for j in range(10):
        a += time_difference(fib)
    for j in range(10):
        b+= time_difference(sim.fib)
    # a = time_difference(fib)
    # b = time_difference(sim.fib)
    print(f"Ratio: {a/b:.2f}x")
    