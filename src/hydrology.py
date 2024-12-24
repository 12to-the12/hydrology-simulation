from terrain import Terrain
from random import randint
import numpy as np
from time import sleep
from config import width, height

import terrain


def erode_terrain(terrain: Terrain, particles=50_000_000, dt=0.1):
    # print(terrain.heightmap[terrain.get_random_position()])
    for i in range(particles):
        p = Particle(terrain, dt)
        # print("done")
        # quit()
        if (i % 5000) == 0:
            print(p)
            terrain.save_heightmap()
            # terrain.save_heightmap(path=f"maps/map{i/1000}.png")
            terrain.paths *= 0.5
            terrain.save_paths()
            print(f"\n{i}")
            # print(f"{p.carrying_capacity}")
            # terrain.trace/=i
            # terrain.trace*=1_000
            terrain.save_trace()
            terrain.save_delta()
            terrain.save_normalmaps()
            # terrain.trace/=1_000
            # terrain.trace*=i
            terrain.trace *= 0.9
    # terrain.trace/=particles
    # terrain.trace*=1_000
    # terrain.save_trace()
    # terrain.save_heightmap()
    # terrain.save_paths()



class Particle:
    def __init__(self, terrain: Terrain, dt: float) -> None:
        self.terrain = terrain
        self.velocity = np.array(
            [0.0, 0.0], dtype=np.float64
        )  # velocity is distance per time
        self.position = self.terrain.get_random_position().astype(np.float64)
        self.volume = 10
        self.initial_volume = self.volume

        # kg/m**3
        self.sediment = 0  # sediment concentration

        self.density = 1
        self.friction = 0.1  # 1 for a second is 0
        self.evaporation_rate = 0.05
        self.deposition_rate = 4
        self.age = 0
        
        self.drop()

    def __repr__(self):
            print(f"direction: {self.direction_of_flow}")
            print(f"velocity: {self.velocity}")
            print(f"speed: {self.speed:.2f}")

            print(f"position: {self.position}")
            print(f"interval: {self.dt:.2f}")
            # print(f"ideal travel: {self.dt*self.speed:.2f}")
            print(f"volume: {self.volume:.2f}")
            print(f"volume factor: {(1-self.evaporation_rate)**self.dt:.2f}")
            print(f"age: {self.age:.2f}")
            return ""
    @property
    def mass(self) -> float:
        return self.volume * self.density

    @property
    def speed(self) -> float:
        return float(np.linalg.norm(self.velocity))

    @property
    def x(self):
        return self.position[0]

    @property
    def y(self):
        return self.position[1]
    
    @property
    def init_x(self):
        return self.init_position[0]

    @property
    def init_y(self):
        return self.init_position[1]

    @property
    # 0->1 of volume left
    def volume_left(self):
        return self.volume / self.initial_volume


    def drop(self):
        self.flowing = True
        while self.flowing:
            # print("\n\n.",end="")
            # print(".")
            if self.volume <= 0.01:
                
                break

            # we need to control the velocity to result in a position change of one

            # accelerate on surface
            self.direction_of_flow = self.terrain.normal_2D(
                self.position
            )  # get the surface normal in 3D
            # print(f"\n\nspeed before: {np.linalg.norm(self.velocity)}")
            # normal_2D[1]  = 0
            # self.dt = dt


            # self.velocity += dt*direction_of_flow/self.mass
            # self.velocity += self.dt * direction_of_flow / self.mass
            # self.velocity += self.direction_of_flow / self.mass
            self.velocity += self.direction_of_flow

            # print(self.speed)
            try:self.dt = 1/self.speed
            except:self.dt = 1
            # print(self.speed)
            # quit()

            




            # print(f"{normal_2D}")
            # print(f"{dt}")
            # print(f"{self.mass}")

            # print(f"speed: {np.linalg.norm(self.velocity)}")
            # print(f"position: {self.position}")

            # move
            self.init_position = self.position.copy()
            self.position += self.velocity * self.dt
            # print(self.position[0])

            if not (0 < self.x < width):
                break
            if not (0 < self.y < height):
                break
            # self.position[0] %= width
            # print(self.position[0])
            # print(f"{self.sediment=}")
            # self.position[1] %= height
            assert 0 <= self.x < width, self.x
            assert 0 <= self.y < height, self.y

            self.init_height = self.terrain.get_height(self.init_position)
            self.height = self.terrain.get_height(self.position)

            # slow down
            self.velocity *=0

            # a friction of 1 removes all velocity
            # self.velocity *= 1 - (self.dt * self.friction)

            self.erode()

            # evaporate
            # self.volume *= 0.5
            self.age += self.dt
            self.volume *= (1 - self.evaporation_rate)**self.dt
    def write(self):
        self.terrain.paths[int(self.y), int(self.x)] = np.array(
        [self.volume_left * 10, 0, (1 - self.volume_left) * 10]
        )
        # self.terrain.trace[int(self.position[1]),int(self.position[0])]+=np.array([self.volume_left*10,0,(1-self.volume_left)*10])
        
        self.terrain.trace[int(self.y), int(self.x)] += np.array(
            [1, 0, 0]
        )
        # self.save()

    def save(self):
            self.terrain.save_trace()
            # self.terrain.save_delta()
            # self.terrain.save_normalmaps()
            # self.terrain.save_heightmap()
    def erode(self):
        # pass
        drop = self.init_height - self.height
        if (drop>0):
            self.terrain.change_height(x=self.init_x,y=self.init_y,value=-drop/2)
            self.terrain.change_height(x=self.x,y=self.y,value=drop/2)
        elif drop<0:
            self.flowing = False
        
        

        self.write()
        # print(".")

    def erode_(self):
        # mass transfer

        # meters
        drop = self.init_height - self.height

        # kilograms/meter**3
        carrying_capacity = self.volume * self.speed * drop

        if carrying_capacity < 0:
            carrying_capacity = 0

        # soaking/deposition force
        # positive means that there's a lot of soaking, negative means a large deposition force
        # concentration differential, dimensionless
        gradient = carrying_capacity - self.sediment

        # time*(deposition_rate)*((volume*speed*drop)-sediment)
        # can't be larger than drop
        # mass/volume. Not mass
        picked_up = self.dt * self.deposition_rate * gradient

        self.sediment += picked_up  # sediment concentration

        # time*volume*rate*dist
        # gives mass deposited
        deposited = self.dt * self.volume * self.deposition_rate * gradient

        # self.terrain._heightmap[
        #     int(self.init_position[1]), int(self.init_position[0])
        # ] -= deposited
        self.terrain.change_height(x=self.x,y=self.y,value=-deposited)


