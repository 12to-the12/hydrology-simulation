
const
    SEED* = 2
    ROWS* = 512
    COLUMNS* = ROWS
    OCTAVES* = 16
    LACUNARITY*: float = 2
    PERSISTENCE*: float = 2.1 # ratio between big and small
    TERRAIN_HEIGHT* = ROWS
    PARTICLES* = 10_000_000
    # MAX_AGE* = 100
    COMPUTE_INTERVAL* = 10_000
    RENDER_INTERVAL* = 10_000
    MOMENTUM_FADE* = 0.2
    FRICTION* = 1
    MOMENTUM_SCALING* = 0.0

    DENSITY*: float = 1
    DEPOSITION_RATE* = 1e-1
    EVAPORATION_RATE* = 1e-3
    MIN_VOLUME* = 1e-2

    MAX_REPOSE* = 0.0