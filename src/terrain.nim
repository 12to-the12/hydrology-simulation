import noisy
import config
import std/math
import linalg

type Cell = object
    height: float
    particledensity: float
    temperature: float # K
    oxygen: float
    fixed_nitrogen: float
    humidity: float # %
    wind: array[3, int]
    hydraulic_momentum: array[3, int]
    


type Terrain* = ref object
    number: int
    width* = COLUMNS
    height* = ROWS
    rows: array[ROWS, array[COLUMNS, Cell]]

proc paintposition*(self: Terrain, x: int, y: int) =
    self.rows[y][x].particledensity += 1

proc Δheight*(self: Terrain, value: float, x: int, y: int) =
    self.rows[y][x].height += value

proc setNumber*(self: Terrain, value: float, x: int, y: int) =
    self.rows[y][x].height = value

proc getNumber*(self: Terrain, x: int, y: int): float=
    self.rows[(y+self.height) mod self.height][(x+self.width) mod self.width].height

proc getParticleDensity*(self: Terrain, x: int, y: int): float=
    self.rows[(y+self.height) mod self.height][(x+self.width) mod self.width].particledensity


func simplexGrid(seed: int, frequency: float): Grid =
    var simplex = initSimplex(seed)
    simplex.frequency = frequency
    simplex.grid((0, 0), (COLUMNS, ROWS))


func brownianTerrain*(seed: int, rows: int, columns: int, octaves: int, lacunarity: float, persistence: float): Terrain =
    type
        gridCollector = seq[Grid]
    var
        noise_passes: gridCollector
        total_amplitude: float = 0
        frequency: float
        amplitude = persistence.pow((octaves-1).float)
    for octave in 0 ..< octaves:
        frequency = (lacunarity^octave.float) / rows.float
        noise_passes.add(simplexGrid(seed, frequency) )
        total_amplitude+=amplitude/persistence^octave


    var output = Terrain()
    
        

    for x in 0 ..< columns:
        for y in 0 ..< rows:
            var height: float = 0
            for octave in 0 ..< octaves:
                height += (noise_passes[octave][x, y]+1)/2*(amplitude/persistence^octave)
            height/=total_amplitude
            height*=TERRAIN_HEIGHT
            output.setNumber(height, x, y)
    return output

# returns the a vector based on the heightmap at a particular cell
proc get_normal*(terrain: Terrain, x:int, y:int): vector3d = 
    # top_left = 
    let
        left = terrain.getNumber(x-1,y)
        right = terrain.getNumber(x+1,y)
        up = terrain.getNumber(x,y-1)
        down = terrain.getNumber(x,y+1)
        center = terrain.getNumber(x,y)

        topleft = [-1.0,0.0,left-center].cross([0.0,-1.0,up-center])
        topright = [0.0,-1.0,up-center].cross([1.0,0.0,right-center])
        bottomleft = [0.0,1.0,down-center].cross([-1.0,0.0,left-center])
        bottomright = [1.0,0.0,right-center].cross([0.0,1.0,down-center])

        normal = topleft.unitize()+topright.unitize()+bottomleft.unitize()+bottomright.unitize()
    normal.unitize()

# NOT UNITIZED IN 2D
# derived from a unitized 3D vector
func get_normal_2d*(terrain: Terrain, x:int, y:int): vector2d = 
    [ terrain.get_normal(x,y)[0], terrain.get_normal(x,y)[1]]
    # [1.0,1.0]