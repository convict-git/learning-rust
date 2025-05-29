pub mod back_of_house {
    // in structs, each fields are still private unless explicitly mentioned
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub struct Point(pub i32, i32); // can't access p.1 outside

    // whereas, in enums, once enum is public, all the variants are public as well
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast_name: &str) -> Breakfast {
            // static method as it doesn't take self
            Breakfast {
                toast: String::from(toast_name),
                seasonal_fruit: String::from("Apple"),
            }
        }
    }
}

// Re-exporting as different name
// (in rust terms, public re-export with an alias, or also known as API forwarding)
pub use back_of_house::Appetizer as HouseAppetizer;

/* NOTE: use -> importing a name into local namespace
 * pub use -> re-exporting that name from your module, so others can import */

pub fn eat_at_rest() {
    let breakfast = back_of_house::Breakfast::summer("brown");
    println!("{:?}", breakfast);
    // println!("{}", breakfast.seasonal_fruit) // this will break since seasonal_fruit is a private field

    // let initial_order = back_of_house::Appetizer::Soup; // OR
    let initial_order = HouseAppetizer::Soup;
    println!("{:?}", initial_order);
}

/*
// use are scoped
use back_of_house::Breakfast; // local namespace

mod some_mod { // some_mod namespace
   // here you cannot use Breakfast directly, because Breakfast is valid in local namespace but not
   // some_mod namespace
}
*/

/*
use std::io;
use std::io::Write;

// merge these two:
use std::io::{self, Write};

// include all :
use std::collections::*;
 */
