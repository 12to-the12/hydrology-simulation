
import terrain
import pixie
import config
import nimpy
import linalg

type
    pixelfunction = proc(a: int, b: int,
            terrain: Terrain): Color {.noSideEffect.}

func normals*(x: int, y: int, terrain: Terrain): Color =
        let normal: vector3d = terrain.get_normal(x, y)
        var
            x = normal.x
            y = normal.y
            z = normal.z
            ix = -normal.x
        if normal.x < 0: x = 0.0
        if normal.x > 0: ix = 0.0
        if normal.y < 0: y = 0.0
        if normal.z < 0: z = 0.0
        color(x, 0.0, ix)
        # color(0.5,0.0,0.0)

func particledensity*(x: int, y: int, terrain: Terrain): Color =
    var particledensity: float = terrain.getParticleDensity(x,y)
    color(particledensity, particledensity, particledensity)

func heightmap*(x: int, y: int, terrain: Terrain): Color =
    var height: float = terrain.getNumber(x, y)/TERRAIN_HEIGHT
    color(height, height, height)

proc fillImage(image: Image, f: pixelfunction, terrain: Terrain) =
    for y in 0 ..< image.height:
        # if(y mod 100) == 0: echo y

        for x in 0 ..< image.width:
            # echo terrain.get_normal(x, y).y
            image.unsafe[x, y] = f(x, y, terrain).rgbx

proc renderTerrain*(terrain: Terrain) =
    var myimage = newImage(ROWS, COLUMNS)
    myimage.fillImage(normals, terrain)
    myimage.writeFile("normals.png")
    myimage.fillImage(heightmap, terrain)
    myimage.writeFile("heightmap.png")
    myimage.fillImage(particledensity, terrain)
    myimage.writeFile("momentum.png")

