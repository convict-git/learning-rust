#[cfg(test)]
mod advanced_features {
    mod unsafe_rust {
        // Static analysis is conservative
        // -- better to reject valid program than to accept invalid programs

        /*
        * 5 features allowed in unsafe rust:
          - Dereference a raw pointer
          - Call an unsafe function or method
          - Access or modify a mutable static variable
          - Implement an unsafe trait
          - Access fields of a union
        *
        * NOTE: Unsafe doesn't turn off borrow-checker though.
        */

        #[test]
        fn dereference_raw_pointer() {
            /*
            == Raw pointers ==
            - Are allowed to ignore the borrowing rules by having both immutable and
              mutable pointers or multiple mutable pointers to the same location
            - Aren’t guaranteed to point to valid memory
            - Are allowed to be null
            - Don’t implement any automatic cleanup

            can be mutable (*mut T) or immutable (*const T) {asterisk is part of type name}.
            can be created using raw borrow operator (&raw const / &raw mut).
             */

            // We can create raw pointers in safe rust as well
            let mut num = 4;
            let imm_raw_ptr: *const i32 = &raw const num; // immutable raw pointer to num
            let mut_raw_ptr: *mut i32 = &raw mut num; // mutable raw pointer to num

            /*
            ^ NOTE: We created a immutable borrow and a mutable borrow through raw pointers, which
            isn't allowed with references. Now you can read from the immutable borrow and write
            using the mutable borrow at the same time inside unsafe rust
            (that might create race conditions though!)
            */

            // We can define raw pointer in safe rust, but can deref them ONLY in unsafe rust
            unsafe {
                assert_eq!(*imm_raw_ptr, 4);
                assert_eq!(*mut_raw_ptr, 4);
                *mut_raw_ptr = 5;
                assert_eq!(*imm_raw_ptr, 5);
                assert_eq!(*mut_raw_ptr, 5);
            }

            // raw pointers doens't necessarily point to valid memory
            let random_address = 0x01234usize;
            let rnd_raw_ptr = random_address as *const i32; // raw pointer to a random (likely) invalid memory

            unsafe {
                // println!("{}", *rnd_raw_ptr);
                // ^ ERROR: This gives seg fault
            }
        }

        #[test]
        fn calling_unsafe_function() {
            unsafe fn dangerous() {
                let num = 4;
                let imm_raw_ptr: *const i32 = &raw const num; // immutable raw pointer to num
                unsafe {
                    // let mut_raw_ptr: *mut i32 = &raw mut num;
                    // ^ ERROR: You cannot borrow num as mutable, so rust borrow-checker is still working in unsafe block as well
                    println!("print to stdout from unsafe: {}", *imm_raw_ptr)
                }
            }

            // dangerous();
            // ^ ERROR: Call to unsafe function cannot be done in a safe block

            unsafe {
                dangerous(); // unsafe call allowed only in unsafe blocks  / unsafe functions
            }
        }
        // Coming back to unsafe rust after some time.. It isn't needed right now.
    }

    mod advanced_traits_and_types {
        #[test]
        fn associated_types() {
            /*
            What's the different between having an associated type vs generic?
            trait Iterator {
                 type Item;
                 fn next(&mut self) -> Option<Self::Item>;
                 ...
            }

            trait Iterator<T> {
                 fn next(&mut self) -> Option<T>;
                 ...
            }

            So, if you were to implement this trait for some type X, in case of associated type,
            there's exactly one value of Item that you'd specify, say:
            impl Iterator for X {
                 type Item = i32;
                 fn next(&mut self) -> Option<Self::Item> { ... }
            }

            but in case of generics, you will have the free generic T, and will require to
            specify T everytime you call next on X.

            impl Iterator<T> for X {
                 fn next(&mut self) -> Option<T> { ...  }
            }

            But this may not be required..
            * */
        }

        #[test]
        fn operator_overloading() {
            use std::ops::Add;
            #[derive(Debug, Copy, Clone, PartialEq)]
            struct Point {
                x: i32,
                y: i32,
            }

            impl Add for Point {
                // Add<Rhs=Self> // default generic is Self
                type Output = Point;

                fn add(self, rhs: Self) -> Self::Output {
                    Point {
                        x: self.x + rhs.x,
                        y: self.y + rhs.y,
                    }
                }
            }

            impl Add<i32> for Point {
                type Output = Point;

                fn add(self, rhs: i32) -> Self::Output {
                    Point {
                        x: self.x + rhs,
                        y: self.y + rhs,
                    }
                }
            }

            assert_eq!(
                Point { x: 1, y: 2 } + Point { x: 0, y: 3 },
                Point { x: 1, y: 5 }
            );
            assert_eq!(Point { x: 1, y: 2 } + 2, Point { x: 3, y: 4 });
        }

        #[test]
        fn newtype() {
            // NewType pattern helps implement external traits on external types
            struct VecStrWrapper(Vec<String>); // A tuple struct
            let wrapper_inst = VecStrWrapper(vec![String::from("hello"), String::from("world")]);

            // Implementing an external trait Display for VecStrWrapper (techically for Vec<String>, an external type)
            impl std::fmt::Display for VecStrWrapper {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "[{}]", self.0.join(", "))
                }
            }
            assert_eq!(wrapper_inst.to_string(), "[hello, world]");

            // If we still want all the methods for the encapsulated type through the new type,
            // we can implement a deref for it
            impl std::ops::Deref for VecStrWrapper {
                type Target = Vec<String>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            assert_eq!(wrapper_inst.len(), 2);

            // But if the whole point was to differentiate between the wrapper and the encapsulated
            // type, say, in some API, then deref/deref_mut might break the cause in borrow APIs
        }

        #[test]
        fn type_alias() {
            // Similar to typescript
            // NOTE: Useful only for reducing repition, but doesn't differentiate as different types
            // type Km = i32 // will treat Km as i32 and it can be used interchangibly

            type Thunk = Box<dyn Fn() + Send + 'static>;

            let f: Thunk = Box::new(|| println!("hello"));
            fn _x_f(_inp_thunk_fn: Thunk) { /* .. */
            }
            fn _y_f() -> Thunk {
                todo!()
            }
        }
    }
}
