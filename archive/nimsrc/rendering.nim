
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

func pretty*(x: int, y: int, terrain: Terrain): Color =
    var height: float = terrain.get_cell(x,y).height
    var volume: float = terrain.get_cell(x,y).volume
    var slope: float = terrain.get_normal_2d(x,y).magnitude
    if height<100: return color(0.0, 0.0, 1.0)
    # if height>600: return color(1.0, 1.0, 1.0)
    # if volume>50: return color(0.1, 0.1, volume/100)
    if slope>0.9: return color(slope, slope, slope)
    color(0.0, 1-volume/100, volume/100)
    # color(mx, 0.0, -mx)

func atmospheric_water*(x: int, y: int, terrain: Terrain): Color =
    var atmospheric_water: float = terrain.get_cell(x,y).atmospheric_water
    color(atmospheric_water, atmospheric_water, atmospheric_water)
    # color(mx, 0.0, -mx)
func impact*(x: int, y: int, terrain: Terrain): Color =
    var impact: float = terrain.get_cell(x,y).impact
    color(0.0, 0.0, impact)
    # color(mx, 0.0, -mx)

func volume*(x: int, y: int, terrain: Terrain): Color =
    var volume: float = terrain.get_cell(x,y).volume
    color(volume/1e2, volume/1e2, volume/1e0)
    # color(mx, 0.0, -mx)

func momentum*(x: int, y: int, terrain: Terrain): Color =
    var momentum: float = terrain.get_cell(x,y).hydraulic_momentum.magnitude
    color(momentum/1e2, momentum/1e2, momentum/1e0)
    # color(mx, 0.0, -mx)

func heightmap*(x: int, y: int, terrain: Terrain): Color =
    var height: float = terrain.get_cell(x,y).height/TERRAIN_HEIGHT
    color(height, height, height)

proc fillImage(image: Image, f: pixelfunction, terrain: Terrain) =
    for y in 0 ..< image.height:
        # if(y mod 100) == 0: echo y

        for x in 0 ..< image.width:
            # echo terrain.get_normal(x, y).y
            image.unsafe[x, y] = f(x, y, terrain).rgbx

proc renderTerrain*(terrain: Terrain) =
    var myimage = newImage(ROWS, COLUMNS)
    myimage.fillImage(pretty, terrain)
    myimage.writeFile("pictures/pretty.png")
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
    myimage.fillImage(atmospheric_water, terrain)
    myimage.writeFile("pictures/atmospheric_water.png")

