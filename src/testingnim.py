import nimporter
import sim

def testnim():
    print("testing nim...")
    
    tt = sim.Terrain()
    tt.setNumber(12,2,2)
    print(tt.getNumber(2,2))
    print("nim tested!")