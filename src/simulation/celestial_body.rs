use ndarray::{Array1, arr1};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CelestialBody {
    pub mu: f64,
    pub radii: [f64; 3],
    pub position: Array1<f64>
}

impl CelestialBody {
    pub fn empty() -> Self {
        Self {
            mu: 0.0,
            radii: [0.0; 3],
            position: arr1(&[0.0; 3])
        }
    }
}

pub type Bodies<'a> = HashMap<&'a str, CelestialBody>;