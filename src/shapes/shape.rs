use std::{fmt::Display, str::FromStr};

use super::{
    circle::Circle,
    collisions::{Contains, Points},
    rect::Rect,
};

pub enum Shape {
    Rect(Rect),
    Circle(Circle),
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Rect(r) => write!(f, "{}", r),
            Shape::Circle(c) => write!(f, "{}", c),
        }
    }
}

// step1: to be able to read the shapes from the file

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // match the string for shape, rest of the input provide to respective impl of FromStr
        let (shape_info, data) = s.split_once(" ").expect("Incorrect file format");
        match shape_info {
            "rect" => Ok(Shape::Rect(Rect::from_str(data)?)),
            "circle" => Ok(Shape::Circle(Circle::from_str(data)?)),
            _ => Err(anyhow::anyhow!("Invalid shape info")),
        }
    }
}

// step2: find collisions between adjacent shapes
// for Collidable trait to work for shapes, we can implement Points and Contains

impl Points for Shape {
    fn get_points_iter(&self) -> super::collisions::PointsIter {
        match self {
            Shape::Rect(r) => r.get_points_iter(),
            Shape::Circle(c) => c.get_points_iter(),
        }
    }
}

impl Contains for Shape {
    fn contains_point(&self, point: (f32, f32)) -> bool {
        match self {
            Shape::Rect(r) => r.contains_point(point),
            Shape::Circle(c) => c.contains_point(point),
        }
    }
}
