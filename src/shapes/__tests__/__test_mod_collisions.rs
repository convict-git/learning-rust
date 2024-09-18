#[cfg(test)]
pub mod tests {
    use crate::shapes::{collisions::Collidable, shape::Shape};

    #[test]
    fn check_collisions() {
        let valid_shapes = std::fs::read_to_string("src/shapes/__tests__/__fixtures__shapes_input")
            .expect("Error reading the file shapes_input")
            .lines()
            .filter_map(|line| line.parse::<Shape>().ok())
            .collect::<Vec<_>>();

        let expected_output_file_content =
            std::fs::read_to_string("src/shapes/__tests__/__fixtures__shapes_output")
                .expect("Error reading output file");

        valid_shapes
            .iter()
            .skip(1)
            .zip(valid_shapes.iter().take(valid_shapes.len() - 1))
            .zip(expected_output_file_content.lines())
            .for_each(|((shape_x, shape_y), expected_output)| {
                assert_eq!(
                    expected_output,
                    if shape_x.collide(shape_y) {
                        "YES"
                    } else {
                        "NO"
                    },
                    "checking if {} and {} collides, and expection is {}",
                    shape_x,
                    shape_y,
                    expected_output
                );
            });
    }
}
