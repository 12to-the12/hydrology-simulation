import std/math
type
    vector3d* = array[3,float]
    vector2d* = array[2,float]

func `*`*(a: vector3d, b:vector3d): vector3d =
    [a[0]*b[0],a[1]*b[0],a[2]*b[2]]

func `*`*(a: vector3d, b:float): vector3d =
    [a[0]*b,a[1]*b,a[2]*b]

func `/`*(a: vector3d, b:vector3d): vector3d =
    [a[0]/b[0],a[1]/b[1],a[2]/b[2]]

func `/`*(a: vector3d, b:float): vector3d =
    [a[0]/b,a[1]/b,a[2]/b]

func `+`*(a: vector3d, b:vector3d): vector3d =
    [a[0]+b[0],a[1]+b[1],a[2]+b[2]]

func `+`*(a: vector3d, b:float): vector3d =
    [a[0]+b,a[1]+b,a[2]+b]

func `-`*(a: vector3d, b:vector3d): vector3d =
    [a[0]-b[0],a[1]-b[1],a[2]-b[2]]

func `-`*(a: vector3d, b:float): vector3d =
    [a[0]-b,a[1]-b,a[2]-b]

func `x`*(vec: vector3d): float =
    vec[0]
func `y`*(vec: vector3d): float =
    vec[1]
func `z`*(vec: vector3d): float =
    vec[2]
func `x`*(vec: vector2d): float =
    vec[0]
func `y`*(vec: vector2d): float =
    vec[1]

func cross*(a: vector3d, b: vector3d): vector3d =
    [  a[1]*b[2] - a[2]*b[1]  ,a[2]*b[0] - a[0]*b[2]  ,a[0]*b[1] - a[1]*b[0]  ]






func `dot`*(a: vector2d, b:vector2d): float =
    a[0]*b[0]+a[1]*b[1]

func `+`*(a: vector2d, b:float): vector2d =
    [a[0]+b,a[1]+b]

func `+`*(a: vector2d, b:vector2d): vector2d =
    [a[0]+b[0],a[1]+b[1]]

func `*`*(a: vector2d, b:float): vector2d =
    [a[0]*b,a[1]*b]

func `/`*(a: vector2d, b:float): vector2d =
    [a[0]/b,a[1]/b]


func magnitude*(vec: vector2d): float = 
    sqrt(vec[0]*vec[0]+vec[1]*vec[1])

func magnitude*(vec: vector3d): float = 
    sqrt(vec[0]*vec[0]+vec[1]*vec[1]+vec[2]*vec[2])

func unitize*(vec: vector2d): vector2d = 
    let mag: float = vec.magnitude
    if mag==0: return [0.0,0.0]
    vec/mag

func unitize*(vec: vector3d): vector3d = 
    let mag: float = vec.magnitude
    vec/mag

    

