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

    mod message_passing {
        /* "Do not communicate by sharing memory; instead, share memory by communicating."
         * `channels` -> one way communication between a transmitter and a receiver
         */

        use std::{sync::mpsc, thread, time::Duration};

        #[test]
        fn basic_channel_communication() {
            // `mpsc` -> multiple producer, single consumer (rust provided in standard library)
            // NOTE: channels cannot relay only a single type of information (for complex
            // use-cases, use enums/structs)
            let (tx, rx) = mpsc::channel::<String>();

            let _handler = thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                tx.send(String::from("Hello from the other side"))
                    .expect("receiver hung up!");
            });

            /* blocking operation to receive data from the thread (upstream channel / transmitter),
             * till sender sends a message (return Ok(T)) or sender is dropped (Err(RecvError))
             * If there's any messages in the buffer, even if the sender is dropped, recv receives
             * it first instead of throwing error */
            let msg = rx.recv().expect("sender dropped");
            assert_eq!(msg, "Hello from the other side");
        }

        #[test]
        fn send_and_recv_multiple_values_using_multiple_producers() {
            let (tx, rx) = mpsc::channel::<i32>();
            let tx2 = tx.clone();
            /* tx.clone() gives another producer for the same channel
             *
             * Remember, Rc recommends Rc::clone(&self) but Sender<T> recommends, tx.clone()
             *tx is perceived more like a handle (like file-description/socket) where .clone() doesn't convey deep
             * clone instead -- Todo more understanding need in this context (API desigining) */

            let _handler_1 = thread::spawn(move || {
                (1..=5).for_each(|val| {
                    tx.send(val).expect(
                        "Thread1: receiver hung up before transmitter could send all the values",
                    );
                    thread::sleep(Duration::from_millis(200));
                });
            });

            let _handler_2 = thread::spawn(move || {
                (6..=8).for_each(|val| {
                    tx2.send(val).expect(
                        "Thread2: receiver hung up before transmitter could send all the values",
                    );
                    thread::sleep(Duration::from_millis(500));
                })
            });

            /* Could `rx` have implemented Iterator trait? Let's see
            pub trait Iterator {
                type Item; // Item would be T, value received through the channel from transmitter?
                fn next(&mut self) -> Option<Self::Item>; // Return try_recv mostly?
            }
            */
            let mut adapted_received_values = rx.iter().map(|x| x * 2).collect::<Vec<_>>(); // OR can do // for val in rx { ... }

            adapted_received_values.sort(); // sorting because we are NOT sure about the order in
                                            // this concurrenct execution of threads

            assert_eq!(adapted_received_values, vec![2, 4, 6, 8, 10, 12, 14, 16]);
        }

        #[test]
        fn bidirection_communication_using_two_mpsc_channel() {
            enum ClientMessage {
                Quit,
                Incr(i32),
                Get,
            }
            enum ServerMessage {
                Get(i32),
            }

            let (server_tx, client_rx) = mpsc::channel::<ServerMessage>();
            let (client_tx, server_rx) = mpsc::channel::<ClientMessage>();

            let _server_handle = thread::spawn(move || {
                let mut server_state: i32 = 0;
                loop {
                    match server_rx.recv() {
                        Ok(client_msg) => match client_msg {
                            ClientMessage::Quit => break,
                            ClientMessage::Incr(value) => server_state += value,
                            ClientMessage::Get => server_tx
                                .send(ServerMessage::Get(server_state))
                                .expect("client hung up before receiving response for the request"),
                        },
                        Err(_) => break, // client hung up before calling quit
                    }
                }
            });

            [
                ClientMessage::Incr(1),
                ClientMessage::Get,
                ClientMessage::Incr(2),
                ClientMessage::Get,
                ClientMessage::Quit,
            ]
            .into_iter() // using into_iter to move out the values from the collection
            .for_each(|client_msg| client_tx.send(client_msg).expect("server hung up"));

            let received_msg_from_server = client_rx
                .iter()
                .map(|server_message| {
                    let ServerMessage::Get(value) = server_message;
                    value
                })
                .collect::<Vec<_>>();

            assert_eq!(received_msg_from_server, vec![1, 3]);
        }
    }

    mod shared_state_concurrency {
        use std::{
            sync::{Arc, Mutex},
            thread,
        };

        struct X(i32);

        // In channels, after the message passing, ownership is given away to the receiver
        // but we might need to consume the same data with multiple consumers at the same time
        #[test]
        fn mutexes_basics() {
            // Mutual exclusion -- uses lock system
            // Mutex::new() -> Mutex<T> -> .lock() -> Result<MutexGuard<'_, T>> -
            //
            // The most common way to unlock the mutex is to drop the associated MutextGuard
            // received when locking it. Other way involves explicit dropping of lock guard using
            // // let lock_guard = m.lock().unwrap(); mem::drop(lock_guard)
            let m = Mutex::<X>::new(X(2));
            {
                let m_result = m.lock();
                match m_result {
                    Ok(mut m_guard) => {
                        *m_guard = X(3);
                        // NOTE: immutable m, provides interior mutability. Basically RefCell but threadsafe
                    }
                    Err(_) => todo!(),
                } // m is unlocked as soon as m_result is dropped (which is basically when m_guard is dropped)
            }
        }

        #[test]
        fn sharing_mutex_between_multiple_threads() {
            /* // This example breaks because we are trying to move counter to FnMut and is needed
               // by multiple such FnMut closures (for multiple threads)
               // Historically, we have solved this using Reference Counted smart pointer (Rc), where
               // we can have multiple immutable borrowers.
               // But for thread-safety, we use Arc (Atomic Reference Count smart pointer) instead of Rc.

            let counter = Mutex::<i32>::new(0);

            (0..10)
                .map(|_| {
                    thread::spawn(move || {
                        let mut counter_mutex_guard = counter // can't move counter here, used in FnOnce
                            .lock()
                            .expect("Failed to acquired lock on counter mutex");
                        *counter_mutex_guard += 1;
                    })
                })
                .for_each(|handler| {
                    handler.join().expect("Error joining the thread");
                });

            let counter_value = *counter.lock().expect("Error locking the counter mutex");
            assert_eq!(counter_value, 10);
            */

            let rc_counter = Arc::new(Mutex::new(0)); // Arc<Mutex<i32>>

            (0..10)
                .map(|_| {
                    let rc_counter_t = Arc::clone(&rc_counter);
                    // use clone in each thread. Original rc_counter is just borrowed immutably in map's closure
                    // (which technically allows FnMut)

                    thread::spawn(move || {
                        // And now, we can move rc_counter_t, which is moved exactly once
                        let mut counter_mutex_guard = rc_counter_t
                            .lock()
                            .expect("Failed to acquired lock on counter mutex");
                        *counter_mutex_guard += 1;
                    })
                })
                .for_each(|handler| {
                    handler.join().expect("Error joining the thread");
                });

            let counter_value = *rc_counter.lock().expect("Error locking the counter mutex");
            assert_eq!(counter_value, 10);
        }
    }

    mod send_and_sync_traits {
        /* === Send ===
         * Send trait: ownership of values can be transfered (move) from one thread to another
         *
         * (all primitives types in Rust are Send, except some smart pointers like Rc<T> (alternative Arc<T>))
         */

        /* === Sync ===
         * Sync trait: allowing access from multiple threads (through & references)
         *
         * (all primitives types implement Sync, except some smart poointers like Rc<T> and RefCell<T>
         * (alternative is Arc<T> and Mutex<T>))
         *
         * T is Sync, if &T implements Sync (reference can be sent safely to other threads and
         * thus safe refering from multiple threads)
         */

        /*
         * == Rc<T> neither Send nor Sync ==
         *
         * - NOT Send: increases / decreases ref count, but the count isn't behind any lock.
         * rc1 = Rc::clone(&Rc<T>) returns another Rc, rc2, but referring the same allocation.
         * If these two rc1, and rc2 and used in different threads (moved):
         * -- Two threads can update the count at the same time (no atomicity).
         * -- Two threads can try to drop at the same time, resulting in double-free.
         *
         * - NOT Sync:
         * Passing references aren't safe either since &Rc<T> could be used for Rc::clone and again
         * leading to unsafe increases/decreases of strong/weak ref counts.
         */

        /*
         * == RefCell<T> Send (if T:Send) but NOT Sync ==
         *
         * - Is Send: RefCell when moved, is the sole owner responsible for performing any operation
         * on the value, and no other RefCell can be mutating the value owned by the first RefCell.
         * But why only if T:send? When RefCell<T> is moved, T is moved too. If T isn't Send, it
         * isn't safe to move it across the thread.
         *
         * - NOT Sync: because &RefCell<T>, can perform mutable borrowing in different threads
         */

        /*
         * == Mutex<T> Send (if T: Send) and Sync ==
         *
         * - Is Send: Similar to RefCell<T>
         *
         * - Is Sync: can be dereffed only through lock system
         *
         * but == MutexGuard<T> == NOT Send, NOT Sync
         *
         * - NOT Send: MutexGuard acquired lock on a Mutex in a particular thread, moving it
         */
    }
}
