from terrain import Terrain
from random import randint
import numpy as np
from time import sleep
from config import width, height, particles
import time
import terrain


def erode_terrain(terrain: Terrain, particles=particles, dt=0.1):
    # print(terrain.heightmap[terrain.get_random_position()])
    for i in range(particles):
        p = Particle(terrain, dt)
        # print("done")
        # quit()
        start = time.time()
        batch_size = 1_000
        if (i % batch_size) == 0:
            stamp = time.time()
            elapsed = stamp - start
            start = stamp
            print(f"{batch_size} particles in {elapsed:.2f} s")
            print(p)
            terrain.save_heightmap()
            
            # terrain.save_animation_frame(i / 5_000)
            # terrain.save_heightmap(path=f"maps/map{i/1000}.png")
            terrain.paths *= 0.5
            terrain.save_paths()
            print(f"\n{i}")
            terrain.save_trace()
            terrain.save_delta()
            # terrain.save_normalmaps()
            terrain.trace *= 0.9
            terrain.compute_momentum()
            terrain.save_momentum()
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
        self.friction = 0.8  # 1 for a second is 0
        self.evaporation_rate = 1e-3 #.05
        self.deposition_rate = 8 # 4
        self.age = 0
        self.steps = 0

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
        print(f"steps: {self.steps:.2f}")
        return ""

    @property
    def mass(self) -> float:
        return self.volume * self.density

    @property
    def speed(self) -> float:
        return float(np.linalg.norm(self.velocity))

    @property
    def momentum(self) -> float:
        return self.volume*self.velocity

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

            
            # if self.steps > 1000:print(self.steps)

            # we need to control the velocity to result in a position change of one

            # accelerate on surface
            self.direction_of_flow = self.terrain.normal_2D(
                self.position

                
            )  # get the surface normal in 3D
            try:
                self.dt = 1 / self.speed
            except:
                self.dt = 1

            # a friction of 1 removes all velocity
            self.velocity *= (1-self.friction)**self.dt
            self.velocity += self.direction_of_flow
            # print(f"\n\nspeed before: {np.linalg.norm(self.velocity)}")
            # normal_2D[1]  = 0
            # self.dt = dt

            # self.velocity += dt*direction_of_flow/self.mass
            # self.velocity += self.dt * direction_of_flow / self.mass
            # self.velocity += self.direction_of_flow / self.mass
            
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
            self.init_height = self.terrain.get_height(self.init_position)
            self.height = self.terrain.get_height(self.position)
            # meters
            self.Δ_height = self.init_height - self.height
            # potential_energy = self.mass*self.Δ_height*9.8
            # kinetic_energy = 0.5*self.mass*self.speed**2
            # new_kinetic_energy = potential_energy+kinetic_energy
            # new_speed = new_kinetic_energy*2/self.mass**0.5
            
            # self.velocity += self.direction_of_flow*new_speed
            # if self.velocity.all()==0:self.velocity += self.direction_of_flow
            # print(self.speed)

            # self.position[0] %= width
            # print(self.position[0])
            # print(f"{self.sediment=}")
            # self.position[1] %= height
            assert 0 <= self.x < width, self.x
            assert 0 <= self.y < height, self.y



            # slow down
            # self.velocity *= 0


            self.erode_dumb()

            # evaporate
            # self.volume *= 0.5
            self.age += self.dt
            self.steps += 1
            self.volume *= (1 - self.evaporation_rate) ** self.dt

    def write(self):
        x_momentum,y_momentum = self.momentum
        self.terrain.momentum_track[int(self.y), int(self.x)][0] += x_momentum
        self.terrain.momentum_track[int(self.y), int(self.x)][1] += y_momentum



        self.terrain.paths[int(self.y), int(self.x)] = np.array(
            [self.volume_left * 10, 0, (1 - self.volume_left) * 10]
        )
        # self.terrain.trace[int(self.position[1]),int(self.position[0])]+=np.array([self.volume_left*10,0,(1-self.volume_left)*10])

        self.terrain.trace[int(self.y), int(self.x)] += np.array([1, 0, 0])
        # self.save()

    def save(self):
        self.terrain.save_trace()
        # self.terrain.save_delta()
        # self.terrain.save_normalmaps()
        # self.terrain.save_heightmap()

    def erode_dumb(self):
        # pass
        
        if self.Δ_height > 0:
            self.terrain.change_height(x=self.init_x, y=self.init_y, value=-self.Δ_height / 2)
            self.terrain.change_height(x=self.x, y=self.y, value=self.Δ_height / 2)
        elif self.Δ_height < 0:
            self.flowing = False
        
        else:
            pass
            # self.flowing = False
        

        self.write()
        # print(".")

    def erode_smart(self):
        # mass transfer

        # kilograms/meter**3
        carrying_capacity = self.volume * self.speed * self.Δ_height

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
        self.terrain.change_height(x=self.x, y=self.y, value=-deposited)

        self.write()
