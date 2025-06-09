#[cfg(test)]
mod concurrency {
    // Threads
    /* Issues:
     * - Race conditions - access a resource in an inconsitent order
     * - Deadlocks - fight for resource, incorrectly handled dependencies?
     * - Execution dependent bugs - which happens only under certain situations and hard to reproduce
     */
    mod threads_basics {
        use std::thread;

        #[test]
        fn test() {
            let handler = thread::spawn(|| {
                println!("Hello from the other side");
                // NOTE: To see println, use `cargo test -- --nocapture`
            });
            // Here if we don't wait for the spawned thread to finish, before this main finishes (in
            // this case the `test` function), the spawned thread might outlive the main thread
            handler
                .join()
                // join returns Result, and is blocked till the thread returns (or panics)
                .expect("Couldn't join the thread; Most likely the thread paniced");
        }

        #[test]
        fn move_closures() {
            let mut v = vec![1, 2, 3];
            let d = 4;

            let handler = thread::spawn(move || {
                // spawn(f: F), where F is FnOnce() -> T, and F is 'static
                v.push(d); /* this will break if we don't specificy `move` in f,
                            * (move basically tells the closure that all the captured values are moved and not borrowed)
                            * because f cannot borrow v (mutably) since thread might out
                            * live the v's owner thread (main/move_closures here)
                            * OR possibly v can be dropped before thread could access v (race conditions).
                            *
                            * Technically, F is marked 'static and hence all the captured values
                            * should outlive 'static, and the only way to achieve that is either:
                            * - variable's lifetime is 'static, like &str on heap
                            * - variable is moved to F
                            * - variable hold values that implements Copy trait
                            *
                            * Checkout: https://doc.rust-lang.org/std/thread/fn.scope.html as an alternative!
                            */
                v
            });

            assert_eq!(d, 4);
            /* NOTE: remember, if copy trait is implement for captured elements, copy happens
             * instead of move, and variables are still usable, like d */

            v = handler
                .join()
                .expect("Some failure happened when trying to join the thread");

            assert_eq!(v, [1, 2, 3, 4]);
        }
    }
}
