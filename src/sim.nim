import nimpy

proc fib(n: int): int {.exportpy.} =
    if n == 0:
        return 0
    elif n < 3:
        return 1
    return fib(n-1) + fib(n-2)

const ROWS = 4
const COLUMNS = 4

type Location = object
    height: int

type Terrain = ref object of PyNimObjectExperimental
    number: int
    rows: array[ROWS, array[COLUMNS, Location]]

proc setNumber(self: Terrain, value: int, x: int, y: int) {.exportpy.} =
    self.rows[y][x].height = value
    echo self.rows

proc getNumber(self: Terrain, x: int, y: int): int {.exportpy.} =
 