// Closures

// Traits implemented by closures:
// - FnOnce -> all closures implement atleast this trait, moves captured values out of closure. Can be called only once.
// - FnMut -> mutates values captured in closure, can be called multiple times.
// - Fn -> neither mutates, nor moves, can be called multiple times
//
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
        F: FnOnce() -> T,
    {
        match self {
            MyOption::Some(x) => x,
            MyOption::None => f(),
        }
    }
}

pub fn check() {
    let mut list = vec![1, 2, 3];

    let mut fn_borrows_mutably = || list.push(4);
    // NOTE: fn_borrows_mutably should be binded to a mutable variable. Why?
    /*
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

    let mut list2 = vec![1, 2, 3];
    let mut sort_operations: Vec<String> = Vec::new();
    let v = String::from("Hello world");
    let my_closure = |x: &i32, y: &i32| {
        // sort_operations.push(v); // ERROR
        /* This will turn the function to FnOnce since v is moved (String hence no copy trait).
         * sort_by expects the closure to implement FnMut or Fn since it might be called multiple times
         *
         * One way to fix it is by v.clone()
         * */
        if x < y {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    };

    list2.sort_by(my_closure);

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
