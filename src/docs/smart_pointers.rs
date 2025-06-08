#[cfg(test)]
mod tests {
    mod basics {
        // == Smart pointers ==
        /* Rust already has & (references, borrows the value they point to)
         * Smart pointers have additional metadata and functionalities over & and pointers
         * - references just borrow values, smart pointers in most cases owns the value they point to.
         * Eg. String and Vec<_>
         *
         * == Box ==
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
        #[test]
        fn test() {
            let b = Box::new(4);
            // after b's scope end, both the the pointer (on the stack) and the value (on the heap) are dropped

            println!("{:?}", *b);

            let mut n = 5;
            let x = &mut n;
            {
                let _b2 = Box::new(x); // b2 is on stack, value of x is moved.
                                       // b2 is dropped, the value of x, i.e. moved mutable reference to n on heap is also dropped
            }
            // println!("{}", x); // Error: This breaks value of x was moved out
            assert_eq!(n, 5); // This still works
        }
    }

    mod my_box {
        use std::ops::Deref;

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

        #[test]
        fn test_deref() {
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
        #[test]
        fn test_drop() {
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
    }

    mod reference_counted_sp {
        // == Rc<T> The Reference counted Smart Pointer ==
        // - Multiple possible owners of the same value (similar to immutable borrows)
        // - Useful when can't decide the last ownership at compile time (mostly an espace hatch for lifetimes hell)
        // - Only for single-threaded systems

        use std::{fmt::Display, rc::Rc};

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
        #[test]
        fn test_box_ref() {
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
        #[test]
        fn test_rc() {
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

            let get_ref_counts = || {
                [
                    Rc::strong_count(&a),
                    Rc::strong_count(&b),
                    Rc::strong_count(&c),
                ]
            };
            assert_eq!(get_ref_counts(), [3, 1, 1]);
            {
                let _d = Rc::new(List::Cons(2, Rc::clone(&a)));
                assert_eq!(get_ref_counts(), [4, 1, 1]); // increased strong ref count for a
            }
            assert_eq!(get_ref_counts(), [3, 1, 1]); // reference count reduce since _d died
        }
    }

    mod interior_mutability {
        // == Interior Mutability Pattern == (allow mutation inside an immutable value)
        /* allow mutating the value, even when the value is borrowed immutably, against the rust's
         * borrowing rules (unsafe code behind the scenes but giving safe APIs).
         * Ensure the checker rules are manually handled by us correctly, since the borrowing rules
         * are checked at runtime (instead of compile-time) and program will panic if rules are breached.
         * Static Analysis of the rust compiler is conservative and more restrictive and might reject
         * safe programs but never accept any unsafe program.
         */

        /* smart pointers:
         * // compile-time borrow checker:
         * Box<T> (value on heap, pointer on stack, no perf overhead, data size not known at compile time,
         * both mut/immutable borrow),
         * Rc<T> (reference counted, like immutable borrow but with multiple owners, NO interior mutability, single threaded),
         * // run-time borrow checker:
         * RefCell<T> (reference counted, interior mutability, single threaded, mut/immutable borrows at
         * runtime, single owner)
         */
        mod limit_tracker {
            pub trait Messenger {
                fn send(&self, msg: &str);
            }

            pub struct LimitTracker<'a, T: Messenger> {
                messenger: &'a T,
                value: usize,
                max: usize,
            }

            impl<'a, T> LimitTracker<'a, T>
            where
                T: Messenger,
            {
                pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
                    LimitTracker {
                        messenger,
                        value: 0,
                        max,
                    }
                }

                pub fn set_value(&mut self, value: usize) {
                    self.value = value;

                    let percentage_of_max = self.value as f64 / self.max as f64;

                    if percentage_of_max >= 1.0 {
                        self.messenger.send("Error: You are over your quota!");
                    } else if percentage_of_max >= 0.9 {
                        self.messenger
                            .send("Urgent warning: You've used up over 90% of your quota!");
                    } else if percentage_of_max >= 0.75 {
                        self.messenger
                            .send("Warning: You've used up over 75% of your quota!");
                    }
                }
            }
        }

        use std::cell::RefCell;

        use limit_tracker::{Messenger, *};
        struct MockMessenger {
            // sent_messages: Vec<String>,
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> Self {
                MockMessenger {
                    // sent_messages: vec![],
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, msg: &str) {
                // self.sent_messages.push(String::from(msg));
                // // since self is borrowed immutably, we can't push, hence we need interior mutability.
                self.sent_messages.borrow_mut().push(String::from(msg));
            }
        }
        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
            limit_tracker.set_value(80);

            // assert_eq!(mock_messenger.sent_messages.len(), 1);
            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        }
    }

    mod list_with_rc_refcell {
        // == Rc<RefCell<T>> ==, Multiple owners of mutable data
        /* Rc<T> -> provides immutable borrows with multiple owners
         * RefCell<T> -> provides interior mutability (mutability to immutable value)
         *
         * so we can get multiple owners mutability access to an immutable value using Rc<RefCell<T>>
         */

        use std::{cell::RefCell, rc::Rc};

        // Aim: List which has mutability on both values and structure
        enum List {
            // Cons(i32, Box<List>), // this would break when value will move out
            // Cons(i32, Rc<List>), // This does allow multiple owners but only immutable borrows
            Cons(i32, Rc<RefCell<List>>),
            Nil,
        }

        impl List {
            fn get_wrapped_nil() -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(Self::Nil))
            }

            fn get_wrapped_list(value: i32, next: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
                // value is copied, and we Rc clone the SP given through next
                Rc::new(RefCell::new(Self::Cons(value, Rc::clone(next))))
            }

            fn update_next(&mut self, new_next: &Rc<RefCell<Self>>) -> Result<(), anyhow::Error> {
                match self {
                    Self::Cons(_, ref_cell) => {
                        // - clone the Rc and use it
                        // - drop the existing Rc (otherwise memory leak) -- happens automatically RAII
                        *ref_cell = Rc::clone(new_next);
                        Ok(())
                    }
                    Self::Nil => Err(anyhow::anyhow!("Can't update_next on Nil instance of List")),
                }
            }

            fn update_value(&mut self, new_value: i32) -> Result<(), anyhow::Error> {
                match self {
                    Self::Cons(old_value, _) => {
                        *old_value = new_value;
                        Ok(())
                    }
                    Self::Nil => Err(anyhow::anyhow!(
                        "Can't update_value on Nil instance of List"
                    )),
                }
            }
        }

        impl Drop for List {
            fn drop(&mut self) {
                println!(
                    "Drop called for {}",
                    if let List::Cons(value, _) = self {
                        value.to_string()
                    } else {
                        String::from("Nil")
                    }
                );
            }
        }

        #[test]
        fn test() {
            let nil = List::get_wrapped_nil();
            let a = List::get_wrapped_list(1, &nil);
            let b = List::get_wrapped_list(2, &a);
            let c = List::get_wrapped_list(3, &a);
            let get_ref_counts = || {
                [
                    Rc::strong_count(&a),
                    Rc::strong_count(&b),
                    Rc::strong_count(&c),
                ]
            };

            // b -> a <- c
            // change it to b <- a <- c, i.e. b's next to nil, a's next to b
            assert_eq!(get_ref_counts(), [3, 1, 1]);
            (*b).borrow_mut().update_next(&nil);
            assert_eq!(get_ref_counts(), [2, 1, 1]);
            (*a).borrow_mut().update_next(&b);
            assert_eq!(get_ref_counts(), [2, 2, 1]);
            (*c).borrow_mut().update_value(3);
        }
    }

    mod list_with_refcell_rc {
        use std::{cell::RefCell, rc::Rc};

        // better way, RefCell<Rc<List>>
        enum List {
            Cons(RefCell<i32>, RefCell<Rc<List>>),
            Nil,
        }

        impl List {
            fn update_value(&self, next_value: i32) -> Result<(), anyhow::Error> {
                match self {
                    Self::Cons(value, _) => {
                        *value.borrow_mut() = next_value;
                        Ok(())
                    }
                    Self::Nil => Err(anyhow::anyhow!(
                        "Can't update_value on Nil instance of List"
                    )),
                }
            }

            fn update_next(&self, new_next: &Rc<Self>) -> Result<(), anyhow::Error> {
                match self {
                    Self::Cons(_, ref_cell) => {
                        *ref_cell.borrow_mut() = Rc::clone(new_next);
                        // this should drop the existing ref_cell
                        Ok(())
                    }
                    Self::Nil => Err(anyhow::anyhow!("Can't update_next on Nil instance of List")),
                }
            }

            fn get_wrapped_list(value: i32, next: &Rc<Self>) -> Rc<Self> {
                Rc::new(Self::Cons(
                    RefCell::new(value),
                    RefCell::new(Rc::clone(next)),
                ))
            }

            fn get_wrapped_nil() -> Rc<List> {
                Rc::new(Self::Nil)
            }
        }

        impl Drop for List {
            fn drop(&mut self) {
                println!(
                    "Drop called for List with value {}",
                    if let Self::Cons(value, _) = self {
                        (*value).borrow().to_string()
                    } else {
                        String::from("Nil")
                    }
                );
            }
        }

        #[test]
        fn test() {
            let nil = List::get_wrapped_nil();

            let a = List::get_wrapped_list(1, &nil);
            let b = List::get_wrapped_list(2, &a);
            let c = List::get_wrapped_list(3, &a);

            let get_ref_counts = || {
                [
                    Rc::strong_count(&a),
                    Rc::strong_count(&b),
                    Rc::strong_count(&c),
                ]
            };

            // b -> a <- c
            // change it to b <- a <- c, i.e. b's next to nil, a's next to b
            assert_eq!(get_ref_counts(), [3, 1, 1]);
            (*b).update_next(&nil);
            assert_eq!(get_ref_counts(), [2, 1, 1]);
            (*a).update_next(&b);
            assert_eq!(get_ref_counts(), [2, 2, 1]);
            (*c).update_value(4);
        }
    }

    mod directed_tree_node_with_refcell_rc {
        use std::{cell::RefCell, rc::Rc};

        // TreeNode
        struct TreeNode<T> {
            value: RefCell<T>,
            children: RefCell<Vec<Rc<TreeNode<T>>>>,
        }

        impl<T> TreeNode<T> {
            fn new(value: T) -> Self {
                Self {
                    value: RefCell::new(value),
                    children: RefCell::new(vec![]),
                }
            }

            fn add_child(&self, child: &Rc<TreeNode<T>>) {
                (*self.children.borrow_mut()).push(Rc::clone(child));
            }

            fn change_value(&self, value: T) {
                *self.value.borrow_mut() = value;
            }
        }

        #[test]
        fn tree_nodes_have_right_strong_counts() {
            let vertices_rc = (1..=6)
                .map(|value| Rc::new(TreeNode::new(value)))
                .collect::<Vec<_>>();

            let edges = [(1, 2), (1, 5), (2, 3), (2, 4), (5, 6)];

            edges.iter().for_each(|(u, v)| -> () {
                match vertices_rc.get(u - 1) {
                    Some(rc_u) => match vertices_rc.get(v - 1) {
                        Some(rc_v) => {
                            rc_u.add_child(rc_v);
                        }
                        None => todo!(),
                    },
                    None => todo!(),
                }
            });

            let strong_counts = vertices_rc
                .iter()
                .map(|rc_u| Rc::strong_count(rc_u))
                .collect::<Vec<usize>>();

            assert_eq!(strong_counts, vec![1, 2, 2, 2, 2, 2]);
        }
    }

    mod weak_pointers {
        use std::{
            cell::RefCell,
            rc::{Rc, Weak},
        };

        // Creating ref cycles: Generally due nested use of interior mutability and reference counters
        //
        // == Preventing ref cycles using weak references (Weak<T>) ==
        // Rc::clone : share ownership using strong references
        // (strong_count needs to be 0 for drop)
        //
        // Rc::downgrade(&self) -> Weak<T> : doesn't express an ownership using weak references
        // (weak_count, doesn't need to be 0 for drop)
        //
        // but ofcourse, that means we need to check manually if value through weak reference is NOT
        // dropped. This can be done using rc::Weak::upgrade(&self<T>) -> Option<Rc<T>>

        enum Parent<T> {
            Yes(T),
            No,
        }

        struct TreeNode<T> {
            pub value: RefCell<T>,
            /* node -> children should be strong reference, should reduce strong count of children if parent node drops */
            pub children: RefCell<Vec<Rc<TreeNode<T>>>>,
            /* node -> parent, should be a weak reference, even if node is dropped,
             * parent's strong count shouldn't change, instead just the weak count should */
            pub parent: RefCell<Parent<Weak<TreeNode<T>>>>,
        }

        impl<T> TreeNode<T> {
            pub fn new(value: T) -> Self {
                Self {
                    value: RefCell::new(value),
                    children: RefCell::new(vec![]),
                    parent: RefCell::new(Parent::No),
                }
            }

            fn add_child(&self, child: &Rc<TreeNode<T>>) {
                (*self.children.borrow_mut()).push(Rc::clone(child));
            }

            fn add_parent(&self, parent: &Rc<TreeNode<T>>) {
                *self.parent.borrow_mut() = Parent::Yes(Rc::downgrade(parent));
            }

            fn change_value(&self, value: T) {
                *self.value.borrow_mut() = value;
            }

            pub fn join(parent: &Rc<TreeNode<T>>, child: &Rc<TreeNode<T>>) {
                (*parent).add_child(child);
                (*child).add_parent(parent);
            }
        }

        impl<T: Copy> TreeNode<T> {
            pub fn get_values_till_root(&self) -> Vec<T> {
                let mut v = vec![*self.value.borrow()];

                if let Parent::Yes(ref p) = *self.parent.borrow() {
                    if let Some(rc_p) = &p.upgrade() {
                        v.append(&mut rc_p.get_values_till_root().clone());
                    }
                }
                v
            }
        }

        #[test]
        fn test_strong_and_weak_counters() {
            let vertices_rc = (1..=6)
                .map(|value| Rc::new(TreeNode::new(value)))
                .collect::<Vec<_>>();

            let edges = [(1, 2), (1, 5), (2, 3), (2, 4), (5, 6)];

            edges.iter().for_each(|(u, v)| -> () {
                match vertices_rc.get(u - 1) {
                    Some(rc_u) => match vertices_rc.get(v - 1) {
                        Some(rc_v) => {
                            TreeNode::join(rc_u, rc_v);
                        }
                        None => todo!(),
                    },
                    None => todo!(),
                }
            });

            let get_strong_and_weak_counts = || {
                vertices_rc
                    .iter()
                    .map(|rc_u| (Rc::strong_count(rc_u), Rc::weak_count(rc_u)))
                    .collect::<Vec<(usize, usize)>>()
            };

            assert_eq!(
                get_strong_and_weak_counts(),
                vec![(1, 2), (2, 2), (2, 0), (2, 0), (2, 1), (2, 0)]
            );

            {
                let rc_7 = Rc::new(TreeNode::new(7));
                match vertices_rc.get(5) {
                    Some(rc_6) => {
                        TreeNode::join(rc_6, &rc_7);
                    }
                    None => todo!(),
                }
                assert_eq!(rc_7.get_values_till_root(), vec![7, 6, 5, 1]);
            }
            assert_eq!(
                get_strong_and_weak_counts(),
                vec![(1, 2), (2, 2), (2, 0), (2, 0), (2, 1), (2, 1)]
            );
        }

        #[test]
        fn test_parent_dropped() {
            let leaf = Rc::new(TreeNode::new(2));
            {
                let root = Rc::new(TreeNode::new(1));
                leaf.add_parent(&root);
                root.add_child(&leaf);

                assert_eq!(Rc::weak_count(&root), 1);
                assert_eq!(Rc::strong_count(&root), 1);

                assert_eq!(Rc::strong_count(&leaf), 2);
                assert_eq!(Rc::weak_count(&leaf), 0);
                // node dropped here
            }

            assert_eq!(Rc::strong_count(&leaf), 1);
            assert_eq!(Rc::weak_count(&leaf), 0);
            assert_eq!(
                if let Parent::Yes(ref p) = *leaf.parent.borrow() {
                    p.upgrade().is_none()
                } else {
                    false
                },
                true
            );
            // since node was dropped, weak pointer leads to None.
            // NOTE: This doesn't lead to Parent::No (obviously!)
        }
    }
}
