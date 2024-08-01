use pyo3::prelude::*;
use std::fs;

#[pyfunction]
fn calc_hash_animation(b1:Vec<f32>,b2:Vec<f32>,b3mass:f32,file:String,dt:f32,iterations_per_byte:usize) -> PyResult<Vec<Vec<Vec<f32>>>> {
    //initializing body structs
    let mut b1 = body::new(b1[0],b1[1], b1[2], b1[3], b1[4]);
    let mut b2 = body::new(b2[0],b2[1], b2[2], b2[3], b2[4]);
    //creating bytes vector
    let file_bytes = fs::read(file).unwrap();
    let file_len = file_bytes.len();
    let mut b3 = body::new(b3mass
                                , int_to_float(file_bytes[0], 32.0, 4.0)
                                , int_to_float(file_bytes[1], 32.0, 4.0)
                                , int_to_float(file_bytes[2], 85.0, 1.5)
                                , int_to_float(file_bytes[3], 85.0, 1.5));

    //calculating initial conditions
    let a12 = b1.calc_force_div_mass(&b2);
    let a13 = b1.calc_force_div_mass(&b3);
    let a23 = b2.calc_force_div_mass(&b3);

    let mut fa = [(a12[0]+a13[0])/b1.mass,(a12[1]+a13[1])/b1.mass];
    let mut fb = [(-a12[0]+a23[0])/b2.mass,(-a12[1]+a23[1])/b2.mass];
    let mut fc =  [(-a13[0]-a23[0])/b3.mass,(-a13[1]-a23[1])/b3.mass];

    //initiating result vectors
    let mut b1_x = vec!(b1.x);
    let mut b1_y = vec!(b1.y);
    let mut b2_x = vec!(b2.x);
    let mut b2_y = vec!(b2.y);
    let mut b3_x = vec!(b3.x);
    let mut b3_y = vec!(b3.y);

    //numerical iterative solution using verlet integrals
    for byte in 1..(file_len/4){
        for _step in 0..iterations_per_byte {
            //position update step
            b1.step_pos(fa, dt);
            b2.step_pos(fb, dt);
            b3.step_pos(fc, dt);

            //acceleration update step
            let a12_n = b1.calc_force_div_mass(&b2);
            let a13_n = b1.calc_force_div_mass(&b3);
            let a23_n = b2.calc_force_div_mass(&b3);

            //force update step
            let fa_n = [(a12_n[0]+a13_n[0])/b1.mass,(a12_n[1]+a13_n[1])/b1.mass];
            let fb_n = [(-a12_n[0]+a23_n[0])/b2.mass,(-a12_n[1]+a23_n[1])/b2.mass];
            let fc_n =  [(-a13_n[0]-a23_n[0])/b3.mass,(-a13_n[1]-a23_n[1])/b3.mass];

            //velocity update step
            b1.step_vel(fa_n, fa, dt);
            b2.step_vel(fb_n, fb, dt);
            b3.step_vel(fc_n, fc, dt);

            //setting the forces for the next iteration
            fa = fa_n;
            fb = fb_n;
            fc= fc_n;

            //pushing the new positions to the result vectors
            b1_x.push(b1.x);
            b1_y.push(b1.y);
            b2_x.push(b2.x);
            b2_y.push(b2.y);
            b3_x.push(b3.x);
            b3_y.push(b3.y);
        }

        b3 = body::new(b3mass
            , int_to_float(file_bytes[byte*4], 32.0, 4.0)
            , int_to_float(file_bytes[byte*4+1], 32.0, 4.0)
            , int_to_float(file_bytes[byte*4+2], 85.0, 1.5)
            , int_to_float(file_bytes[byte*4+3], 85.0, 1.5));
        
    }

    //combining the vectors to the result vector
    let res = vec!(vec!(b1_x,b1_y),vec!(b2_x,b2_y),vec!(b3_x,b3_y));

    Ok(res)
}

#[pyfunction]
fn calc_three_body(b1:Vec<f32>,b2:Vec<f32>,b3:Vec<f32>,dt:f32,iterations:usize) -> PyResult<Vec<Vec<Vec<f32>>>> {
    //initializing body structs
    let mut b1 = body::new(b1[0],b1[1], b1[2], b1[3], b1[4]);
    let mut b2 = body::new(b2[0],b2[1], b2[2], b2[3], b2[4]);
    let mut b3 = body::new(b3[0],b3[1], b3[2], b3[3], b3[4]);

    //calculating initial conditions
    let a12 = b1.calc_force_div_mass(&b2);
    let a13 = b1.calc_force_div_mass(&b3);
    let a23 = b2.calc_force_div_mass(&b3);

    let mut fa = [(a12[0]+a13[0])/b1.mass,(a12[1]+a13[1])/b1.mass];
    let mut fb = [(-a12[0]+a23[0])/b2.mass,(-a12[1]+a23[1])/b2.mass];
    let mut fc =  [(-a13[0]-a23[0])/b3.mass,(-a13[1]-a23[1])/b3.mass];

    //initiating result vectors
    let mut b1_x = vec!(b1.x);
    let mut b1_y = vec!(b1.y);
    let mut b2_x = vec!(b2.x);
    let mut b2_y = vec!(b2.y);
    let mut b3_x = vec!(b3.x);
    let mut b3_y = vec!(b3.y);

    //numerical iterative solution using verlet integrals
    for _step in 0..iterations {
        //position update step
        b1.step_pos(fa, dt);
        b2.step_pos(fb, dt);
        b3.step_pos(fc, dt);

        //acceleration update step
        let a12_n = b1.calc_force_div_mass(&b2);
        let a13_n = b1.calc_force_div_mass(&b3);
        let a23_n = b2.calc_force_div_mass(&b3);

        //force update step
        let fa_n = [(a12_n[0]+a13_n[0])/b1.mass,(a12_n[1]+a13_n[1])/b1.mass];
        let fb_n = [(-a12_n[0]+a23_n[0])/b2.mass,(-a12_n[1]+a23_n[1])/b2.mass];
        let fc_n =  [(-a13_n[0]-a23_n[0])/b3.mass,(-a13_n[1]-a23_n[1])/b3.mass];

        //velocity update step
        b1.step_vel(fa_n, fa, dt);
        b2.step_vel(fb_n, fb, dt);
        b3.step_vel(fc_n, fc, dt);

        //setting the forces for the next iteration
        fa = fa_n;
        fb = fb_n;
        fc= fc_n;

        //pushing the new positions to the result vectors
        b1_x.push(b1.x);
        b1_y.push(b1.y);
        b2_x.push(b2.x);
        b2_y.push(b2.y);
        b3_x.push(b3.x);
        b3_y.push(b3.y);
    }

    //combining the vectors to the result vector
    let res = vec!(vec!(b1_x,b1_y),vec!(b2_x,b2_y),vec!(b3_x,b3_y));

    Ok(res)
}

/// A Python module implemented in Rust.
#[pymodule]
fn three_body_hash(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calc_three_body, m)?)?;
    m.add_function(wrap_pyfunction!(calc_hash_animation, m)?)?;
    Ok(())
}

//objects
pub struct body {
    mass : f32,
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

fn int_to_float(integer:u8,divide:f32,subtract:f32) -> f32 {
    return ((integer as f32)/divide)-subtract
}