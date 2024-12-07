from terrain import Terrain
from random import randint
import numpy as np
from time import sleep
from config import width, height

import terrain
def erode_terrain(terrain: Terrain, particles=1000, dt=1e-2):

    # print(terrain.heightmap[terrain.get_random_position()])
    for _ in range(particles):
        Particle(terrain, dt)
    # terrain.save_map()
    terrain.save_write()

    # sleep(0.5)


class Particle:
    def __init__(self, terrain: Terrain, dt: float) -> None:
        self.terrain = terrain
        self.velocity = np.array([0.,0.],dtype=np.float64) # velocity is distance per time
        self.position = self.terrain.get_random_position().astype(np.float64)
        self.volume = 1

        self.sediment = 0

        self.density = 1e-1
        friction = 0.9
        evaporation_rate = 0.1
        deposition_rate = 0.1



        self.drop(dt, friction, evaporation_rate, deposition_rate)

    @property
    def mass(self) -> float:
        return self.volume*self.density

    def drop(self, dt: float, friction, evaporation_rate, deposition_rate):
        while True:
            print(".")
            if self.volume<=0.01:
                break
            # accelerate on surface
            normal_2D = self.terrain.normal_2D(self.position) # get the surface normal in 3D
            # print(f"\n\nspeed before: {np.linalg.norm(self.velocity)}")
            # normal_2D[1]  = 0
            self.velocity += dt*normal_2D/self.mass
            # print(f"{normal_2D}")
            # print(f"{dt}")
            # print(f"{self.mass}")


            # print(f"speed: {np.linalg.norm(self.velocity)}")
            # print(f"position: {self.position}")


            # move
            # self.init_position = self.position.copy()
            self.position+=self.velocity
            self.position%=np.array([width, height])
            # self.init_height = self.terrain.heightmap[self.init_position[1],self.init_position[0]]
            # self.height = self.terrain.heightmap[self.position[1],self.position[0]]

            # slow down
            self.velocity *=0

            # self.velocity *= 1-(dt*friction)
            # if self.init_height>self.height:
            #     print(f"dropped {self.init_height-self.height} meters")
            # else:
            #     print(f"gained {self.height-self.init_height} meters")
            


            # mass transfer

            # sediment_equilibrium = 0.5*self.volume*np.linalg.norm(self.velocity)*(self.init_height-self.height)
            # soaking/deposition force
            # cdiff = sediment_equilibrium-self.sediment
            # print(f"{cdiff=}")
            # self.sediment += dt*deposition_rate*cdiff
            # print(dt*deposition_rate*cdiff)
            # self.terrain.heightmap[self.init_position[1],self.init_position[0]] -= dt*self.volume*deposition_rate*cdiff


            # prinout
            # print()
            # print(f"{normal_2D=}")
            # print(f"{self.position=}")
            # print(f"{self.velocity=}")
            self.terrain.write[int(self.position[1]),int(self.position[0])]=1

            # evaporate
            self.volume *= 1-evaporation_rate

            # self.terrain.save_map()
            # sleep(0.5)
        # self.terrain.heightmap[self.position[1],self.position[0]]*=0.1
