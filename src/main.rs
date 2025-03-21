mod shapes;
use shapes::{collisions::Collidable, shape::Shape};

fn main() {
    let valid_shapes = std::fs::read_to_string("shapes_input")
        .expect("Error reading the file shapes_input")
        .lines()
        .filter_map(|line| line.parse::<Shape>().ok())
        .collect::<Vec<_>>();

    valid_shapes
        .iter()
        .skip(1)
        .zip(valid_shapes.iter().take(valid_shapes.len() - 1))
        .for_each(|(shape_x, shape_y)| {
            if shape_x.collide(shape_y) {
                println!("{} collides with {}", shape_x, shape_y);
            }
        });
}
