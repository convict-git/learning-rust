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
}
