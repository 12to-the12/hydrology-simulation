import toml
toml.load("config.toml")
with open('config.toml', 'r') as f:
    config = toml.load(f)
width = config["width"]
height = config["height"]
noise_detail = config["noise_detail"]
persistence = config["persistence"]
lacunarity = config["lacunarity"]
seed = config["seed"]

