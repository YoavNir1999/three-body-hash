//objects
pub struct body {
    pub mass : f32,
    pub x : f32,
    pub y : f32,
    vx : f32,
    vy : f32
}

impl body {
    pub fn new(mass:f32,x:f32,y:f32,vx:f32,vy:f32) -> body {
        let b = body{
            mass,
            x,
            y,
            vx,
            vy
        };
        b
    }
    //calculating the force vectors
    pub fn calc_force_div_mass(&self,other:&body) -> [f32;2] {
        let r_vector  = [other.x-self.x,other.y-self.y];
        let dist = (r_vector[0].powi(2)+r_vector[1].powi(2)).powf(1.5);
        return [self.mass*other.mass*r_vector[0]/dist,self.mass*r_vector[1]/dist]
    }

    //calculating the next position vector
    pub fn step_pos(&mut self,force:[f32;2],dt:f32) {
        self.x = self.x +self.vx*dt + 0.5*force[0]*dt.powi(2);
        self.y = self.y +self.vy*dt + 0.5*force[1]*dt.powi(2);
    }

    //calculating the next valocity vector
    pub fn step_vel(&mut self,a_new:[f32;2],a_old:[f32;2],dt:f32) {
        self.vx = self.vx+0.5*(a_new[0]+a_old[0])*dt;
        self.vy = self.vy+0.5*(a_new[1]+a_old[1])*dt;

        let limx = 4.0;
        let limy = 4.0;

        if self.x.abs() > limx{
            self.vx = -0.5 * self.vx;
            if self.x>limx {
                self.x = limx
            } else if self.x<limx {
                self.x = -limx
            };
        };
        if self.y.abs() > limy {
            self.vy = -0.5 * self.vy;
            if self.y>limy {
                self.y = limy
            } else if self.y<limy {
                self.y = -limy
            };
        };
    }
}