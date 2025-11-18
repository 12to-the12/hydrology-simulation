import nimpy
import rendering
import terrain
import config
import erode


import linalg
import vmath

proc Main() {.exportpy.} = 
    echo "running program..."
    var
        terrain: Terrain = brownianTerrain(SEED, ROWS, COLUMNS, OCTAVES, LACUNARITY, PERSISTENCE)
    
    for iteration in 0..<10_000:

        # terrain.advance_climate()
        terrain.hydrological_erosion(PARTICLES)


when isMainModule:
    # var a = vec2(1.0, 1.0)
    # a=a+2
    # echo a*2

    Main()