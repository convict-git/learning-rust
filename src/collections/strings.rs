// dereference and type coercion
pub fn deref_coercion() {
    let s_empty = String::new();
    let mut s1 = String::from("Hello");
    s1.push_str(" from the other side!"); // s1's ownership mutably borrowed, passed &str

    let s2 = String::from(" I must've called a thousand times");
    // s1.push_str(s2); // ERROR: because s2 is String and expected &str

    let s2_ref = &s2;
    /* but we can pass s2_ref which is &String even when expected &str, why?
    This is because String struct implements Deref trait
    which allows String to be dereferenced to some other type
    impl Deref for String {
       type Target = str;
       fn deref(&self) -> &Self::Target {
           &self[..] // slice of the whole string
       }
    }
    */
    s1.push_str(s2_ref);
    s1.push('.');
    println!("{s1}");
}

pub fn add_assign() {
    let mut s1 = String::from("Hello");

    s1 += " my fascist "; // allowed, &str

    // s1 += String::from("world"); // ERROR! won't work
    s1 += &String::from(" again!"); // passed &String derefed to &str

    /* why += works? because String implements AddAssign trait:
     trait AddAssign<Rhs = Self> { // default type value is Self for Rhs
         fn add_assign(&mut self, rhs: Rhs);
     }

    impl AddAssign<&str> for String {
        fn add_assign(&mut self, other: &str) {
            self.push_str(other);
        }
    } */
    println!("{s1}");
}

/*
 * NOTE: You cannot do this! Only the traits defined in the current crate can be implemented for
 * the types defined outside the crate
 *
use std::ops::AddAssign;
impl AddAssign<String> for String {
    fn add_assign(&mut self, rhs: String) {
        self.push_str(&rhs);
    }
}
*/

pub fn indexing() {
    let s = String::from("Hello World");
    for c in s.chars() {
        // s is imm borrowed
        println!("{c}");
    }
}
