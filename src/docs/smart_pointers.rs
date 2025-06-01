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
        let b2 = Box::new(x); // b2 is on stack, value of x is moved.
                              // b2 is dropped, the value of x, i.e. moved mutable reference to n on heap is also dropped
    }
    println!("{}", n); // This still works

    // println!("{}", x); // Error: This breaks value of x was moved out
}
