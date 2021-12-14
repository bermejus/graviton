use spice;

use ndarray::{Array1, ArrayView1, arr1, aview1};
use algorithm::*;
use simulation::*;

mod algorithm;
mod simulation;

/*const BODIES: [&str; 11] = [
    "SUN",
    "MERCURY BARYCENTER",
    "VENUS BARYCENTER",
    "EARTH",
    "MOON",
    "MARS BARYCENTER",
    "JUPITER BARYCENTER",
    "SATURN BARYCENTER",
    "URANUS BARYCENTER",
    "NEPTUNE BARYCENTER",
    "PLUTO BARYCENTER"
];*/

fn fcn(y: ArrayView1<f64>, t: f64) -> Array1<f64> {
    &y * t
}

fn main() {
    let mut bodies = Bodies::from([
        ("SUN", CelestialBody::zero()),
        ("MERCURY BARYCENTER", CelestialBody::zero()),
        ("VENUS BARYCENTER", CelestialBody::zero()),
        ("EARTH", CelestialBody::zero()),
        ("MOON", CelestialBody::zero()),
        ("MARS BARYCENTER", CelestialBody::zero()),
        ("JUPITER BARYCENTER", CelestialBody::zero()),
        ("SATURN BARYCENTER", CelestialBody::zero()),
        ("URANUS BARYCENTER", CelestialBody::zero()),
        ("NEPTUNE BARYCENTER", CelestialBody::zero()),
        ("PLUTO BARYCENTER", CelestialBody::zero())
    ]);
    
    spice::furnsh("assets/kernels/kernels.tm");
    let et = spice::str2et("2127-MAR-23 16:00:00");

    for body in bodies.iter_mut() {
        // Update the gravitational constant
        let mu = spice::bodvrd(body.0, "GM", 1)[0];
        body.1.mu = mu;

        // Update the position
        let position = spice::spkpos(body.0, et, "J2000", "NONE", "SSB").0;
        body.1.position = arr1(&position);

        // Print the body state
        println!("{} position: {:#}, mu: {}", body.0, body.1.position, body.1.mu);
    }
    spice::unload("assets/kernels/kernels.tm");

    let y0 = arr1(&[2.0]);
    let tspan = [0.0, 5.0];

    let res1 = ode54(fcn, y0.view(), tspan, 1e-12, 1e-12, None)[0];
    let res2 = ode87(fcn, y0.view(), tspan, 1e-12, 1e-12, None)[0];
    let exact = 2.0*(0.5*25f64).exp();

    println!("Result ode54: {}", res1);
    println!("Result ode87: {}", res2);
    println!("Exact: {}", exact);
    println!("");
    println!("Diff ode54: {:e}", ((res1-exact)/exact).abs());
    println!("Diff ode87: {:e}", ((res2-exact)/exact).abs());

    /*spice::furnsh("assets/kernels/kernels.tm");

    let location = spice::vscl(AU, [10.0, 10.0, 10.0]);
    let mut accel = arr1(&[0.0; 3]);

    for &body in BODIES.iter() {
        let et = spice::str2et("2127-MAR-23 16:00:00");

        let body_pos = spice::spkpos(body, et, "J2000", "NONE", "SSB").0;
        let rel_dist = spice::vsub(body_pos, location);

        let unit = arr1(&spice::vhat(rel_dist));
        let mu = spice::bodvrd(body, "GM", 1)[0];

        let mag = spice::vnorm(rel_dist).powi(2);
        println!("{} distance: {} AU, mu: {} km^3/s^2", body, spice::vnorm(body_pos) / AU, mu);

        accel += &(-(mu / mag) * &unit);
    }

    println!("Acceleration at location {:?} is: {} m/s^2", location, accel * 1e3);

    spice::unload("assets/kernels/kernels.tm");*/
}
