print("<startup>")
import toml
import numpy as np
from terrain import Terrain
from hydrology import erode_terrain
# from benchmarking import benchmark
# from testingnim import testnim

from timeit import timeit


# print(arr)
def erosion():
    terrain = Terrain()
    terrain.save_normalmaps()
    terrain.save_heightmap()
    print(f"min height: {np.min(terrain._heightmap):.2f}")
    print(f"mean height: {np.mean(terrain._heightmap):.2f}")
    print(f"max height: {np.max(terrain._heightmap):.2f}")

    print(f"min normalmap: {np.min(terrain.magnormalmap):.2f}")
    print(f"mean normalmap: {np.mean(terrain.magnormalmap):.2f}")
    print(f"max normalmap: {np.max(terrain.magnormalmap):.2f}")

    # quit()

    print("eroding...")

    erode_terrain(terrain)
    terrain.save_heightmap()


def time_erosion():
    print(timeit(erosion, number=1))


if __name__ == "__main__":
    erosion()
    # testnim()
