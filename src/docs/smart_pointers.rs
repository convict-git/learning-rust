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

use std::{fmt::Display, ops::Deref, rc::Rc};

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

// == Rc<T> The Reference counted Smart Pointer ==
// - Multiple possible owners of the same value (similar to immutable borrows)
// - Useful when can't decide the last ownership at compile time (mostly an espace hatch for lifetimes hell)
// - Only for single-threaded systems

/* Linkedlist:
 *
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
// let nil = Box::new(List::Nil);
// let a = Box::new(List::Cons(4, nil));
// let b = Box::new(List::Cons(3, a));
// let c = Box::new(List::Cons(2, a)); // breaks because a was moved
 */

pub fn check4() {
    // So instead we can keep references here, and also have to specify lifetimes
    enum ListRef<'a> {
        Cons(i32, &'a Box<ListRef<'a>>),
        Nil,
    }
    let nil = Box::new(ListRef::Nil);
    let a = Box::new(ListRef::Cons(4, &nil));
    let b = Box::new(ListRef::Cons(3, &a));
    let c = Box::new(ListRef::Cons(2, &a));
    let d = Box::new(ListRef::Cons(5, &c));
    // This works fine as far as we aren't going to mutate anything (since we are borrowing immutably),
    // Currently it looks trivial because everything in a single scope.
    // But 'a should live long enough, and cases, where you would want to return this data-structure, pass to different threads,
    // This won't scale very well and we will be stuck in lifetime hell.
}

// So we will use Rc<T>
pub fn check5() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    impl Display for List {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let List::Cons(x, next) = self {
                write!(f, "{} -> {}", x, *next)
            } else {
                write!(f, "Nil")
            }
        }
    }

    let a = Rc::new(List::Cons(4, Rc::new(List::Nil)));
    // clone the smart pointer of 'a' for shared reference, and pass it to 'b'
    // NOTE: We should not do a manual a.clone() here, since Rc::clone will also handle the
    // reference counting logic. We should follow their API to keep the invariant maintained
    // Also, one interesting take,
    // when figuring out perf issues, we can safely ignore Rc::clone calls, but not _.clone()
    let b = Rc::new(List::Cons(3, Rc::clone(&a)));
    let c = Rc::new(List::Cons(1, Rc::clone(&a)));

    let print_ref_counts = || {
        println!(
            "a_strong_count: {}\nb_strong_count: {}\nc_strong_count: {}\n",
            Rc::strong_count(&a),
            Rc::strong_count(&b),
            Rc::strong_count(&c),
        );
    };
    println!("a: {}\nb: {}\nc: {}\n", a, b, c);

    {
        let _d = Rc::new(List::Cons(2, Rc::clone(&a)));
        print_ref_counts(); // 4 1 1
    }

    print_ref_counts(); // 3 1 1 // reference count reduce since _d died
}
