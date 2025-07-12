pub trait Collidable<T> {
    fn collide(&self, other: &T) -> bool;
    fn collides(&self, others: &[T]) -> bool {
        return others.iter().any(|other| self.collide(other));
    }
}

pub struct PointsIter {
    points: Vec<(f32, f32)>,
    idx: usize,
}

impl Iterator for PointsIter {
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

impl From<Vec<(f32, f32)>> for PointsIter {
    fn from(points: Vec<(f32, f32)>) -> Self {
        return PointsIter { points, idx: 0 }; // short-hand works here too!
    }
}

pub trait Points {
    fn get_points_iter(&self) -> PointsIter;
}

pub trait Contains {
    fn contains_point(&self, point: (f32, f32)) -> bool;
}

// implementing generics
impl<V, T> Collidable<T> for V
where
    V: Contains,
    T: Points,
{
    fn collide(&self, other: &T) -> bool {
        return other
            .get_points_iter()
            .any(|point| self.contains_point(point));
    }
}
