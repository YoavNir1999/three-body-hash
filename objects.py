class Body:
    def __init__(self,mass,x,y,vx,vy):
        self.mass = mass
        self.x = x
        self.y = y
        self.vx = vx
        self.vy = vy

    def to_list(self):
        return [self.mass,self.x,self.y,self.vx,self.vy]

