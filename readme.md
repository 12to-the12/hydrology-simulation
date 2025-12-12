see my worldsim note for details



- soil types
- ecology
- rain
- wind

# modelling the multithreaded hydrological erosion:
random location cannot be a terrain method, otherwise it'd mutate the state, the rng needs to come from somewhere else.

apparently send is a really costly operation that gets slower with more threads. Batch it.

writing to an array is O(n) with a really low coefficient