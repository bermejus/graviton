use spice;

use ndarray::{arr1, s};
use algorithm::*;
use simulation::*;

mod algorithm;
mod simulation;

fn initialize_planets(bodies: &mut Bodies) {
    for body in bodies.iter_mut() {
        let body_name = body.0.split(' ').next().unwrap();

        body.1.mu = spice::bodvrd(body.0, "GM", 1)[0];
        body.1.radii = spice::bodvrd(body_name, "RADII", 3).try_into().unwrap();
    }
}

fn update_planets(bodies: &mut Bodies, et: f64) {
    for body in bodies.iter_mut() {
        body.1.position = arr1(&spice::spkpos(body.0, et, "J2000", "NONE", "SSB").0);
    }
}

fn concat<T>(x1: &[T], x2: &[T]) -> Vec<T>
    where T: Clone {
    [x1, x2].concat()
}

fn main() {
    let mut params = Parameters::from([
        ("bodies", Bodies::from([
            ("SUN", CelestialBody::empty()),
            ("MERCURY BARYCENTER", CelestialBody::empty()),
            ("VENUS BARYCENTER", CelestialBody::empty()),
            ("EARTH", CelestialBody::empty()),
            ("MOON", CelestialBody::empty()),
            ("MARS BARYCENTER", CelestialBody::empty()),
            ("JUPITER BARYCENTER", CelestialBody::empty()),
            ("SATURN BARYCENTER", CelestialBody::empty()),
            ("URANUS BARYCENTER", CelestialBody::empty()),
            ("NEPTUNE BARYCENTER", CelestialBody::empty()),
            ("PLUTO BARYCENTER", CelestialBody::empty())
        ]).into()),
        ("m", 100f64.into())
    ]);

    spice::furnsh("assets/kernels/kernels.tm");
    let et = spice::str2et("2127-MAR-23 16:00:00");
    
    let bodies: &mut Bodies = params.get_mut("bodies").unwrap().get_mut();
    initialize_planets(bodies);
    update_planets(bodies, et);
    println!("{:#?}", bodies);

    spice::unload("assets/kernels/kernels.tm");

    let y0 = arr1(&concat(&[1.2 * AU; 3], &[20.0; 3]));
    let tspan = [0.0, 3600.0 * 24.0 * 365.0 * 30.0];
    let mut res = ode87(|y, t| dynamics(y, t, &params), y0.view(), tspan, 1e-12, 1e-12, None);
    res.slice_mut(s![..3]).mapv_inplace(|e| e / AU);
    println!("Result: {}", res);
}
