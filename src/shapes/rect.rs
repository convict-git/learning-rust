use super::{
    area::Area,
    collisions::{Contains, Points, PointsIter},
};
use std::{fmt::Display, str::FromStr};

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
}

impl FromStr for Rect {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(anyhow::anyhow!("Badly formed rectangle"));
        }

        return Ok(Rect {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            height: parts[2].parse()?,
            width: parts[3].parse()?,
        });
    }
}

impl Contains for Rect {
    fn contains_point(&self, (x, y): (f32, f32)) -> bool {
        return self.x <= x && self.width + self.x >= x && self.y <= y && self.height + self.y <= y;
    }
}

impl Points for Rect {
    fn points(&self) -> PointsIter {
        return vec![
            (self.x, self.y),
            (self.x, self.y + self.height),
            (self.x + self.width, self.y),
            (self.x + self.width, self.y + self.height),
        ]
        .into();
    }
}

impl Area for Rect {
    fn area(&self) -> f32 {
        return self.height * self.width;
    }
}

// implementing some foreign traits like Default and Display for Rect
impl Default for Rect {
    // static method, doesn't require &self
    fn default() -> Self {
        return Rect {
            height: 10.0,
            width: 30.0,
            x: 0.0,
            y: 0.0,
        };
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Rect: ({}, {}): ({}x{})",
            self.x, self.y, self.height, self.width
        );
    }
}

/*
// iterators
pub struct RectIter {
    points: Vec<(f32, f32)>,
    idx: usize,
}

// implementing the trait Iterator for RectIter,
/*
   pub trait Iterator {
       type Item;
       fn next(&mut self) -> Option<Self::Item>;
   }
*/
impl Iterator for RectIter {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.points.get(self.idx) {
            Some(point) => {
                self.idx += 1;
                return Some(*point);
            }
            _ => None,
        }
    }
}

impl From<&Rect> for RectIter {
    fn from(rect: &Rect) -> Self {
        return RectIter {
            points: vec![
                (rect.x, rect.y),
                (rect.x, rect.y + rect.height),
                (rect.x + rect.width, rect.y),
                (rect.x + rect.width, rect.y + rect.height),
            ],
            idx: 0,
        };
    }
}

/*
   pub trait IntoIterator {
       type Item;
       type IntoIter;
       fn into_iter(self) -> Self::IntoIter;
   }
*/
impl IntoIterator for Rect {
    type Item = (f32, f32);
    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return (&self).into();
    }
}

impl IntoIterator for &Rect {
    type Item = (f32, f32);
    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return self.into();
    }
}

impl Collidable<Rect> for Rect {
    fn collide(&self, other: &Rect) -> bool {
        return other.into_iter().any(|(x, y)| self.contains_point((x, y)));
    }
}

impl Collidable<Circle> for Rect {
    fn collide(&self, other: &Circle) -> bool {
        return self.contains_point((other.x, other.y));
    }
}
*/
