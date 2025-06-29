
import terrain
import std/random
import config
import  linalg
import rendering

const
    DENSITY: float = 1
    EVAPORATION_RATE = 1e-3
    DEPOSITION_RATE = 8


type Particle = object
    velocity: array[2,float]
    pos: array[2,float]
    volume: float # cubic meters
    initial_volume: float
    sediment: float # % of capacity
    steps: int
    age: float
    
    



proc dropParticle(terrain: Terrain) =
    var
        x = rand(ROWS-1).float
        y = rand(COLUMNS-1).float
        dt: float
        age: float
        startheight: float
        startpos: vector2d
        Δheight: float

    var particle = Particle()
    var normal: vector2d
    particle.pos = [x,y]
    particle.volume = 10.0
    particle.velocity = [0.0,0.0]
    particle.steps = 0
    while true:
        if age>MAX_AGE: return
        startheight = terrain.getNumber(particle.pos.x.int,particle.pos.y.int)   
        normal = terrain.get_normal_2d(particle.pos.x.int,particle.pos.y.int)
        dt = 1/normal.magnitude
        startpos = particle.pos
        particle.pos = particle.pos + normal.unitize
        # echo particle.pos
        # echo "normal: ", normal.unitize
        age += dt
        Δheight = startheight-terrain.getNumber(particle.pos.x.int,particle.pos.y.int)
        # if Δheight>0: return
        if not (0 < particle.pos.y and particle.pos.y   < terrain.height): return
        if not (0 < particle.pos.x and particle.pos.x   < terrain.width): return

        # echo "Δheight: ", Δheight
        terrain.paintposition(particle.pos.x.int,particle.pos.y.int)
        terrain.Δheight(-Δheight/2,startpos.x.int,startpos.y.int)
        terrain.Δheight(Δheight/2,particle.pos.x.int,particle.pos.y.int)
        # terrain.Δheight(1,particle.pos.x.int,particle.pos.y.int)
    # echo "birth: ", x, " ",y
    # echo "current: ", particle.pos

    


# inplace
proc erode*(terrain: Terrain, particles: int) = 
    randomize()
    for particle in 0..<particles:
        if particle mod 1_000 == 0:
            echo particle
            renderTerrain(terrain)
        terrain.dropParticle()
    echo "eroding..."

