use std::{f32::consts::PI, fmt::Display, str::FromStr};

use super::{
    area::Area,
    collisions::{Contains, Points},
};

pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle ({}, {}) {}", self.x, self.y, self.radius)
    }
}

impl FromStr for Circle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Badly formed circle"));
        }

        return Ok(Circle {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            radius: parts[2].parse()?,
        });
    }
}

impl Contains for Circle {
    fn contains_point(&self, (x, y): (f32, f32)) -> bool {
        return (x - self.x) * (x - self.x) + (y - self.y) * (y - self.y)
            <= self.radius * self.radius;
    }
}

impl Points for Circle {
    fn get_points_iter(&self) -> super::collisions::PointsIter {
        return vec![(self.x, self.y)].into();
    }
}

impl Area for Circle {
    fn area(&self) -> f32 {
        return self.radius * self.radius * PI;
    }
}
