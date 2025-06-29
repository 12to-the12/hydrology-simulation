import nimpy
import std/math
import pixie
import rendering
import terrain
# import erode

proc fib(n: int): int  =
    if n == 0:
        return 0
    elif n < 3:
        return 1
    return fib(n-1) + fib(n-2)


# func height(terrain: Terrain) = 
#     terrain.rows.len

# func width(terrain: Terrain) = 
#     terrain.rows[0].len



# const chars = "-~+=%"

# proc printHeightmap(terrain: Terrain) {.exportpy.} =
#     let gradations: int = chars.len
#     for x in 0 ..< COLUMNS:
#         for y in 0 ..< ROWS:
#             let
#                 value: float = terrain.getNumber(x, y)
#                 index = (value*gradations.float).floor.int
#                 character = chars[index]
#             stdout.write character
#             # stdout.write character
#             stdout.write ' '

#         echo ""





proc Main() {.exportpy.} = 
    saveImageTest()