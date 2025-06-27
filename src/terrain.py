
from config import config, height, width, noise_detail, persistence, lacunarity, seed
import numpy as np
import opensimplex # type: ignore
from random import randint
import numpy.typing as npt
from soil import Soil
from time import sleep

class Terrain:
    def __init__(self) -> None:
        self.paths = np.zeros((height,width,3))
        self.trace = np.zeros((height,width,3))
        self.momentum = np.zeros((height,width,2))
        self.momentum_track = np.zeros((height,width,2))

        self.base = 0
        self.zenith = max(width, height)/2
        self.initialize_to_noise()
        self.soil = Soil(1)

    def initialize_to_noise(self):
        self._heightmap = np.zeros((height,width))
        self.initialize_brownian(noise_detail, persistence=persistence,lacunarity=lacunarity,seed=seed)

    # persistence is how overwhelming the lower frequencies are. A value of 1 treats high frequencies well
    # lacunarity is how quickly frequencies increase
    def initialize_brownian(self, octaves, persistence=2, lacunarity=2, seed=None):
        if seed: opensimplex.seed(seed)
        else: opensimplex.random_seed()

        frequency = 1
        amplitude = persistence**(octaves-1)
        total_amplitude = 0
        for octave in range(octaves):
            print(f"{octave=}")
      
            # frequency = 2**octave
            # amplitude = 2**(octaves-octave-1)
            # print(frequency)
            # print(amplitude)
            self.add_noise_pass(frequency,amplitude,seed=seed)
            total_amplitude+=amplitude
            frequency*=lacunarity
            amplitude/=persistence
        self._heightmap/=total_amplitude


        self._heightmap-=self._heightmap.min()
        self._heightmap/=self._heightmap.max()
        self._heightmap*=self.zenith
        self._heightmap+=self.base

        self.initial_heightmap = self._heightmap.copy()


        
    def add_noise_pass(self, frequency=1, amplitude=1, seed=None):
        if seed:
            next_seed = (opensimplex.get_seed()%1_000_000)*977%1_000_000
            next_seed = int(next_seed)
            opensimplex.seed(next_seed)
        else:
            opensimplex.random_seed()
        # print(opensimplex.get_seed())


        # arr = np.zeros((height,width))
        x_coords = np.linspace(0,frequency,width)
        y_coords = np.linspace(0,frequency,height)
        arr = opensimplex.noise2array(x_coords,y_coords)

        arr+=1
        arr/=2

        self._heightmap += arr*amplitude

        self.save_heightmap()

    def get_random_position(self):
        y_bound, x_bound = self._heightmap.shape
        x = randint(0,x_bound-1)
        y = randint(0,y_bound-1)
        return np.array([x,y],dtype=np.int64)

    # momenutm is computed as a weighted sum of accumulated and existing
    def compute_momentum(self):
        rate = 0.2 # replacement rate
        # tmp = self.momentum.copy()
        self.momentum = (1-rate)*self.momentum + rate*self.momentum_track
        self.momentum_track = np.zeros((height,width,2))

    # returns a 3D array from a point
    # samples the immediately left, right, up, and down heights to find the normal vector
    def normal(self, position: npt.NDArray) -> np.ndarray:
        position = position.astype(np.int64)
        assert position.dtype == np.int64
        x,y = position
        x%=width
        y%=height
        # print(f"{x=},{y=}")
        # print(f"{width=},{height=}")
        # print(f"{x%width},{(y-1)%height}")

        left = self._heightmap[y,(x-1)%width]
        right = self._heightmap[y,(x+1)%width]
        # print(x)
        # print((y-1)%height)
        up = self._heightmap[(y-1)%height,x]

        down = self._heightmap[(y+1)%height,x]

        # print(f"{left=}")
        # print(f"{right=}")
        # print(f"{up=}")
        # print(f"{down=}")

        x_pitch = (left-right)/2
        y_pitch = (up-down)/2
        normal_vector = np.array([x_pitch, y_pitch, 1],dtype=np.float64)
        # normalizes it by dividing by magnitude
        normal_vector/=np.linalg.norm(normal_vector)
        # normal_vector = np.array([x_pitch, y_pitch])
        # print(f"{normal_vector=}")



        return normal_vector
    # [0] is x, [1] is y
    def save_paths(self):
        self.save_map(map=self.paths,path="./pictures/paths.png",colorize=False)

    def save_trace(self,path="./pictures/trace.png"):
        self.save_map(map=self.trace,path=path,colorize=False)
    
    def save_heightmap(self,path="./pictures/heightmap.png"):
            self.save_map(self._heightmap,path=path)
    
    def save_momentum(self,path="./pictures/momentum.png"):
            self.save_map(self.momentum,path=path,vector=True)

    def save_animation_frame(self,number):
        self.save_heightmap(path=f"./animation/heightmap-animation/{number:.0f}-heightmap.png")
        self.save_trace(path=f"./animation/trace-animation/{number:.0f}-heightmap.png")
    def save_delta(self,path="./pictures/delta.png"):
            delta = self._heightmap-self.initial_heightmap
            # print(np.max(self._heightmap))
            # print(np.max(self.initial_heightmap))
            
            # print(np.min(self._heightmap))
            # print(np.min(self.initial_heightmap))
            
            # print(f"delta max:\t{np.max(delta):.2f}")
            # print(f"delta min:\t{np.min(delta):.2f}")
            # print(f"delta mean:\t{np.mean(delta):.2f}")

            # volume = np.sum(self._heightmap)
            # print(f"total volume: {volume:.2f}")

            # sleep(3)
            # quit()
            # delta-=np.min(delta)


            # span = np.max(delta)-np.min(delta)
            # delta/=span
            self.save_map(map=delta,path=path,colorize=True)


    def normal_2D(self, position: np.ndarray) -> np.ndarray:
        normal_vector = self.normal(position)
        assert normal_vector.shape == (3,)

        normal_2D = normal_vector[:2]
        assert normal_2D.shape == (2,)
        return normal_2D
    
    def save_normalmaps(self):
        self.xnormalmap = np.zeros((height,width))
        self.ynormalmap = np.zeros((height,width))
        self.magnormalmap = np.zeros((height,width))


        for y in range(height):
            for x in range(width):
                # self.xnormalmap[y,x] = x
                xval, yval = list(self.normal_2D(np.array([x,y])))

                self.xnormalmap[y,x] = xval
                self.ynormalmap[y,x] = yval
                self.magnormalmap[y,x] = np.linalg.norm(self.normal_2D(np.array([x,y])))
        
        self.save_map(map=self.xnormalmap,path="./pictures/x.png",colorize=True)
        self.save_map(map=self.ynormalmap,path="./pictures/y.png",colorize=True)
        self.save_map(map=self.magnormalmap,path="./pictures/mag.png",colorize=True)

    def get_height(self, position: npt.NDArray):
        x = int(position[0])
        y = int(position[1])
        return self._heightmap[y,x]
    
    def set_height(self,x=0,y=0,value=0):

        self._heightmap[int(y),int(x)] = value
    
    
    def change_height(self,x=0,y=0,value=0):
        self._heightmap[int(y),int(x)] += value   

    def save_map(self, map, path="./pictures/image.png",colorize=False,vector=False):

            arr = map.copy()
            
            if vector:
                assert arr.shape[-1] ==2
                x_arr = arr[:,:,0]
                y_arr = arr[:,:,1]
                mag = np.sqrt(x_arr**2+y_arr**2)
                self.save_map(mag,path=path)
                return None

            elif colorize: 
                assert arr.shape[-1] !=3
                negative_arr = map.copy()
                arr = arr.clip(min=0)
                
                negative_arr*=-1
                negative_arr = negative_arr.clip(min=0)

                arr/=arr.max()
                negative_arr/=negative_arr.max()



                arr = arr.reshape(height,width,1)
                negative_arr =  negative_arr.reshape(height,width,1)
                zeros = np.zeros_like(arr)
                arr = np.concatenate((negative_arr,zeros,arr),axis=2)
                arr = arr.reshape(height,width,3)
                if path=="./pictures/delta.png":
                    pass
                    # print(arr.shape)
                    # print(np.max(arr))
                    # print(np.min(arr))
                    # print(np.mean(arr))
                    # sleep(3)


            else:
                if arr.shape[-1]==1:
                    arr = np.repeat(arr,3,axis=-1)
                # std_dev = arr.std()
                # rng = 1 # standard deviations to show on each side

                
                # arr+=std_dev*rng
                start = np.percentile(arr,5)
                end = np.percentile(arr,95)
                arr-=start
                arr/=end
                arr = np.clip(arr,0,1)
                
                


            



            arr*=255
            arr = arr.astype(np.uint8)

            # save out
            from PIL import Image
            im = Image.fromarray(arr)
            im.save(path)


# def test_terrain():
#     terrain = Terrain()
#     position = np.array([5,5])
#     normal_2D = terrain.normal_2D(position)
#     assert normal_2D is 5, f"{normal_2D=}"


