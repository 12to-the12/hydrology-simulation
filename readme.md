see my worldsim note for details



- soil types
- ecology
- rain
- wind

# modelling the multithreaded hydrological erosion:
random location cannot be a terrain method, otherwise it'd mutate the state, the rng needs to come from somewhere else.

apparently send is a really costly operation that gets slower with more threads. Batch it.

writing to an array is O(n) with a really low coefficient



1400 ms to process 1_000_000 raindrops on 10 threads.
700 ms to do the math, and another 700 to simply write to terrain.
Can't figure out how to write as soon as the value is created.