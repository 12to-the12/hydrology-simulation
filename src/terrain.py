
from config import config, height, width, noise_detail
import numpy as np
import opensimplex # type: ignore
from random import randint



class Terrain:
    def __init__(self) -> None:
        self.initialize_to_noise()

    def initialize_to_noise(self):
        self.heightmap = np.zeros((height,width))
        # self.add_noise_pass()
        self.initialize_brownian(noise_detail)

    def initialize_brownian(self, octaves):
        for octave in range(octaves):
            frequency = 2**octave
            amplitude = 2**(octaves-octave-1)
            print(f"{frequency=}")
            print(f"{amplitude=}")

            self.add_noise_pass(frequency,amplitude)

        
    def add_noise_pass(self, frequency=1, amplitude=1):
        opensimplex.random_seed()

        # arr = np.zeros((height,width))
        x_coords = np.linspace(0,frequency,width)
        y_coords = np.linspace(0,frequency,height)
        arr = opensimplex.noise2array(x_coords,y_coords)

        # for y in range(height):
        #     for x in range(width):
        #         arr[y][x] = opensimplex.noise2(x/width*frequency,y/height*frequency)
        arr+=1
        arr/=2
        # arr = 2**arr


        self.heightmap += arr*amplitude


    def save_heightmap(self, path="noise.png"):
            # normalize
            arr = self.heightmap
            # print(self.heightmap.max())
            # print(self.heightmap.min())

            # arr+=1
            # print(arr.max())
            # print(arr.min())

            arr = arr/arr.max()
            # convert to black and white image
            arr*=255
            arr = arr.astype(np.uint8)
            arr = np.repeat(arr, 3, axis=-1)
            arr = arr.reshape(height,width,3)
            # save out
            from PIL import Image
            im = Image.fromarray(arr)
            print("here")
            im.save(path)


        