use super::{
    area::Area,
    collisions::{Contains, Points},
};
use std::f32::consts::PI;

pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl Contains for Circle {
    fn contains_point(&self, (x, y): (f32, f32)) -> bool {
        return (x - self.x) * (x - self.x) + (y - self.y) * (y - self.y)
            <= self.radius * self.radius;
    }
}

impl Points for Circle {
    fn points(&self) -> super::collisions::PointsIter {
        return vec![(self.x, self.y)].into();
    }
}

impl Area for Circle {
    fn area(&self) -> f32 {
        return self.radius * self.radius * PI;
    }
}
