// generics used in function def, structs, enums, method defs

// ============
// function def
// largest is defined for any type that implements teh PartialOrd trait, so that <,>,<=,>= are defined for this type
// how is this different from C++? In C++, you don't have to say what traits T has to follow,
// instead compiler will complain at the usage of larget where the type is not comparable
fn largest<T: std::cmp::PartialOrd>(l: &[T]) -> Option<&T> {
    l.iter().fold(l.first(), |acc, element| match acc {
        Some(current_largest) if current_largest < element => Some(element),
        _ => acc,
    })
}

pub fn test_largest() {
    println!("{}", largest(&["Hello", "world", "A", "b"]).unwrap());
    println!("{}", largest(&[1, 2, u32::max_value()]).unwrap());
    println!("{}", largest::<u32>(&[]).is_none());
}

// ============
// structs
struct Point<T> {
    x: T,
    y: T,
}

struct DPoint<T, U> {
    x: T,
    y: U,
}

pub fn test_structs() {
    let p = Point { x: 2, y: 3 };
    // let q = Point { x: 2, y: 3.2 }; // ERROR mismatch types
    let q = DPoint { x: 2, y: 3.2 }; // works fine, T = i32, U = f64
}

// enums Option<T>, Result<T,E>

// method definitions
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

trait Dist {
    fn get_dist(&self) -> u32;
}

struct OtherPoint;
impl Dist for OtherPoint {
    fn get_dist(&self) -> u32 {
        return 3;
    }
}

impl<T> Point<T> {
    fn get_dist_from_point<U: Dist>(&self, other: &U) -> u32 {
        // fn get_dist_from_point(&self, other: &dyn Dist) -> u32 { // alternate using dyn
        other.get_dist()
    }
}

pub fn test_method_def() {
    let p = Point { x: 2, y: 3 };
    let other_point: OtherPoint = OtherPoint {};
    println!("{}", p.get_dist_from_point(&other_point));
}

// generics don't make your code slower. Monomorphization ensures compiler, generates code for all
// possible types with which your generic is called and uses that instead, result in no runtime
// cost (zero cost abstraction)
