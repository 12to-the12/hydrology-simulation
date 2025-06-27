import nimpy
import std/math
proc fib(n: int): int {.exportpy.} =
    if n == 0:
        return 0
    elif n < 3:
        return 1
    return fib(n-1) + fib(n-2)

const
    ROWS = 16
    COLUMNS = 16
    ITERATIONS = 1
    LACUNARITY = 0.5

type Location = object
    height: float

type Terrain = ref object of PyNimObjectExperimental
    number: int
    rows: array[ROWS, array[COLUMNS, Location]]

proc setNumber(self: Terrain, value: float, x: int, y: int) {.exportpy.} =
    self.rows[y][x].height = value

proc getNumber(self: Terrain, x: int, y: int): float {.exportpy.} =
    self.rows[y][x].height

import noisy

func simplexGrid(seed: int, frequency:float): Grid = 
    var simplex = initSimplex(seed)
    simplex.frequency = frequency
    simplex.grid((0,0), (COLUMNS,ROWS))


func brownianTerrain(seed: int): Terrain {.exportpy.} = 
    type
        gridCollector = array[0..ITERATIONS, Grid]
    var passes: gridCollector
    for pass in 0 ..< ITERATIONS:
        passes[pass] = simplexGrid(seed, LACUNARITY^pass.float )

    var output = Terrain()
    
    for x in 0 ..< COLUMNS:
        for y in 0 ..< ROWS:
            var value: float = 0
            for pass in 0 ..< ITERATIONS:
                value += (passes[pass][x,y]+1)/2
            value /= ITERATIONS
            output.setNumber(value,x,y)
    return output

const chars = "-~+=%"

proc printHeightmap(terrain: Terrain) {.exportpy.} =
    let gradations: int = chars.len
    for x in 0 ..< COLUMNS:
        for y in 0 ..< ROWS:
            let
                value: float =  terrain.getNumber(x,y)
                index = (value*gradations.float).floor.int
                character = chars[index]
            stdout.write character
            # stdout.write character
            stdout.write ' '

        echo ""