#[cfg(test)]
mod oops {
    #[test]
    fn basics() {
        // Encapsulation is achived using pub, simple.

        /* === Inheritence ===
         * NOTE: There's not way to define a struct to use field / methods from some "parent" struct
         * but traits can have default implementations, which can also be overriden by any type
         * that implements that trait.
         *
         * === Polymorphism === Both static (using generics which can be bounded too), and dynamic using
         * trait objects.
         *
         */

        /*
         * Why use dyn Trait, when you can use enums? Enums can definitely help in some cases, but
         * limited to when the type of the value in consideration can only be from certain set of
         * types.
         *
         * == Difference between impl Tr and dyn Tr, where Tr is some Trait ==
         * - impl Tr -> static dispatch (static polymorphism)
         * actual types and Size are known at compile time based on the usage.
         * This basically gets replaced at compile time with the actual types.
         * These cannot be stored in heterogenous containers.
         *
         * - dyn Tr -> dynamic dispatch (dynamic polymorphism)
         * known at runtime, and resolved using virtual tables. Size is not known at compile time.
         * This can be used for trait objects, i.e. reference to objects of different types implementing
         * a particular trait(s).
         */

        /* == Type Erasure ==
         * When we want the compile to forget the actual time at compile time and just care about the type
         * implemeting certain trait(s). This allow dynamic (runtime) polymorphism, allows to treat
         * heterogenous types uniformly assuming they implement certain trait(s).
         * Once type is erased, i.e. captured with dyn Tr, you can no longer retrive the size, fields,
         * original impl methods, etc, only the methods implemented for the traits are accessible.
         *
         * let type_erased_animal: Box<dyn Animal> = Box::new(Dog {})
         * If you work on this `type_erased_animal`, any methods from Animal will surely access the
         * original Dog object but, you cannot technically write code assuming it's a dog! Let that settle in!
         *
         *
         * == Trait Objects ==
         * When traits are used as runtime types, (again dyn Tr).
         * Useful for maintaing a collection of heterogenously typed objects but implementing certain trait(s).
         * fn f(x: &dyn Tr) { ... x.tr_method() ... }
         * Examples usages like:
         * - Plugins: Vec<Box<dyn Plugin>>,
         * - Heterogenous containers: Vec<Box<dyn Drawable>>,
         * - APIs hiding implementation types: fn get_reader() -> Box<dyn Read>,
         *   // consumer have no idea about the actual returned type
         *
         * -- Trait object is basically a Fat pointer -> [data pointer | vtable pointer]
         * data pointer (where the actual data is stored) points to the actual Data object
         * vtable pointer points to a table where the Trait methods are implemented for the Data
         *
         * - Unlike some OOP languages, a trait object cannot be "downcasted" to a more concrete type
         * (except in the case of the Any trait? Todo Read later!).
         *
         * == Object Safety ==
         * Imp NOTE: Not all traits can be turned into trait objects! Trait has to be "Object-safe" for dyn Trait.
         * - Trait methods can't be using Self in return type or by value
         * - Trait methods should NOT be generic, vtables can't be resolved for generic types at runtime.
         * - Only &self, &mut self, Box<Self> are allowed
         */
    }

    #[test]
    fn posts() {
        trait State {
            fn request_review(self: Box<Self>) -> Box<dyn State>;
            fn approve(self: Box<Self>) -> Box<dyn State>;
            fn content<'a>(&'a self, post: &'a Post) -> &'a str {
                ""
            }
        }

        struct Published {}

        impl State for Published {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn content<'a>(&self, post: &'a Post) -> &'a str {
                // Accessible here though content is private field, since they are defined in the
                // same module
                &post.content
            }
        }

        struct PendingReview {}
        impl State for PendingReview {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                Box::new(Published {})
            }
        }

        struct Draft {}
        impl State for Draft {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                Box::new(PendingReview {})
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }

        struct Post {
            state: Option<Box<dyn State>>,
            content: String,
        }

        impl Post {
            fn request_review(&mut self) {
                if let Some(s) = self.state.take() {
                    // NOTE: This is why we wrapped it with Option: Rust doesn't let us keep
                    // unpopulated fields in a struct even momentarily, so it would be impossible to
                    // move out a value and replace with something else, unless we swap. `take`
                    // does the mem swapping for us.
                    // Hence, we do a `take()` which replaces the Some(T) to None, and move out the T value.
                    //
                    // This is useful since we need to do something like: self.state = self.state.request_review()
                    // and, self is mutabily borrowed but `request_review(self)` needs owned value
                    //
                    // Also, `take` will ensure that the previous value is dropped since it moves the
                    // value of out of the Option.

                    self.state = Some(s.request_review());
                }
            }
            fn approve(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.approve());
                }
            }
        }

        /* NOTE: One can add more methods in the above at client side by writing another trait,
         * say StateExt:
         * trait StateExt {
         * }
         * impl<S: State> StateExt for S {
         * }
         *
         * One can also add states, which will require adding another struct and implement State
         * trait for it. Though it feels like these newly added states can be kept only in the
         * beginning of the transition chain.
         */

        // NOTE: We could have possibly used Enums here for the state. So always check for the
        // trade-offs between using an enum v/s trait objects.
        // - when using enums, match expressions will come at every place where state is checked,
        //  while when using trait objects, we only need to add one new struct and implement State for it.
        // - but this comes at runtime cost of maintaining the vtable pointer and traversing the vtable

        // Other way to do it is managing the state as different Post types, like Post, DraftPost,
        // PublishedPost ..
    }
}
