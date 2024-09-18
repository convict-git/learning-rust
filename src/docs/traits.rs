// "Orphan rule": You cannot implement an external trait for an external type

// Traits as parameters
trait Area {
    fn get_area(&self) -> f32;
}

fn get_volume(len: f32, obj_with_area: &impl Area) -> f32 {
    // mostly you will be using generics instead (see trait bound syntax somewhere below!)
    len * obj_with_area.get_area()
}

struct Rect {
    height: f32,
    widght: f32,
}

impl Area for Rect {
    fn get_area(&self) -> f32 {
        return self.height * self.widght;
    }
}

pub fn test_traits_as_params() {
    let r = Rect {
        height: 3.2,
        widght: 4.5,
    };

    println!("{}", get_volume(2f32, &r));
}

// trait bound syntax
fn get_volume_bound<T: Area>(len: f32, obj_with_area: &T) -> f32 {
    len * obj_with_area.get_area()
}

// multiple-traits bound syntax
trait Logger {
    fn log(&self);
}

fn log_area<T: Area + Logger>(obj: &T) {
    obj.log()
}

// Clearer trait bounds with where clauses (alternative syntax)
use std::fmt::Display;
// fn some_function<T: Clone, U: Display + Clone>(t: &T, u: &U) -> u32 {} // OR
fn some_function<T, U>(t: &T, u: &U) -> u32
where
    T: Display + Clone,
    U: Clone,
{
    42
}

// see collisions.rs for nicer examples
