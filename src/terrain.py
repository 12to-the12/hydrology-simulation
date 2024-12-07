
from config import config, height, width, noise_detail, persistence, lacunarity, seed
import numpy as np
import opensimplex # type: ignore
from random import randint
import numpy.typing as npt


class Terrain:
    def __init__(self) -> None:
        self.write = np.zeros((height,width))
        self.initialize_to_noise()

    def initialize_to_noise(self):
        self.heightmap = np.zeros((height,width))
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
        self.heightmap/=total_amplitude

        base = 0
        top = 50
        self.heightmap-=self.heightmap.min()
        self.heightmap/=self.heightmap.max()
        self.heightmap*=top
        self.heightmap+=base

        
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

        self.heightmap += arr*amplitude
    def get_random_position(self):
        y_bound, x_bound = self.heightmap.shape
        x = randint(0,x_bound-1)
        y = randint(0,y_bound-1)
        return np.array([x,y],dtype=np.int64)

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

        left = self.heightmap[y,(x-1)%width]
        right = self.heightmap[y,(x+1)%width]
        # print(x)
        # print((y-1)%height)
        up = self.heightmap[(y-1)%height,x]

        down = self.heightmap[(y+1)%height,x]

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
    def save_write(self):
        self.save_map(map=self.write,path="write.png")
    def normal_2D(self, position: np.ndarray) -> np.ndarray:
        normal_vector = self.normal(position)
        assert normal_vector.shape == (3,)

        normal_2D = normal_vector[:2]
        assert normal_2D.shape == (2,)
        return normal_2D
    
    def save_normalmaps(self):
        self.xnormalmap = np.zeros((height,width))
        self.ynormalmap = np.zeros((height,width))


        for y in range(height):
            for x in range(width):
                # self.xnormalmap[y,x] = x
                xval, yval = list(self.normal_2D(np.array([x,y])))

                self.xnormalmap[y,x] = xval
                self.ynormalmap[y,x] = yval
        
        self.save_map(map=self.xnormalmap,path="x.png")
        self.save_map(map=self.ynormalmap,path="y.png")


    def save_map(self, map=None, path="noise.png"):
            if map is None:
                # print("map is not None")
                map=self.heightmap
                arr = map.copy()

                arr/=50
                arr*=255
                arr = arr.astype(np.uint8)
                arr = np.repeat(arr, 3, axis=-1)
                arr = arr.reshape(height,width,3)

            else:
                arr = map.copy()
                negative_arr = map.copy()
                arr = arr.clip(min=0)
                negative_arr*=-1
                negative_arr = negative_arr.clip(min=0)

                # print(arr.max())
                # print(arr.min())
                # arr-=arr.min()
                arr/=arr.max()
                negative_arr/=arr.max()

                arr = arr.reshape(height,width,1)
                negative_arr =  negative_arr.reshape(height,width,1)
                # print(f"shape: {arr.shape}")
                # print(f"shape: {negative_arr.shape}")



                arr = np.concatenate((arr,negative_arr,negative_arr),axis=2)
                # print(arr.shape)
                arr = arr.reshape(height,width,3)
                # print(arr.shape)
                # print(arr.max() )
                # print(arr.min() )
                arr*=255
                arr = arr.astype(np.uint8)



                # print("custom map set")

            # print(map)
            # print("saving")
            # print(arr.max())
            # print(arr.min())

            # print(arr.max())
            # print(arr.min()) 
            # normalize
            # arr = arr/arr.max()
            # convert to black and white image

            # save out
            from PIL import Image
            im = Image.fromarray(arr)
            im.save(path)



# def test_terrain():
#     terrain = Terrain()
#     position = np.array([5,5])
#     normal_2D = terrain.normal_2D(position)
#     assert normal_2D is 5, f"{normal_2D=}"


    