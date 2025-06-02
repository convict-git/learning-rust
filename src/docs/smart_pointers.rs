// == Smart pointers ==
/* Rust already has & (references, borrows the value they point to)
 * Smart pointers have additional metadata and functionalities over & and pointers
 * - references just borrow values, smart pointers in most cases owns the value they point to.
 * Eg. String and Vec<_>
 *
 * Box<T> -> lightest smart pointer, storing value of type T, keeping it in heap. only the pointer stays on
 * the stack. No performance overhead. Useful when:
 *
 * - size not known at compile time, but use a value of that type in a context where you need the
 * exact size (like recursive type)
 * - large amount of data, but don't want to copy the data when transfering the ownership
 * - when you want to own a value, but all you can is that type implements certain traits rather
 * than being of a specific type (also known as trait object) (? ToDo: need more explanation)
 *
 * Box implements Deref, and Drop traits -- useful as smart pointers (ToDo: More on this later!)
 * */

pub fn check() {
    let b = Box::new(4);
    // after b's scope end, both the the pointer (on the stack) and the value (on the heap) are dropped

    println!("{:?}", *b);

    let mut n = 5;
    let x = &mut n;
    {
        let _b2 = Box::new(x); // b2 is on stack, value of x is moved.
                               // b2 is dropped, the value of x, i.e. moved mutable reference to n on heap is also dropped
    }
    println!("{}", n); // This still works

    // println!("{}", x); // Error: This breaks value of x was moved out
}

struct MyBox<T>(T); // a generic tuple struct

impl<T> MyBox<T> {
    pub fn new(t: T) -> Self {
        MyBox(t) // still haven't figured out the heap allocation part
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T; // Associated type

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn check2() {
    let x = 5;
    let b = MyBox::new(x);
    // println!("{}", b);
    assert_eq!(5, x);
    assert_eq!(5, *b); //
    let _t = *(b.deref()); // similar to *b
    let c = |x: &str| println!("{}", x);

    let s = String::from("Hello");
    let bs = MyBox::new(s);

    c(&bs); // passes &MyBox<String>, but c expects &str
            // &MyBox<String> -> &String -> &str: rust does this for us, using Deref coercion
            // We have already provided deref for &MyBox<T> to &T
            // Rust provides deref for &String to &str

    let ss = &(*bs)[..]; // if rust didn't give deref for &String to &str
                         // *bs (&String) // & [..] -> str slice for whole string
    c(ss);

    /*
     * NOTE: good part is that all deref is compile time computation from rust compiler hence no runtime cost
     *
     * Also, deref cannot be implemented for more than one Target.
     * Hence, rust compiler has exactly one deref path to try till it reached the desired method parameters.
     *
     * We can also use DerefMut for mutable dereferences, i.e. &mut self -> &mut Self::Target
     *
     * NOTE:
     * &T and &mut T -> can be dereffed to &U, when T: Deref<Target=U>
     * &mut T -> can be dereffed to &mut U, when T: DerefMut<Target=U>
     */
}

// Drop Trait -> what happens to the value when the owner dies (value goes out of the scope)
//
// why implement Drop trait for smart pointers?
// customize what happens to the referenced value when the pointers goes out of the scope,
// like in Box<T>, who will have to free-up the memory allocated on the heap
//
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop for MyBox called for MyBox");
    }
}
/* Some pointers about Drop Trait:
 * we cannot call .drop(). This is enforced since rust anyway calls the drop for the value at the end of its scope (RAII from C++),
 * and it can cause double free error (freeing already freed memory)
 * instead we can use std::mem::drop() for intentional drop (some use cases are freeing up locks when used, and not waiting for it to go out of scope),
 */
pub fn check3() {
    let b = MyBox::<i32>::new(3);

    drop(b); // moves

    // drop(b); // NOTE: You cannot call drop twice! drop moves the value here since MyBox<T>
    // doesn't implement Copy trait

    /*
     * But wait:
     * does it make sense? A struct cannot implement both Copy and Drop trait together by design.
     * Why? First, it's very important to understand the difference between Copy and Clone.
     * Copy is bit-wise copy (i.e. memory is duplicated), it might look fine for primitives but
     * think about a MyTupleStruct(Box<T>). It's copy will copy the value of Box<T> pointer bitwise.
     * Dropping will lead to drop on the same memory ref.
     *
     * Hence rust doesn't allow Copy and Drop trait to be implemented simultaneously
     */

    /*
     * Some ways to drop: (// anything that moves out the value and/or drop due to out of scope or explicit drop)
     * let mut s = String::new();
     *
     * drop(s);
     * (|_| ())(s);
     * { s };
     */
}
