
import terrain
import std/random
import config
import linalg
import rendering
import std/math
import times, os




type Particle = object
    velocity: vector2d
    pos: array[2, float]
    volume: float   # cubic meters
    initial_volume: float
    sediment: float # total transported sediment in cubic meter
    steps: int
    age: float


func speed(particle: Particle): float =
    particle.velocity.magnitude

# func carrying_capacity(particle: Particle) = dropParticle
#     particle.speed*particle.volume

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
    particle.pos = [x, y]
    particle.volume = 1.0
    particle.velocity = [0.0, 0.0]
    particle.steps = 0
    particle.sediment = 0.0

    terrain.Δimpact(1.0, particle.pos.x.int,
                particle.pos.y.int)
    
    while true:
        
        # if age > MAX_AGE: return
        if particle.volume < MIN_VOLUME: return
        startheight = terrain.getNumber(particle.pos.x.int, particle.pos.y.int)
        normal = terrain.get_normal_2d(particle.pos.x.int, particle.pos.y.int)

        startpos = particle.pos

        ## both
        particle.velocity = particle.velocity+normal.unitize*0.1

        ## MOMENTUM MAP STUFF
        var momentum = terrain.getMomentum(particle.pos.x.int,
                particle.pos.y.int).unitize
        var scaling = (normal.unitize).dot(momentum.unitize)

        particle.velocity = particle.velocity+momentum*(
                scaling*MOMENTUM_SCALING)

        particle.pos = particle.pos + particle.velocity

        if not (0 < particle.pos.y and particle.pos.y < terrain.height): return
        if not (0 < particle.pos.x and particle.pos.x < terrain.width): return
        # echo particle.pos
        # echo "normal: ", normal.unitize
        dt = 1/particle.velocity.magnitude
        age += dt
        Δheight = startheight-terrain.getNumber(particle.pos.x.int,
                particle.pos.y.int)
        # if Δheight>0: return


        # echo "Δheight: ", Δheight
        terrain.Δvolume(particle.volume, particle.pos.x.int,
                particle.pos.y.int)
        terrain.ΔMomentum(particle.velocity, particle.pos.x.int,
                particle.pos.y.int)







        # volume * speed * change in height
        let local_flow_rate = terrain.get_volume(particle.pos.x.int,
                particle.pos.y.int)

        var carrying_capacity: float = particle.volume * particle.speed * Δheight
        # var carrying_capacity: float = particle.volume * particle.speed * Δheight
        if carrying_capacity < 0.0: carrying_capacity = 0
        # echo "carrying capacity: ", carrying_capacity

        if particle.sediment<0: particle.sediment=0
        # soaking / deposition force. positive is soak, negative is deposition
        var soaking_force = carrying_capacity - particle.sediment

        var picked_up = dt * DEPOSITION_RATE * soaking_force
        
        # echo "volume:", particle.volume
        # echo "Δheight:", Δheight
        # echo "speed:", particle.speed
        # echo "dt:", dt
        particle.sediment += picked_up
        # echo "soaking force:", soaking_force
        # echo "picked up:", picked_up
        # echo ""


        var deposited = dt * particle.volume * DEPOSITION_RATE * soaking_force
        # if particle.sediment.isNaN:
        #     echo "sediment is not a number"
        #     # echo carrying_capacity
        #     # echo particle.sediment
        #     # echo "it's not a number"
        #     sleep(10_000)
        # echo "sediment:", particle.sediment
        # echo "deposited:", deposited
        # echo "otherwise:", -Δheight/2

        terrain.Δheight(-deposited, startpos.x.int, startpos.y.int)

        # var newΔheight = startheight-terrain.getNumber(particle.pos.x.int,
        #         particle.pos.y.int)

        # if newΔheight>MAX_REPOSE:
        #     var allowed_delta = newΔheight-MAX_REPOSE
        #     terrain.Δheight(-allowed_delta/2, startpos.x.int, startpos.y.int)
        #     terrain.Δheight(allowed_delta/2, particle.pos.x.int, particle.pos.y.int)

        # terrain.Δheight(-Δheight/2, startpos.x.int, startpos.y.int)
        # terrain.Δheight(Δheight/2, particle.pos.x.int, particle.pos.y.int)

        particle.volume *= (1 - EVAPORATION_RATE).pow(dt)

        particle.velocity = particle.velocity*(1-FRICTION).pow(dt)
    # terrain.Δheight(1,particle.pos.x.int,particle.pos.y.int)




# echo "birth: ", x, " ",y
# echo "current: ", particle.pos




# inplace
proc erode*(terrain: Terrain, particles: int) =
    randomize()
    for particle in 0..<particles:
        if particle mod COMPUTE_INTERVAL == 0:
            echo "computing maps..."
            terrain.computeMaps()
        if particle mod 10_000 == 0:
            echo particle
        if particle mod RENDER_INTERVAL == 0:
            echo particle
            renderTerrain(terrain)
        terrain.dropParticle()

