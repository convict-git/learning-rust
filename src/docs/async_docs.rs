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
    use std::{future::Future, pin::pin};

    use super::*;

    pub async fn get(url: &str) -> Response {
        Response(reqwest::get(url).await.unwrap())
    }

    pub fn tokio_rt_block_on<F: std::future::Future>(future: F) -> F::Output {
        // a new tokio runtime is created everytime `run` is called
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(future)
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
        join,
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
                    let random_delay = rand::thread_rng().gen_range(1..=200);
                    {
                        let mut item_mutex_guard = items_counted_mutex_clone.try_lock().unwrap();
                        item_mutex_guard.push(i);
                        // free the lock before await itself
                    }
                    async_sleep(Duration::from_millis(random_delay)).await; // tokio::time::sleep async anologous of std::thread::sleep
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
            let random_delay = rand::thread_rng().gen_range(1..=200);
            async_sleep(Duration::from_millis(random_delay)).await;
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
}
