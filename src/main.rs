mod collections;
mod docs;
mod shapes;
/*
use shapes::{collisions::Collidable, shape::Shape};

fn check_collisions() {
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
        closures::check_b, guessing_game::play, match_docs::push_down_reference,
        packages_crates::eat_at_rest, smart_pointers::check2,
    },
};

fn main() {
    // update_value();
    check2();
}
