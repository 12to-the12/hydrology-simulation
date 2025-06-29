import noisy
import config
import std/math


type Cell = object
    height: float
    temperature: float # K
    momentum: array[3, int]
    

type Terrain* = ref object
    number: int
    rows: array[ROWS, array[COLUMNS, Cell]]

proc setNumber*(self: Terrain, value: float, x: int, y: int) =
    self.rows[y][x].height = value

proc getNumber*(self: Terrain, x: int, y: int): float=
    self.rows[y][x].height


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
            output.setNumber(height, x, y)
    return output
