/*
 * == Async Programming in Rust ==
 * - More about: compute-bound operations v/s IO-bound operations, later.
 * - Future: maybe NOT ready now, but will become ready at some point in the future
 * - awaits are possible on blocks and functions
 * - async/awaits are syntatic sugars -- will be compiled to eqv code using the Future trait
 *   (just like for loop is replaced with Iterator trait)
 * - Future is lazy, just like iterators, they won't do anything unless awaited
 *   (much different than JS where the promises are eager)
 * - await keyword is at the end which helps in chaining (coming from JS we know the pain)
 *
 * == NOTE on starvating ==
 * - Rust gives runtimes a chance to pause a task, and shift to another task if the current future it
 *   is awaiting on isn't completed yet. Reverse implication is true as well; i.e. it can only pause
 *   at await checkpoints (anything between two await runs synchronously).
 *   This can lead to a future starving other futures.
*/

// == External crates used: ==
// future -> standard library for Future
// tokio -> crate for async runtime for Rust
// reqwest -> crate for data-fetching through HTTP requests
// scraper -> crate for parsing and querying browser documents

/* == Runtimes ==
 * - Unlike other languages, Rust doesn't couple the async runtime (it doesn't have any) with its main runtime. This allows
 *   flexibility in choosing the runtime based on our needs,
 *   which also means that we will have to setup the runtimes ourselves too.
 * - Executors run/execute the async blocks.
 *
 * NOTE: since we are using tokio, we will be using tokio::sync::mpsc instead of std::sync::mpsc
 * for message passing.
 */

struct Response(reqwest::Response);
impl Response {
    pub async fn text(self) -> String {
        self.0.text().await.unwrap() // If the response cannot be deserialized, this panics instead of returning a [`Result`]
    }
}

struct Html {
    inner: scraper::Html,
}
impl Html {
    /// Parse an HTML document from a string
    pub fn parse(source: &str) -> Html {
        Html {
            inner: scraper::Html::parse_document(source),
        }
    }

    /// Get the first item in the document matching a string selector
    pub fn select_first<'a>(&'a self, selector: &'a str) -> Option<scraper::ElementRef<'a>> {
        let selector = scraper::Selector::parse(selector).unwrap();
        self.inner.select(&selector).nth(0)
    }
}

mod helpers {
    use futures::future::{self, Either};
    use rand::Rng;
    use std::{future::Future, pin::pin, time::Duration};
    use tokio::time::{sleep as async_sleep, Sleep};

    use super::*;

    pub async fn get(url: &str) -> Response {
        Response(reqwest::get(url).await.unwrap())
    }

    pub fn tokio_rt_block_on<F: std::future::Future>(future: F) -> F::Output {
        // a new tokio runtime is created everytime `run` is called
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(future)
    }

    pub async fn async_random_sleep(max_time: u64) {
        let random_delay = rand::thread_rng().gen_range(1..=max_time);
        async_sleep(Duration::from_millis(random_delay)).await; // tokio::time::sleep async anologous of std::thread::sleep
    }

    // ToDo : understand later the need for pinning the future. It has to do with not allowing to
    // move the future in memory but more clarity of thought is needed
    pub async fn race<A, B, F1, F2>(f1: F1, f2: F2) -> Either<A, B>
    where
        F1: Future<Output = A>,
        F2: Future<Output = B>,
    {
        let f1 = pin!(f1);
        let f2 = pin!(f2);
        // NOTE: select is from futures::future and NOT std::future
        match future::select(f1, f2).await {
            Either::Left((a, _f2)) => Either::Left(a),
            Either::Right((b, _f1)) => Either::Right(b),
        }
    }
}

async fn page_title(url: &str) -> Option<String> {
    let response = helpers::get(url).await;
    let response_text = response.text().await;

    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html()) // Option::map to convert Option<T> to Option<U>
}
// NOTE: The actual return type is impl Future<Output=Option<String>>.
/*
* below is the transpiled code for the page_title function (awaits are yet to be transpiled, this
* is just to show the function definition and fn body wrapped in the async move block and return type
* of the code inside the block becomes the Futured returned type of the outer function)
fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move { // Async move block
       let response = helpers::get(url).await;
       let response_text = response.text().await;

       Html::parse(&response_text)
           .select_first("title")
           .map(|title_element| title_element.inner_html())
    }
}
*/

#[cfg(test)]
mod async_docs {
    // NOTE: Technically reqwest should be mocked but for now we are doing real HTTP requests in these tests

    use super::*;
    use futures::{
        future::{join_all as join_all_futures, Either},
        join, Stream,
    };
    use rand::Rng;
    use std::{
        future::Future,
        pin::{pin, Pin},
        sync::Arc,
        thread,
        time::Duration,
    };
    use tokio::{
        sync::{mpsc::unbounded_channel, Mutex},
        task::{spawn as spawn_task, yield_now as async_yield_now},
        time::sleep as async_sleep,
    };
    use tokio_stream::{iter as stream_from_iter, wrappers::UnboundedReceiverStream, StreamExt};

    #[test]
    fn basic_async() {
        assert_eq!(
            helpers::tokio_rt_block_on(async { page_title("https://google.com").await }),
            Option::Some(String::from("Google"))
        );
    }

    #[test]
    fn race_in_async() {
        match helpers::tokio_rt_block_on(async {
            helpers::race(
                page_title("https://google.com"),
                page_title("https://facebook.com"),
            )
            .await
        }) {
            Either::Left(Some(t)) => assert_eq!(t, "Google"),
            Either::Right(Some(t)) => assert_eq!(t, "Facebook"),
            _ => panic!("Some error occured!"),
        }
    }

    #[test]
    fn concurrency_with_async() {
        // NOTE: here we are using Arc from std::sync::Arc, and Mutex from tokio::sync::Mutex (instead of std)
        // Why are we using Arc<Mutex<T>>, Arc because we need counted reference smart pointer that
        // allows us multiple borrows. Mutex because we need interior mutability with locking.
        let items_counted_mutex = Arc::new(Mutex::new(vec![]));
        helpers::tokio_rt_block_on(async {
            let items_counted_mutex_clone = Arc::clone(&items_counted_mutex);
            let task_join_handle = spawn_task(async move {
                for i in 0..=5 {
                    {
                        let mut item_mutex_guard = items_counted_mutex_clone.try_lock().unwrap();
                        item_mutex_guard.push(i);
                        // free the lock before await itself
                    }
                    helpers::async_random_sleep(200).await;
                }
            });

            {
                let items_counted_mutex_clone = Arc::clone(&items_counted_mutex);
                let mut item_mutex_guard = items_counted_mutex_clone.try_lock().unwrap();
                for i in 6..10 {
                    item_mutex_guard.push(i);
                }
            }

            task_join_handle.await.unwrap();
            // We need to await the spawned task to ensure the current future completion
            // means all internally created futures are completed too
        });

        // since helpers::run is blocking (since it uses tokio::runtime::block_on), we can
        // assert_eq here safely
        {
            let mut items_ref = items_counted_mutex.try_lock().unwrap();
            items_ref.sort();
            assert_eq!(*items_ref, (0..10).collect::<Vec<_>>());
        }
    }

    #[test]
    fn concurrency_with_fairness_using_join_and_channels_for_message_passing() {
        let (sender_1, mut receiver) = unbounded_channel::<i32>();
        let sender_2 = sender_1.clone();
        let mut data: Vec<i32> = vec![];

        helpers::tokio_rt_block_on(async {
            // async move to ensure senders are moved and dropped once the async block is completed
            let tx_fut1 = pin!(async move {
                for i in 0..=5 {
                    sender_1.send(i).unwrap();
                    async_sleep(Duration::from_millis(200)).await;
                }
            });

            let tx_fut2 = pin!(async move {
                for i in 6..10 {
                    sender_2.send(i).unwrap();
                    async_sleep(Duration::from_millis(200)).await; // fairness can't be achieved using join if this was 100ms
                }
            });

            let rx_fut = pin!(async {
                // NOTE: We don't have an iterator for async series of items
                while let Some(msg) = receiver.recv().await {
                    data.push(msg);
                    // This while loop will break both the sender_1 and sender_2 are dropped
                }
            });

            // Using join will ensure fairness (using join! macro instead of join, join3, join4... fns )
            // join!(tx_fut1, tx_fut2, rx_fut);
            // NOTE: We can use the join! macro or join, join2, join3... only when the number of
            // futures are known at compile time. But there can be cases, when we need to work on
            // collection of futures

            // let futures = vec![tx_fut1, tx_fut2, rx_fut]; // ERROR: This breaks because all the
            // async blocks, even if they return the same type, Future<Result=()>, aren't identical
            // The compiler suggests to pin these. Todo: Read more on this later.
            // Also, since we are using dyn Future, we need to Box it since the Size is not known
            // at compile time. But we really don't need heap allocations here, just references to
            // pinned memory in the stack will work as well!

            let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_fut1, tx_fut2, rx_fut];
            join_all_futures(futures).await; // here the collection must implement Iterator and
                                             // Item must be a Future
        });

        assert_eq!(data, vec![0, 6, 1, 7, 2, 8, 3, 9, 4, 5]); // NOTE: This could be still a flaky test
    }

    #[test]
    fn yield_now_dont_let_us_starve() {
        let slow_closure = || thread::sleep(Duration::from_millis(1000));
        // ^ imitating a sync blocking slow job

        let race_result = helpers::tokio_rt_block_on(async {
            let fut_a = async {
                slow_closure(); // block the thread with a slow blocking task
                async_yield_now().await; // regular yeild ensure that other futures are not
                                         // starving due the long-running slow blocking operations
                                         // in one of the futures. This is done by manually yielding
                                         // the control back to runtime to allow other futures to run
                slow_closure();
                async_yield_now().await;
                slow_closure();
                async_yield_now().await;
                slow_closure();
            };

            let fut_b = async {
                slow_closure();
            };

            helpers::race(fut_a, fut_b).await
        });
        if let Either::Left(_) = race_result {
            panic!("First future can't win the race");
            // NOTE: Uncomment the async_yield_now().await in fut_a to see fut_b never getting the
            // resource to win the race and starve
        }
    }

    #[test]
    fn abstractions_with_futures_custom_timeout() {
        // Implementing custom async timeout function which returns a Result<Future::Output, Duration>
        async fn custom_timeout<F: Future>(
            f: F,
            max_time: Duration,
        ) -> Result<F::Output, Duration> {
            match helpers::race(f, async_sleep(max_time)).await {
                Either::Left(o) => Ok(o),
                Either::Right(_) => Err(max_time),
            }
        }

        let slow_fut = async {
            async_sleep(Duration::from_millis(2000)).await;
            return 0;
        };

        let fast_fut = async {
            async_sleep(Duration::from_millis(100)).await;
            return 1;
        };

        helpers::tokio_rt_block_on(async {
            // Slow future should timeout and return Result::Err
            assert!(custom_timeout(slow_fut, Duration::from_millis(200))
                .await
                .is_err());

            // Fast future should run correctly and return Result::Ok
            assert!(custom_timeout(fast_fut, Duration::from_millis(200))
                .await
                .is_ok());
        });
    }

    #[test]
    fn abstraction_with_futures_custom_future_retry_with_timeouts() {
        // Todo: Keeping this iterative, though we can keep it recursive, but that needs some Box
        // pinning since future cannot be infinitely sized.. Will come back to this later
        async fn retry_with_timeout<F: Future, FutGenClosure: Fn() -> F>(
            fut_generation_closure: FutGenClosure,
            max_time: Duration,
            max_tries: u8,
        ) -> Result<(F::Output, u8), ()> {
            let mut current_tries = 0;
            loop {
                current_tries += 1;
                if current_tries > max_tries {
                    break;
                }

                // Need a fut_generation_closure since we a need a fresh future for every iteration.
                // This is because helpers::race will move the future and will make it unusable for further iterations
                let fut = fut_generation_closure();
                if let Either::Left(o) = helpers::race(fut, async_sleep(max_time)).await {
                    return Result::Ok((o, current_tries));
                }
            }

            Result::Err(())
        }

        let get_a_random_slow_future = || async {
            helpers::async_random_sleep(200).await;
            42
        };

        helpers::tokio_rt_block_on(async {
            // NOTE: This is a flaky test but just for learning purposes, assuming that atleast in
            // 10 random tries, one of the slow_future will run in less than 50ms.
            assert!(
                retry_with_timeout(get_a_random_slow_future, Duration::from_millis(50), 10)
                    .await
                    .is_ok()
            );
        });
    }

    #[test]
    fn streams_basics() {
        // == Streams == async form of iterators
        // Also, we can convert any iterator to stream

        assert_eq!(
            helpers::tokio_rt_block_on(async {
                let (tx, mut rx) = unbounded_channel::<i32>();

                let client_fut = async move {
                    let v_iter = vec![1, 2, 3].into_iter();
                    let mut stream_from_v_iter = stream_from_iter(v_iter);
                    // stream_from_v_iter implements tokio_stream::StreamExt trait
                    // (Ext is popularly used for extension for some trait),
                    // so unless we import tokio_stream::StreamExt, we can't use .next()
                    // [traits needs to be imported for their methods to be used]

                    while let Some(x) = stream_from_v_iter.next().await {
                        tx.send(x).unwrap(); // without await
                    }
                };
                let server_fut = async {
                    let mut msgs = vec![];
                    while let Some(x) = rx.recv().await {
                        msgs.push(x * x);
                    }
                    msgs
                };

                let (_, msgs) = join!(client_fut, server_fut);
                msgs
            }),
            vec![1, 4, 9]
        );
    }

    #[test]
    fn composing_and_merging_streams() {
        #[derive(Debug)]
        enum StreamResponse<T> {
            Result(T),
            Interval(i32),
        }
        // Creating a get_messages "sync" function which returns a stream from an async task
        fn get_messages() -> impl Stream<Item = StreamResponse<i32>> {
            let (tx, rx) = unbounded_channel::<StreamResponse<i32>>();

            spawn_task(async move {
                let msgs = 0..10;
                for msg in msgs {
                    helpers::async_random_sleep(200).await;
                    tx.send(StreamResponse::Result(msg)).unwrap();
                }
            });

            UnboundedReceiverStream::new(rx)
        }

        // A never ending infinite stream
        fn get_intervals() -> impl Stream<Item = i32> {
            let (tx, rx) = unbounded_channel::<i32>();
            spawn_task(async move {
                let mut count = 0;
                loop {
                    helpers::async_random_sleep(5).await;
                    if let Err(err_msg) = tx.send(count) {
                        eprintln!(
                            "Error sending message {:?} to unbounded channel. ERROR reason: {:?}",
                            count, err_msg
                        );
                        break; // breaks the infinite loop interval if receiver is dropped, or
                               // basically the unbounded_channel closes (called close() on rx)
                    }
                    count += 1;
                } // Never ending loop
            });

            UnboundedReceiverStream::new(rx)
        }

        assert_eq!(
            helpers::tokio_rt_block_on(async {
                let interval_stream = get_intervals()
                    // Mapping the impl Stream<Item = i32>  to impl Stream<Item = StreamResponse<i32>>, so it can be merged with get_messages stream
                    .map(|x| StreamResponse::<i32>::Interval(x))
                    // Throttle to reduce the polling frequency of the stream
                    .throttle(Duration::from_millis(100))
                    // Timeout
                    .timeout(Duration::from_millis(200));

                let msgs_stream = get_messages();
                // Composing the message stream with timeouts
                let stream_with_timeouts = msgs_stream.timeout(Duration::from_millis(100));
                // Take only first 50 items from the merged stream
                let merged_stream = stream_with_timeouts.merge(interval_stream).take(50);

                let mut pinned_merged_stream = pin!(merged_stream); // This needs to be pinned. ToDo: check out later why?

                let mut received = vec![];
                while let Some(result_from_stream) = pinned_merged_stream.next().await {
                    // NOTE: that timeout will only tell us that we didn't receive the next item in
                    // that duration. But unbounded_channel will ensure that we receive the next item in the future polls.
                    match result_from_stream {
                        Ok(StreamResponse::Result(msg)) => received.push(msg),
                        Ok(StreamResponse::Interval(i)) => eprintln!("Interval {i}"),
                        Err(_) => eprint!("This stream timed-out\n"),
                    }
                }
                received
            }),
            // This is a flaky test and might fail since we are taking only first 50 stream items
            (0..10).collect::<Vec<_>>()
        );
    }

    #[test]
    fn closer_look_at_async_traits() {
        // Future, Stream, StreamExt, Pin, Unpin
        /* == Future ==
         * A future is polled, and at every poll, it might be either in pending state or completed.
         * If it's in pending state, the async runtime gets control, pause work on this future, and
         * moves to the other pending futures for polling, and check this one later.
         *
         * pub trait Future {
         *      type Output;
         *      fn poll (self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output>;
         *      // where enum Poll <T> { Ready(T), Pending, }
         *      // NOTE: this self has a type annotation!
         *      // generally we have fn f(self, ..) or f(&self, ..) or f(&mut self, ..)
         * }
         *
         * // enum Poll <T> { Ready(T), Pending, }
         *
         * == Type annotation on self ==
         * - Can't be any type, has to be a type on which the method is implemented,
         *   a reference or a smart pointer to that type, or a "Pin wrapping a reference to that type"
         */

        /*
         * == Pin and Unpin ==
         * Directly awaiting a future, pins the future implicitly. But in cases, where we don't,
         * like `join_all`, where we are constructing a new future,
         *
         * Box<T> implements Future trait, only if the underlying T is a Future that implements Unpin trait.
         *
         * Pin is a compile-time wrapper for pointer types (like, &, &ref, &mut ref, Box, Rc..
         * basically types that implement Deref or DerefMut), and has no runtime-overhead or any
         * runtime property like Rc (reference counted smart pointer) or others have.
         *
         * By default, object that has reference to itself, is unsafe to move in the memory, since
         * the references still points to the old memory address (which can be stale after move,
         * overwritten or corrupt) -- so make sure that DS that self-references, isn't allowed to
         * move (just like the borrow-checker which doesn't allow move when it has active
         * references).
         *
         * So, when you Pin a reference to a value, the value can no longer move.
         * (NOTE: referencing ds like some smart pointer can still move, but not the underlying memory)
         * Technically, the value is Pin-ned.
         * Eg. Pin<Box<SomeType>>, Compiler ensures the value of type SomeType, refered through the
         * Box smart pointer, is pinned in the memory.
         *
         * Pin is implemented by default for every type by Rust.
         * So, Unpin, tells the compiler that it's safe to move out without worrying about
         * self-references. Others implement !Unpin (NOTE: read exclaimation mark before) trait.
         * (Marker traits, just like Send/Sync, are just a way for compiler to ensure a behaviour
         * for the type in certain context)
         */

        /* == Stream and StreamExt ==
         * Iterator -> next
         * Future -> poll
         * Stream -> poll_next
         * trait Stream {
         *      type Item;
         *
         *      fn poll_next(self: Pin<&mut Self>, ctx: &mut Context<'_>) ->
         *      Poll<Option<Self::Item>> {}
         * }
         *
         * Poll tells whether the next future in this stream is completed or not,
         * Option tells whether we have more elements coming in the stream or not.
         *
         * StreamExt automatically implemented for all types that implement Stream Trait.
         * So you just need to implement Stream trait for some streaming type and StreamExt will be
         * available automatically.
         * StreamExt has some interesting methods.
         */
    }
}
