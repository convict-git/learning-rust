#[cfg(test)]
mod closures_and_fn_pointers {
    mod closures {
        // == Closures ==

        // Traits implemented by closures:
        // - FnOnce -> all closures implement atleast this trait, moves captured values out of closure. Can be called only once.
        // - FnMut -> mutates values captured in closure, can be called multiple times.
        // - Fn -> neither mutates, nor moves, can be called multiple times

        enum MyOption<T> {
            Some(T),
            None,
        }

        impl<T> MyOption<T> {
            fn unwrap_or_else_value(self, val: T) -> T {
                match self {
                    MyOption::Some(x) => x,
                    MyOption::None => val,
                }
            }

            fn unwrap_or_else<F>(self, f: F) -> T
            where
                F: FnOnce() -> T, // since F: FnOnce -> f can move captured values, do mutable borrows ...
            {
                match self {
                    MyOption::Some(x) => x,
                    MyOption::None => f(),
                }
            }
        }

        #[test]
        fn closure_traits() {
            let mut list = vec![1, 2, 3];
            let mut fn_borrows_mutably = || list.push(4);
            /*
            // NOTE: fn_borrows_mutably should be binded to a mutable variable. Why?
            pub trait FnMut<Args: Tuple>: FnOnce<Args> {
               /// Performs the call operation.
               #[unstable(feature = "fn_traits", issue = "29625")]
               extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
            }

            // so when, call_mut is called, it takes a mutable borrow of self, and hence fn_borrows_mutably should be mut
            // Also, it's easier for the reader, to understand, since fn_borrows_mutably is mut, it can
            // lead to mutations
            */

            // println!("{list:?}");
            /* NOTE: this will break, because, fn_borrows_mutably has an mutable borrow of list already, and
             * we can no more borrow immutably/mutable before the existing mutable borrow ends */

            fn_borrows_mutably();
            println!("{list:?}");

            std::thread::spawn(move || println!("From a different spawned thread: {list:?}"))
                .join()
                .unwrap();
            /* We can forcefully move the values captured by a closure using `move` before the parameters.
             *
             * Imp NOTE: Can we capture list in the closure defined inside spawn without `move` keyword (i.e. just borrow mutably/immutably)?
             * No, because the newly spawned thread might outlive the main thread, and if it was just a borrow,
             * main thread will drop the value before this thread could end (list would dangle).
             * Hence, spawn expects F must be 'static, i.e. it expects the arguments to live for the 'static lifetime.
             * There are only two ways: either the data is itself 'static (eg. string literals), or is
             * owned by the closure itself (so it lives as long as closure lives)
             * */

            // println!("{list:?}"); // This will break, since list was forcefully moved, and ownership was
            // transfered to a different thread. And the list was dropped when the thread ended.
        }

        #[test]
        fn more_examples_on_traits() {
            let mut list = vec![1, 2, 3];
            let mut sort_operations: Vec<String> = Vec::new();
            let v = String::from("Hello world");
            let my_closure = |x: &i32, y: &i32| {
                // sort_operations.push(v); // ERROR
                /* This will turn the function to FnOnce since v is moved (String hence no copy trait).
                 * `sort_by` expects the closure to implement FnMut or Fn since it might be called multiple times
                 * //  impl<T> [T]
                 * //  pub fn sort_by<F>(&mut self, mut compare: F)
                 * //  where F: FnMut(&T, &T) -> Ordering,
                 *
                 * One way to fix it is by v.clone()
                 * */
                if x < y {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            };

            list.sort_by(my_closure);
        }

        #[test]
        fn lifetimes() {
            /* When designing a function that accepts/returns a closure, think of lifetimes as well!
             * We don't want the value usage to outlive the value itself (use-after-free).
             * Also, specifying lifetimes will generally lead to better error messages than rust doing it itself */

            // NOTE: Try out what happens if you don't specify the lifetimes in the below mentioned case.
            // How will the lifetime elision work here?
            // What will be the inferred lifetimes by the rust compiler? What issues will it cause?
            fn make_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
                || s_ref.to_string()
            }

            let s = String::from("hello world");
            let _s_clone = make_cloner(&s)(); // [just reminding]: automatic deref from &String to &str
        }

        #[test]
        fn iterators_and_closures() {
            // Iterator trait and the next method -- revisit to iterators
            /*
            trait Iterator {
                type Item; // associated type with the trait
                fn next(&mut self) -> Option<Self::Item>; // why &mut self? the struct for which you define the
                                                          // trait Iterator for, is responsible for handling
                                                          // the state of the iterator as well, eg. current
                                                          // index in a vector iterator.
                                                          // So on next, you will be changing some fields in
                                                          // the struct to maintain the state
            }
            */
            //
            /* In vec, we have
             * - iter() -> for immutable references to the items,
             * - into_iter() -> for owned values,
             * - iter_mut() -> for mutable reference to the items
             *
             * Various methods for iterators:
             * - consuming iterators, eg sum
             * - iterator adaptors, eg map
             */
            let v = vec![1, 2, 3];
            let v_iter = v.iter();

            let v_inc_iter = v_iter.map(|x| x + 1);
            // NOTE: Iterators are lazy. Nothing happens here since iterator isn't consumed yet.
            // Hence, iterator adaptors don't do anything unless a iterator consumer is used, like collect.

            // to consume the iterator, let's collect
            let v_inc = v_inc_iter.collect::<Vec<i32>>();
            println!("{v_inc:?}");

            // NOTE: Now you know about closures and the various ways they capture the values from the
            // environment, you should be mindful about passing the closures to these iterator methods
        }

        #[test]
        fn returning_closures() {
            // Closures are represented by traits: FnOnce, FnMut, Fn and hence can't be returned directly,
            // infact, they don't have concrete opaque types, so the return types are trait objects instead.
            fn returns_inc_closure() -> impl Fn(i32) -> i32 {
                |x| x + 1
            }

            fn returns_dec_closure() -> impl Fn(i32) -> i32 {
                move |x| x - 1
            }

            // let handlers = vec![returns_inc_closure(), returns_dec_closure()];
            // ^ ERROR: Though both the functions return a closure that impl Fn(i32) -> i32,
            //   that doesn't results in same opaque type. These are technically trait objects
            //   whose size can't be known at compile time.
            //   So either we use borrowed values or wrap them in a smart pointer.

            let handlers: Vec<Box<dyn Fn(i32) -> i32>> = vec![
                Box::new(returns_inc_closure()),
                Box::new(returns_dec_closure()),
                Box::new(returns_inc_closure()),
            ];
            assert_eq!(handlers.iter().fold(0, |acc, f| f(acc)), 1);
        }
    }

    mod fn_pointers {
        #[test]
        fn basics() {
            // Fn : Trait
            // fn : Type (function-pointer type)
            // syntax is almost the same, Fn(_) -> _ / fn(_) -> _

            // fn implements all the Fn traits: Fn, FnMut, FnOnce.. so you can pass a fn pointer
            // where closure is expected (hence don't design APIs expecting fn pointers, instead
            // keep it with a generic type f which implements certain Fn trait)

            let list_of_numbers = vec![1, 2, 3];
            let list_of_strings_using_closure: Vec<String> =
                list_of_numbers.iter().map(|i| i.to_string()).collect();

            assert_eq!(list_of_strings_using_closure, vec!["1", "2", "3"]);

            // OR using fully-qualified fn pointer from ToString trait
            let list_of_strings_using_fn_pointer: Vec<String> =
                list_of_numbers.iter().map(ToString::to_string).collect();

            assert_eq!(list_of_strings_using_fn_pointer, vec!["1", "2", "3"]);

            // Imp NOTE: Each enum variant is also an initialized function, and that can also be
            // used as fn pointers
            #[derive(Debug, PartialEq)]
            enum Status {
                Value(i32),
                Stop,
            }
            let statuses: Vec<Status> = list_of_numbers.into_iter().map(Status::Value).collect();
            // into_iter because Status::Value(i32) not &i32

            assert_eq!(
                statuses,
                vec![Status::Value(1), Status::Value(2), Status::Value(3)]
            );

            // Imp NOTE: `fn` (the function-pointer type) can't be used if some value is dynamically captured in the
            // closure from its defined scope.
        }
    }
}
