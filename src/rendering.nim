
import terrain
import pixie
import config
import nimpy

type
    pixelfunction = proc(a: int, b: int, terrain: Terrain): Color {.noSideEffect.}

func heightmap*(x: int, y: int, terrain: Terrain): Color =
        var height: float = terrain.getNumber(x, y)
        color(height, height, height)

func fillImage(image: Image, f: pixelfunction, terrain: Terrain) =
    for y in 0 ..< image.height:
        # if(y mod 100) == 0: echo y
        
        for x in 0 ..< image.width:
            image.unsafe[x, y] = f(x, y, terrain).rgbx

proc renderTerrain(terrain: Terrain) = 
    var myimage = newImage(ROWS, COLUMNS)
    myimage.fillImage(heightmap, terrain)
    myimage.writeFile("image.png")

proc saveImageTest*() {.exportpy.} =
    echo "generating terrain..."
    var
        terrain: Terrain = brownianTerrain(SEED, ROWS, COLUMNS, OCTAVES, LACUNARITY, PERSISTENCE)
    echo "rendering terrain..."    
    renderTerrain(terrain)
