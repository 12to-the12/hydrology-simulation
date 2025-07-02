import noisy
import config
import std/math
import linalg


type Cell* = ref object
    height*: float
    temperature: float # K
    oxygen: float
    fixed_nitrogen: float
    humidity: float    # %
    wind: vector2d
    volume*: float
    volume_acc*: float
    impact*: float
    hydraulic_momentum_acc*: vector2d
    hydraulic_momentum*: vector2d



type Terrain* = ref object
    number: int
    width* = COLUMNS
    height* = ROWS
    rows: array[ROWS, array[COLUMNS, Cell]]

func get_cell*(self: Terrain, pos: vector2d): Cell =
    self.rows[(pos[1].int+self.height) mod self.height][(pos[0].int+self.width) mod self.width]

func get_cell*(self: Terrain, x: int, y: int): Cell =
    self.rows[(y+self.height) mod self.height][(x+self.width) mod self.width]
    # self.rows[y][x]

proc Δimpact*(self: Terrain, value: float, x: int, y: int) =
    self.get_cell(x, y).impact += value*MOMENTUM_FADE


# proc Δvolume*(self: Terrain, value: float, x: int, y: int) =
#     self.get_cell(x, y).volume += value*MOMENTUM_FADE

func simplexGrid(seed: int, frequency: float): Grid =
    var simplex = initSimplex(seed)
    simplex.frequency = frequency
    simplex.grid((0, 0), (COLUMNS, ROWS))

func computeMaps*(self: Terrain) =
    for x in 0 ..< COLUMNS:
        for y in 0 ..< ROWS:
            self.rows[y][x].volume = self.rows[y][
                    x].volume*(1-MOMENTUM_FADE)+self.rows[y][
                    x].volume_acc*MOMENTUM_FADE
            self.rows[y][x].volume_acc = 0.0


            self.rows[y][x].hydraulic_momentum = self.rows[y][
                    x].hydraulic_momentum*(1-MOMENTUM_FADE)+self.rows[y][
                    x].hydraulic_momentum_acc*MOMENTUM_FADE
            self.rows[y][x].hydraulic_momentum_acc = [0.0, 0.0]

func brownianTerrain*(seed: int, rows: int, columns: int, octaves: int,
        lacunarity: float, persistence: float): Terrain =
    type
        gridCollector = seq[Grid]
    var
        noise_passes: gridCollector
        total_amplitude: float = 0
        frequency: float
        amplitude = persistence.pow((octaves-1).float)
    for octave in 0 ..< octaves:
        frequency = (lacunarity^octave.float) / rows.float
        noise_passes.add(simplexGrid(seed, frequency))
        total_amplitude+=amplitude/persistence^octave


    var output = Terrain()



    for x in 0 ..< columns:
        for y in 0 ..< rows:
            var height: float = 0
            for octave in 0 ..< octaves:
                height += (noise_passes[octave][x, y]+1)/2*(
                        amplitude/persistence^octave)
            height/=total_amplitude
            height*=TERRAIN_HEIGHT
            # if height<500:height = 500


            height *= sin(x.float*3.14159/ROWS)*0.5+sin(
                    y.float*3.14159/COLUMNS)*0.5
            output.rows[y][x] = new Cell
            output.get_cell(x, y).height = height
            # output.setNumber(height, x, y)
    return output

# returns the a vector based on the heightmap at a particular cell
proc get_normal*(terrain: Terrain, x: int, y: int): vector3d =
    # top_left =
    let
        left = terrain.get_cell(x-1, y).height
        right = terrain.get_cell(x+1, y).height
        up = terrain.get_cell(x, y-1).height
        down = terrain.get_cell(x, y+1).height
        center = terrain.get_cell(x, y).height

        topleft = [-1.0, 0.0, left-center].cross([0.0, -1.0, up-center])
        topright = [0.0, -1.0, up-center].cross([1.0, 0.0, right-center])
        bottomleft = [0.0, 1.0, down-center].cross([-1.0, 0.0, left-center])
        bottomright = [1.0, 0.0, right-center].cross([0.0, 1.0, down-center])

        normal = topleft.unitize()+topright.unitize()+bottomleft.unitize()+bottomright.unitize()
    normal.unitize()

# NOT UNITIZED IN 2D
# derived from a unitized 3D vector
func get_normal_2d*(terrain: Terrain, x: int, y: int): vector2d =
    [terrain.get_normal(x, y)[0], terrain.get_normal(x, y)[1]]
    # [1.0,1.0]
