use spice;

use ndarray::{arr1};
use algorithm::*;
use simulation::*;

mod algorithm;
mod simulation;

const BODIES: [&str; 11] = [
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
];

fn main() {
    spice::furnsh("assets/kernels/kernels.tm");

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

    spice::unload("assets/kernels/kernels.tm");
}
