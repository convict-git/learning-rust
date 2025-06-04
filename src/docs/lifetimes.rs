// validating references with lifetimes
//
// every reference in a rust has a lifetime

// ERROR: here rust needs the lifetime annotation for the returned value

/*
 // ERROR: This function returns a ref which means a borrowed value:
 // Now it has to be borrowed either from x OR y (OR global?), why?
 // because it cannot return a borrowed value from the current scope as it will outlive it.
 // So if the returned value if borrowed from x OR y, the return value has to be intersection of
 // the lifetimes of x AND y (i.e. the lifetime in which both were valid).
fn longest(x: &str, y: &str) -> &str {
    if x > y {
        x
    } else {
        y
    }
}
*/
// Generic lifetime parameters -> must start with apostrophe (')
//
// declare just like generic types inside <>
// this will take the smaller (intersection) lifetime as 'a
// also, this doesn't mean that returned value has EXACTLY the lifetime 'a, but instead ATLEAST 'a
// This isn't to enforce this lifetime, instead just help the borrow checker invalidate the usage
// which breaks ATLEAST 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn test_longest() {
    let x = String::from("Hello world");
    let z;
    {
        let y = String::from("bye world");
        // remember deref coerc, hence &String -> &str conversion
        z = longest(&x, &y);
        // y is dropped here, hence anything that borrows from y is invalidated after this scope ends
        println!("{z}"); // This is fine
    }

    // println!("{z}"); // ERROR: here z borrows a value which has the lifetime that
    // is same as of y which is already dropped when its scope ended, and z can be possibly
    // holding an invalid reference at this point
}

// Lifetime annotions for structs -> if the value held by the struct is a reference (and NOT owned by the struct itself)
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str, // helds a reference to a str, hence ALWAYS needs a lifetime specifier!
}

impl<'a> ImportantExcerpt<'a> {
    fn get_first_dot_splitted(from: &'a str) -> Option<Self> {
        match from.split('.').next() {
            Some(valid_first_str) => Some(ImportantExcerpt {
                part: valid_first_str,
            }),
            _ => None,
        }
    }
}

pub fn test_lifetime_with_structs() {
    let x = String::from("Hello world. How are you doing?");
    let ie_instance = ImportantExcerpt::get_first_dot_splitted(&x);
    println!("{:?}", ie_instance);
}

pub fn first_word(input: &str) -> &str {
    // let x = String::from("World").as_str(); // ERROR: references a temp value that is dropped as soon as the scope ends
    let x = "Hello"; // but this has 'static lifetime as by default strings in rust have 'static lifetime
                     // 'static -> lives in entire lifetime
    return x;
}

// Three rules for lifetimes elision(omitting):
//  1. Compiler assigns different lifetime parameters to lifetime in the input
//    - fn f(foo: &'_ i32) //  one lifetime paramter by default
//    - fn f(foo: &'_ i32, bar: &'_ f32) //  two lifetime parameters by default
//    - fn f(foo: &'_ ImportantExcerpt<'_>) // NOTE: two lifetime parameters by default
//  2. If exactly, one input lifetime param, than that's used for all output lifetime params as well
//    - fn f<'a>(foo: &'a i32) -> &'a i32
//  3. If multiple input lifetime params, one with self is used for all output lifetime params

// ** Ref ** : https://doc.rust-lang.org/reference/lifetime-elision.html

// Lifetime annotations for method definitions

impl<'a> ImportantExcerpt<'a> {
    // here output lifetime paramter, Option<&'_ str> is allowed to be omitted and is same as of self parameter
    fn strip_prefix_from_excerpt(&self, with_str: &str) -> Option<&str> {
        // Some(with_str) ERROR: was supposted to return the data with lifetime of self
        self.part.strip_prefix(with_str)
    }
}

// Static lifetime
// All string literals (hard-coded strings in the program) have static lifetime, and they live for the entirity of the program
const STR: &'static str = "Hello world";
// NOTE: The above 'static just means that the memory allocated for this reference will never be deallocated
// NOT that it is in static part of the memory region or something!
