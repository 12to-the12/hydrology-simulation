print("<startup>")
import toml
import numpy as np
from terrain import Terrain
from hydrology import erode_terrain
# from benchmarking import benchmark

from timeit import timeit
import pickle

# print(arr)
def erosion():
    try:
        with open('cache/terrain.pkl','rb') as f:
            terrain = pickle.load(f)  
        print("cache hit, skipping generation")
    except:
        print("cache miss, generating noise map and saving to file...")
        terrain = Terrain()
        terrain.save_normalmaps()
        terrain.save_heightmap()
        print("terrain generated...")
        with open('cache/terrain.pkl','wb') as f:
            pickle.dump(terrain,f)  
        print("terrain cached to file")
    print(f"min height: {np.min(terrain._heightmap):.2f}")
    print(f"mean height: {np.mean(terrain._heightmap):.2f}")
    print(f"max height: {np.max(terrain._heightmap):.2f}")

    print(f"min normalmap: {np.min(terrain.magnormalmap):.2f}")
    print(f"mean normalmap: {np.mean(terrain.magnormalmap):.2f}")
    print(f"max normalmap: {np.max(terrain.magnormalmap):.2f}")

    # quit()

    terrain.save_normalmaps()
    print("eroding...")

    erode_terrain(terrain)
    terrain.save_heightmap()


def time_erosion():
    print(timeit(erosion, number=1))



def testnim():
    import nimporter
    import sim
    print("testing nim...")
    
    tt = sim.Terrain()
    tt.setNumber(12,2,2)
    print(tt.getNumber(2,2))
    print("nim tested!")
    tt = sim.brownianTerrain(12)
    sim.printHeightmap(tt)

    

if __name__ == "__main__":
    erosion()
    # testnim()
