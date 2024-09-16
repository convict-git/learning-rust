mod collections;
mod docs;
mod errors;
mod shapes;
/*
use shapes::{collisions::Collidable, shape::Shape};

fn check_collisions() {
    let valid_shapes = std::fs::read_to_string("inputs/shapes_input")
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
*/

use crate::{
    collections::{
        hash_maps::{
            basics as hash_map_basics, ref_lifetime_vague as hash_map_ref_lifetime_value,
            update_value,
        },
        strings::{add_assign, deref_coercion, indexing},
        vectors::{try_outs_1, try_outs_2},
    },
    docs::{
        generics::test_largest,
        guessing_game::play,
        lifetimes::{first_word, test_lifetime_with_structs, test_longest},
        match_docs::push_down_reference,
        packages_crates::eat_at_rest,
    },
    errors::recoverable::{
        propagating_error_basic, propagating_errors, propagating_errors_with_option,
    },
};

/*
pub fn read_from_file() {
    // match propagating_error_basic("inputs/example_file") {
    //     Ok(s) => println!("{}", s),
    //     Err(e) => println!("{}", e),
    // }
    match propagating_errors_with_option("inputs/example_file") {
        Some(s) => println!("{}", s),
        None => println!("Some error occurred"),
    }
}
*/

fn main() {
    println!("{}", first_word(&String::from("World")));
}
