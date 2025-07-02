
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

func impact*(x: int, y: int, terrain: Terrain): Color =
    var impact: float = terrain.getimpact(x,y)
    color(0.0, 0.0, impact)
    # color(mx, 0.0, -mx)

func volume*(x: int, y: int, terrain: Terrain): Color =
    var volume: float = terrain.getvolume(x,y)
    color(0.0, 0.0, volume/1e2)
    # color(mx, 0.0, -mx)

func momentum*(x: int, y: int, terrain: Terrain): Color =
    var mx: float = terrain.getMomentum(x,y)[0]
    var my: float = terrain.getMomentum(x,y)[1]
    var momentum: float = terrain.getMomentum(x,y).magnitude
    color(momentum/1e2, momentum/1e2, momentum/1e0)
    # color(mx, 0.0, -mx)

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
    myimage.writeFile("pictures/normals.png")
    myimage.fillImage(heightmap, terrain)
    myimage.writeFile("pictures/heightmap.png")
    myimage.fillImage(momentum, terrain)
    myimage.writeFile("pictures/momentum.png")
    myimage.fillImage(volume, terrain)
    myimage.writeFile("pictures/volume.png")
    myimage.fillImage(impact, terrain)
    myimage.writeFile("pictures/impact.png")

