fn fn1() {
    println!("Hello, world!");
    // values
    // mutable references, mutable borrowers
    // immutable references, immutable borrowers

    // () unit, nothing -> since everything in rust is a statement
    let x: Vec<_> = vec![1, 2, 3].iter().map(|x| x + 1).collect();

    // need mutable because to make changes in vec_iter, (like internal index) when moving
    // but here vec![1, 2, 3] is temporary and lives till the end of the statement. If we need the
    // iterator to point to a valid vec which stays in the memory, we need to ensure the lifetime
    // of the vector is held till all the usages of the iterator otherwise the borrow check will
    // complain
    // let mut vec_iter = vec![1, 2, 3].iter().map(|x| x + 1);
    let vector = vec![1, 2, 3]; //
    let mut vec_iter = vector.iter().map(|x| x + 1);

    // empty and mutable new_vec
    let mut new_vec = vec![];

    // pattern matching
    while let Some(x) = vec_iter.next() {
        new_vec.push(x);
    }

    println!("{:?}", x);
}

fn read_from_file() {
    match std::fs::read_to_string("lines") {
        // arms of match should produce a single consistent type because match is
        // expected to resolve to a single type
        //
        // Also `lines` has type `pub fn lines(&self) -> Lines<'_>`
        // <'_> is a lifetime annotation.
        // The underscore ('_) is a shorthand for an anonymous lifetime, indicating that the lifetime
        // of the returned Lines iterator is tied to the lifetime of the &self reference.
        // This means that the Lines iterator cannot outlive the borrowed reference to self.
        //
        // Lines is an iterator struct
        Ok(file_string) => {
            // file_string.lines().for_each(|line| println!("{}", line));
            file_string
                .lines()
                .enumerate()
                .filter(|(idx, _)| idx % 2 == 0)
                .for_each(|(_, line)| println!("{}", line));
        }
        Err(e) => {
            println!("Error : {}", e);
        }
    }
}

enum Status {
    Pending,
    Completed,
}

impl Status {
    fn is_pending(&self) -> bool {
        // this `if let` is with pattern matching without binding
        if let Status::Pending = self {
            return true;
        } else {
            return false;
        }
    }
}

fn check_enums() {
    let pending_status = Status::Pending;
    let completed_status = Status::Completed;

    if pending_status.is_pending() {
        println!("This is a pending status");
    };

    if !completed_status.is_pending() {
        println!("This is a NOT pending status");
    }
}

// union types way in rust
enum Item {
    Number(usize),
    String(String),
    Vec(Vec<usize>),
}

// implicit pattern matching - matches patter based on context, value, ref, mut ref, owned value
// automatic dereferencing -
//
// match item:
//      &Item::Text(ref t) => println!("Text: {}", t),
//      // item matches a reference to Item::Text and
//      // ref t, tells rust to create a reference to a value
//
// For types that implement the Copy trait, like usize, the value is copied rather than moved.
//  so Item::Number(x) => println!("Number: {}", x) // here x is copied and not moved
impl Item {
    fn print(&self) {
        // we use match self and not match *self (can do that as well),
        // because rust automatically derefernces the reference for us through the context
        match self {
            // pattern matching with binding
            Item::Number(x) => println!("A number: {}", x),
            Item::String(s) => println!("A string: {}", s),
            Item::Vec(v) => {
                println!("A vector: {:?}", v);
                // here v was &Vec<usize>, hence only borrowed in println and is usable afterwards
                let _x = v.iter();
            }
        }
    }
}

// passing a mutable reference of vector of Item
fn mutate_items(items: &mut Vec<Item>) {
    items.push(Item::String("Hello world".to_string())); // you need to make &str -> String conversation
}

fn test_items() {
    let mut items: Vec<Item> = vec![Item::Number(2)];
    mutate_items(&mut items);
    items.iter().for_each(|item| item.print());
}

// enum Option<T> = { Some(T), None }
// enum Result<V, E> = { Ok(T), Err(E) }

fn multiply(nums: &Vec<usize>, index: usize) -> usize {
    return nums.get(index).unwrap_or(&index) * 5;
    // if let Some(x) = nums.get(index) {
    //     return x * 5; }
    // return index * 5;
}

fn test_multiply() {
    let x = multiply(&vec![1, 2, 3], 2);
    println!("{}", x);
}

fn numbers_from_files() {
    let file_name = std::env::args()
        .nth(1)
        .expect("File as argument not provided");

    let file = std::fs::read_to_string(&file_name)
        .expect(&format!("Unable to read the file {}", &file_name));

    file.lines().for_each(|line| {
        // use turbo fish syntax to let parse know in what to parse to
        if let Ok(x) = line.parse::<usize>() {
            println!("{}", &x);
        }
        println!("{} Not a number", &line);
    });
}

// borrow-checker - rules:
// 1. only ONE value owner - one OWNER
// 2. UNLIMITED immutable borrows(ref) and NO mutable reference - MANY READERS and no WRITER
// 3. ONE mutable ref and NO immutable ref - one WRITER and no READER
// 4. A ref cannot outlive its value - no READER or WRITER if value is out of lifetime

// test
#[derive(Debug)]
struct ItemX {
    count: usize,
}

fn add_one(item: &mut ItemX) {
    item.count += 1;
}

fn test_borrow1() {
    let mut item = ItemX { count: 0 };
    println!("{:?}", item);
    add_one(&mut item);
    println!("{:?}", item);
}

fn print_all(items: &Vec<ItemX>) {
    items.iter().for_each(|item| println!("{:?}", item));
}

fn test_borrow2() {
    let mut items: Vec<ItemX> = vec![ItemX { count: 0 }];
    let mut default_item = ItemX { count: 1 };
    let mut first = items.get_mut(0).unwrap_or(&mut default_item);
    println!("{:?}", first);
    print_all(&items); // if the `first` is NOT used below this, we can still print_all
                       // i.e. pass a immutable ref to print_all while a mut ref is defined in the
                       // scope because the mutable ref's life ended before

    // println!("{:?}", first);
}

mod shapes;
use shapes::{area::Area, circle::Circle, collisions::Collidable, rect::Rect};

fn test_traits() {
    let rect = Rect::default();

    let circ = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };
    println!("{}, {}", rect, rect.area());
    println!("{}", circ.area());
}

// Cool thing about traits! No global scoping
// use shapes::{Circle, Rect, Area}
// If trait comes from another module, unless imported the trait isn't implemented on the structs
// i.e. I cannot call circ.area() even if the trait Area was implemented in the imported module,
// unless I import Area trait itself too
//
// Crate is a compilable unit of rust
// also when used in "use crate::", is the root module of the current crate

fn main() {
    let circ = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };
    let rect = Rect::default();
    let rect2 = Rect {
        x: 10.0,
        y: 12.0,
        height: 100.0,
        width: 200.0,
    };
    println!("{}", rect.collide(&rect2));
    println!("{}", rect.collide(&circ));
}
