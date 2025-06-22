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

    pub fn run<F: std::future::Future>(future: F) -> F::Output {
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

    use futures::future::Either;

    use super::*;

    #[test]
    fn basic_async() {
        assert_eq!(
            helpers::run(async { page_title("https://google.com").await }),
            Option::Some(String::from("Google"))
        );
    }

    #[test]
    fn race_in_async() {
        match helpers::run(async {
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
}
