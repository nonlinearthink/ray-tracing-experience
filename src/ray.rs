extern crate nalgebra as na;

use na::{Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        return Ray { origin, direction };
    }

    pub fn at(&mut self, time: f64) -> Point3<f64> {
        return self.origin + time * self.direction;
    }
}
