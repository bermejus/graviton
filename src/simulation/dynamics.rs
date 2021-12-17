use ndarray::{ArrayView1, Array1, arr1, s};
use crate::parameters::*;
use crate::constants::*;
use crate::celestial_body::*;
use crate::algorithm::linalg::*;

fn calculate_accel(bodies: &Bodies, position: ArrayView1<f64>) -> Array1<f64> {
    let mut accel = arr1(&[0.0; 3]);
    for body in bodies.iter() {
        let r = &body.1.position - &position;
        let r_mag = magnitude(&r);
        let rhat = normalize(&r);

        let local_accel = -(body.1.mu / r_mag) * rhat;
        accel += &local_accel;
    }
    accel
}

pub fn dynamics(y: ArrayView1<f64>, t: f64, p: &Parameters) -> Array1<f64> {
    let mut dy = Array1::<f64>::zeros(6);
    dy.slice_mut(s![..3]).assign(&y.slice(s![3..6]));

    // Fetch bodies and update their position
    /*let bodies = p.get_mut("bodies").unwrap().get_mut();
    for body in bodies.iter_mut() {
        body.1.position
    }*/
    // Calculate acceleration at given point
    let accel = calculate_accel(
        p.get("bodies").unwrap().get(),
        y.slice(s![..3])
    );

    dy.slice_mut(s![3..6]).assign(&accel);
    dy
}