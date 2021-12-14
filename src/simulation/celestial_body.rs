use ndarray::{Array1, arr1};
use std::collections::HashMap;

pub struct CelestialBody {
    pub mu: f64,
    pub position: Array1<f64>
}

impl CelestialBody {
    pub fn zero() -> Self {
        Self {
            mu: 0.0,
            position: arr1(&[0.0, 0.0, 0.0])
        }
    }
}

pub type Bodies<'a> = HashMap<&'a str, CelestialBody>;