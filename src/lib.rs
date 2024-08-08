use pyo3::prelude::*;
use std::fs;
mod objects;
use objects::*;

#[pyfunction]
fn calc_hash_animation(b1:Vec<f32>,b2:Vec<f32>,b3mass:f32,file:String,dt:f32,iterations_per_byte:usize) -> PyResult<Vec<Vec<Vec<f32>>>> {
    //initializing Body structs
    let mut b1 = Body::new(b1[0],b1[1], b1[2], b1[3], b1[4]);
    let mut b2 = Body::new(b2[0],b2[1], b2[2], b2[3], b2[4]);
    //creating bytes vector
    let file_bytes = fs::read(file).unwrap();
    let file_len = file_bytes.len();
    let mut b3 = Body::new(b3mass
                                , int_to_float(file_bytes[0], 1.0, 127.0)
                                , int_to_float(file_bytes[1], 1.0, 127.0)
                                , int_to_float(file_bytes[2], 4.0, 32.0)
                                , int_to_float(file_bytes[3], 4.0, 32.0));

    //calculating initial conditions
    let a12 = b1.calc_force(&b2);
    let a13 = b1.calc_force(&b3);
    let a23 = b2.calc_force(&b3);

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
    for byte in 4..file_len{
        for _step in 0..iterations_per_byte {
            //position update step
            b1.step_position(fa, dt);
            b2.step_position(fb, dt);
            b3.step_position(fc, dt);

            //acceleration update step
            let a12_n = b1.calc_force(&b2);
            let a13_n = b1.calc_force(&b3);
            let a23_n = b2.calc_force(&b3);

            //force update step
            let fa_n = [(a12_n[0]+a13_n[0])/b1.mass,(a12_n[1]+a13_n[1])/b1.mass];
            let fb_n = [(-a12_n[0]+a23_n[0])/b2.mass,(-a12_n[1]+a23_n[1])/b2.mass];
            let fc_n =  [(-a13_n[0]-a23_n[0])/b3.mass,(-a13_n[1]-a23_n[1])/b3.mass];

            //velocity update step
            b1.step_velocity(fa_n, fa, dt);
            b2.step_velocity(fb_n, fb, dt);
            b3.step_velocity(fc_n, fc, dt);

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

        match byte%4 {
            0 => b3.x = int_to_float(file_bytes[byte], 1.0, 127.0),
            1 => b3.y = int_to_float(file_bytes[byte], 1.0, 127.0),
            2 => b3.vx = int_to_float(file_bytes[byte], 4.0, 32.0),
            3 => b3.vy = int_to_float(file_bytes[byte], 4.0, 32.0),
            _ => ()
        }
        /*  old method
        b3 = Body::new(b3mass
            , int_to_float(file_bytes[byte*4], 1.0, 127.0)
            , int_to_float(file_bytes[byte*4+1], 1.0, 127.0)
            , int_to_float(file_bytes[byte*4+2], 5.0, 6.3)
            , int_to_float(file_bytes[byte*4+3], 5.0, 6.3)); */
        
        
    }

    //combining the vectors to the result vector
    let res = vec!(vec!(b1_x,b1_y),vec!(b2_x,b2_y),vec!(b3_x,b3_y));

    Ok(res)
}

#[pyfunction]
fn fast_hash_file(b1:Vec<f32>,b2:Vec<f32>,b3mass:f32,file:String,dt:f32,iterations_per_byte:usize) -> PyResult<Vec<f32>> {
    //initializing Body structs
    let mut b1 = Body::new(b1[0],b1[1], b1[2], b1[3], b1[4]);
    let mut b2 = Body::new(b2[0],b2[1], b2[2], b2[3], b2[4]);
    //creating bytes vector
    let file_bytes = fs::read(file).unwrap();
    let file_len = file_bytes.len();
    let mut b3 = Body::new(b3mass
                                , int_to_float(file_bytes[0], 1.0, 127.0)
                                , int_to_float(file_bytes[1], 1.0, 127.0)
                                , int_to_float(file_bytes[2], 4.0, 32.0)
                                , int_to_float(file_bytes[3], 4.0, 32.0));

    //calculating initial conditions
    let a12 = b1.calc_force(&b2);
    let a13 = b1.calc_force(&b3);
    let a23 = b2.calc_force(&b3);

    let mut fa = [(a12[0]+a13[0])/b1.mass,(a12[1]+a13[1])/b1.mass];
    let mut fb = [(-a12[0]+a23[0])/b2.mass,(-a12[1]+a23[1])/b2.mass];
    let mut fc =  [(-a13[0]-a23[0])/b3.mass,(-a13[1]-a23[1])/b3.mass];

    //numerical iterative solution using verlet integrals
    for byte in 4..file_len{
        for _step in 0..iterations_per_byte {
            //position update step
            b1.step_position(fa, dt);
            b2.step_position(fb, dt);
            b3.step_position(fc, dt);

            //acceleration update step
            let a12_n = b1.calc_force(&b2);
            let a13_n = b1.calc_force(&b3);
            let a23_n = b2.calc_force(&b3);

            //force update step
            let fa_n = [(a12_n[0]+a13_n[0])/b1.mass,(a12_n[1]+a13_n[1])/b1.mass];
            let fb_n = [(-a12_n[0]+a23_n[0])/b2.mass,(-a12_n[1]+a23_n[1])/b2.mass];
            let fc_n =  [(-a13_n[0]-a23_n[0])/b3.mass,(-a13_n[1]-a23_n[1])/b3.mass];

            //velocity update step
            b1.step_velocity(fa_n, fa, dt);
            b2.step_velocity(fb_n, fb, dt);
            b3.step_velocity(fc_n, fc, dt);

            //setting the forces for the next iteration
            fa = fa_n;
            fb = fb_n;
            fc= fc_n;
        }

        match byte%4 {
            0 => b3.x = int_to_float(file_bytes[byte], 1.0, 127.0),
            1 => b3.y = int_to_float(file_bytes[byte], 1.0, 127.0),
            2 => b3.vx = int_to_float(file_bytes[byte], 4.0, 32.0),
            3 => b3.vy = int_to_float(file_bytes[byte], 4.0, 32.0),
            _ => ()
        }
    }
    let res = vec!(b1.x,b1.y,b2.x,b2.y);

    //combining the vectors to the result vector
    Ok(res)
}

#[pyfunction]
fn fast_hash_data(b1:Vec<f32>,b2:Vec<f32>,b3mass:f32,data:Vec<u8>,dt:f32,iterations_per_byte:usize) -> PyResult<Vec<f32>> {
    //initializing Body structs
    let mut b1 = Body::new(b1[0],b1[1], b1[2], b1[3], b1[4]);
    let mut b2 = Body::new(b2[0],b2[1], b2[2], b2[3], b2[4]);
    //creating bytes vector
    let data_len = data.len();
    let mut b3 = Body::new(b3mass
                                , int_to_float(data[0], 1.0, 127.0)
                                , int_to_float(data[1], 1.0, 127.0)
                                , int_to_float(data[2], 4.0, 32.0)
                                , int_to_float(data[3], 4.0, 32.0));

    //calculating initial conditions
    let a12 = b1.calc_force(&b2);
    let a13 = b1.calc_force(&b3);
    let a23 = b2.calc_force(&b3);

    let mut fa = [(a12[0]+a13[0])/b1.mass,(a12[1]+a13[1])/b1.mass];
    let mut fb = [(-a12[0]+a23[0])/b2.mass,(-a12[1]+a23[1])/b2.mass];
    let mut fc =  [(-a13[0]-a23[0])/b3.mass,(-a13[1]-a23[1])/b3.mass];

    //numerical iterative solution using verlet integrals
    for byte in 4..data_len{
        for _step in 0..iterations_per_byte {
            //position update step
            b1.step_position(fa, dt);
            b2.step_position(fb, dt);
            b3.step_position(fc, dt);

            //acceleration update step
            let a12_n = b1.calc_force(&b2);
            let a13_n = b1.calc_force(&b3);
            let a23_n = b2.calc_force(&b3);

            //force update step
            let fa_n = [(a12_n[0]+a13_n[0])/b1.mass,(a12_n[1]+a13_n[1])/b1.mass];
            let fb_n = [(-a12_n[0]+a23_n[0])/b2.mass,(-a12_n[1]+a23_n[1])/b2.mass];
            let fc_n =  [(-a13_n[0]-a23_n[0])/b3.mass,(-a13_n[1]-a23_n[1])/b3.mass];

            //velocity update step
            b1.step_velocity(fa_n, fa, dt);
            b2.step_velocity(fb_n, fb, dt);
            b3.step_velocity(fc_n, fc, dt);

            //setting the forces for the next iteration
            fa = fa_n;
            fb = fb_n;
            fc= fc_n;
        }

        match byte%4 {
            0 => b3.x = int_to_float(data[byte], 1.0, 127.0),
            1 => b3.y = int_to_float(data[byte], 1.0, 127.0),
            2 => b3.vx = int_to_float(data[byte], 4.0, 32.0),
            3 => b3.vy = int_to_float(data[byte], 4.0, 32.0),
            _ => ()
        }
    }
    let res = vec!(b1.x,b1.y,b2.x,b2.y);

    //combining the vectors to the result vector
    Ok(res)
}


#[pyfunction]
fn calc_three_body(b1:Vec<f32>,b2:Vec<f32>,b3:Vec<f32>,dt:f32,iterations:usize) -> PyResult<Vec<Vec<Vec<f32>>>> {
    //initializing Body structs
    let mut b1 = Body::new(b1[0],b1[1], b1[2], b1[3], b1[4]);
    let mut b2 = Body::new(b2[0],b2[1], b2[2], b2[3], b2[4]);
    let mut b3 = Body::new(b3[0],b3[1], b3[2], b3[3], b3[4]);

    //calculating initial conditions
    let a12 = b1.calc_force(&b2);
    let a13 = b1.calc_force(&b3);
    let a23 = b2.calc_force(&b3);

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
        b1.step_position(fa, dt);
        b2.step_position(fb, dt);
        b3.step_position(fc, dt);

        //acceleration update step
        let a12_n = b1.calc_force(&b2);
        let a13_n = b1.calc_force(&b3);
        let a23_n = b2.calc_force(&b3);

        //force update step
        let fa_n = [(a12_n[0]+a13_n[0])/b1.mass,(a12_n[1]+a13_n[1])/b1.mass];
        let fb_n = [(-a12_n[0]+a23_n[0])/b2.mass,(-a12_n[1]+a23_n[1])/b2.mass];
        let fc_n =  [(-a13_n[0]-a23_n[0])/b3.mass,(-a13_n[1]-a23_n[1])/b3.mass];

        //velocity update step
        b1.step_velocity(fa_n, fa, dt);
        b2.step_velocity(fb_n, fb, dt);
        b3.step_velocity(fc_n, fc, dt);

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
    m.add_function(wrap_pyfunction!(fast_hash_file, m)?)?;
    m.add_function(wrap_pyfunction!(fast_hash_data, m)?)?;
    Ok(())
}

fn int_to_float(integer:u8,divide:f32,subtract:f32) -> f32 {
    return ((integer as f32)/divide)-subtract
}