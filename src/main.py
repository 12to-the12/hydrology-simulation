import toml
import numpy as np
from terrain import Terrain
from hydrology import erode_terrain



# print(arr)

if __name__=="__main__":
    terrain = Terrain()
    terrain.save_normalmaps()
    erode_terrain(terrain)
    terrain.save_map()



