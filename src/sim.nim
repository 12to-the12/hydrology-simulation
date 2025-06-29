import nimpy
import rendering
import terrain
import config
import erode


import linalg


proc saveImageTest*() {.exportpy.} =
    echo "generating terrain..."
    var
        terrain: Terrain = brownianTerrain(SEED, ROWS, COLUMNS, OCTAVES, LACUNARITY, PERSISTENCE)
    echo "eroding terrain..."
    terrain.erode(PARTICLES)    
    # echo "rendering terrain..."    
    # renderTerrain(terrain)


proc Main() {.exportpy.} = 
    echo "running program..."
    saveImageTest()

when isMainModule:
    Main()