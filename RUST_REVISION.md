# Rust Revision Notes

These are your notes from `src/docs/`, pasted exactly as they are, in the order of [The Rust Programming Language](https://doc.rust-lang.org/book/). After each file there is a **Thoughts** section:

- **Big picture**: the mental model the file is building, in a few lines.
- **Corrections**: places where a comment in the file is wrong or loose. Line numbers (`L12`) point into the pasted block. The pasted code was not changed.
- **Worth adding**: short gaps worth filling while you revise.

Your crate is on **edition 2021** (see `Cargo.toml`) and the toolchain here is rustc 1.98. A few answers depend on the edition, and those places say so.

To run one file's tests with output: `cargo test docs::lifetimes -- --nocapture`. The tests in `async_docs` make real HTTP requests, so skip them when you're offline with `cargo test docs:: -- --skip async_docs`.

| # | File | Book chapter |
|---|------|--------------|
| 0 | `mod.rs` | (module index) |
| 1 | `guessing_game.rs` | Ch 2: Programming a Guessing Game |
| 2 | `match_docs.rs` | Ch 6: Enums & Pattern Matching (+ Ch 19: Patterns) |
| 3 | `packages_crates.rs` | Ch 7: Packages, Crates, Modules (+ Ch 14: Cargo) |
| 4 | `str_fuss.rs` | Ch 8.2: Strings (+ Ch 4.3: Slices) |
| 5 | `lifetimes.rs` | Ch 10.3: Lifetimes |
| 6 | `ownership_inventory.rs` | Brown edition's Ownership Inventory quizzes (needs Ch 10) |
| 7 | `closures_and_fn_pointers.rs` | Ch 13: Closures & Iterators (+ Ch 20: fn pointers) |
| 8 | `smart_pointers.rs` | Ch 15: Smart Pointers |
| 9 | `concurrency.rs` | Ch 16: Fearless Concurrency |
| 10 | `async_docs.rs` | Ch 17: Async, Await, Futures, Streams |
| 11 | `oops.rs` | Ch 18: OOP Features |
| 12 | `design_tradeoffs.rs` | Brown edition's Design Trade-offs (after OOP) |
| 13 | `advanced_features.rs` | Ch 20: Advanced Features |
| 14 | `macros.rs` | Ch 20.5: Macros + Appendix C: Derivable Traits |

---

## 0. `mod.rs`: module index

```rust
pub mod advanced_features;
pub mod async_docs;
pub mod closures_and_fn_pointers;
pub mod concurrency;
pub mod design_tradeoffs;
pub mod guessing_game;
pub mod lifetimes;
pub mod macros;
pub mod match_docs;
pub mod oops;
pub mod ownership_inventory;
pub mod packages_crates;
pub mod smart_pointers;
pub mod str_fuss;
```

### Thoughts

**Big picture**
- `pub mod x;` tells the compiler to load `x.rs` (or `x/mod.rs`) from the `docs/` folder as the module `docs::x`. `src/main.rs` has `mod docs;`, which pulls in this `mod.rs`. Since Rust 2018 you can also write `src/docs.rs` next to a `src/docs/` folder instead of using `docs/mod.rs`. Both work.
- Almost every file wraps its content in `#[cfg(test)] mod ... { }`. That code is compiled only under `cargo test`, which is why it never shows up as dead code in a normal build. Only `guessing_game` and `packages_crates` are used from `main.rs`.
- The test path repeats the module name, for example `docs::lifetimes::lifetimes::annotations_for_functions`. The file is one module and the inner `mod lifetimes { }` is a second one. A plain `#[cfg(test)] mod tests` (the usual convention) would read better, but it isn't wrong.

---

## 1. `guessing_game.rs` (Ch 2)

```rust
use rand::Rng; // included the trait Rng
use std::cmp::Ordering;
pub fn play() {
    let random_number = rand::thread_rng().gen_range(1..=100);
    // loop label - with a single quote
    'guessing_loop: loop {
        println!("Guess the number");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Unable to read from the std input");

        let parsed_input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match parsed_input.cmp(&random_number) {
            Ordering::Less => println!("Guessed less"),
            Ordering::Equal => {
                println!("Guessed right");
                break;
            }
            Ordering::Greater => println!("Guessed more"),
        }
    }
}
```

### Thoughts

**Big picture**
- **A trait must be in scope to call its methods** (L1). `gen_range` is a method of the `Rng` trait, so it only works after `use rand::Rng`. You'll hit this again with `StreamExt` in the async notes.
- **Type inference can flow backwards.** `random_number` has no annotation. It becomes `i32` because it's later compared with `parsed_input: i32` through `cmp(&random_number)`.
- `parse()` is generic over its return type. The `: i32` annotation on L13 picks the type. You could also write `parse::<i32>()` (the "turbofish").
- `match` is an expression, and its arms can have type `!` (never): `continue` on L15 and `break` on L22 never produce a value, so they fit an arm that should produce an `i32`.
- `read_line` **appends** to the buffer, and the input keeps its trailing `\n`. That's why the code creates a new `String` on every loop and calls `.trim()`.

**Corrections**
- L5–6: the label `'guessing_loop` is never used. The plain `break` on L22 already exits the innermost loop, and the compiler warns about the unused label. Labels matter when loops are nested: `break 'outer;` or `continue 'outer;`.

**Worth adding**
- `rand` 0.9 renamed these APIs: `rand::thread_rng().gen_range(..)` became `rand::rng().random_range(..)`. Your crate pins 0.8.5, so the code is fine as it is, but newer docs and examples will use the new names.

---

## 2. `match_docs.rs` (Ch 6, with Ch 19 content)

```rust
// Refer: https://doc.rust-lang.org/reference/expressions/match-expr.html for grammar

#[cfg(test)]
mod match_docs {
    #[test]
    pub fn push_down_reference() {
        let opt1 = Option::Some(String::from("Hello world")); // Here s is moved when pattern matching
                                                              // type of Scrutinee expression opt1 is Option<String>
        match opt1 {
            // NOTE: if we use a wildcard, values aren't moved/copied/borrowed
            // Some(_) => println!("Got something"),
            Some(s) => println!("{}", s), // value on the heap is moved to s out of the opt1
            None => println!("Something else"),
        }

        // println!("{:?}", opt1);
        // ^ ERROR: this is invalid, because we are trying to borrow opt1 which already has a partial moved value since
        // String doesn't implement copy trait

        /* NOTE: that this can also be fixed by explicity binding s as ref, i.e.
        match opt1 {
            Some(ref s) => println!("{}", s),
        This way, s is borrowed (hence opt1 is borrowed partially), which still
        keeps s (and hence opt1) readable after the match scope */

        let opt2 = Option::Some(String::from("Hello world again!"));
        // How can this be fixed? - by borrowing opt2 for s
        match &opt2 {
            Some(s) => {
                // s is pushed down reference to String
                // i.e. &Option<String> to &String
                println!("{}", s) // s is borrowed here
            }
            None => println!("Something else"),
        }
        // Since s was borrowed, we can very well read the value
        println!("{:?}", opt2);
    }

    #[test]
    fn refutability() {
        // Patterns -> Irrefutable (that will match for sure) and refutable (may or may NOT match)
        let opt = Option::<i32>::None;

        // let Some(x) = opt;
        // ^ ERROR: Refutable local binding for x, needs an irrefutable pattern here, since None
        // isn't handled.

        if let Some(x) = opt { // This works, just the compiler warns about refutability
             // .. do something here
        }
    }

    #[test]
    fn extra_conditionals_and_bindings() {
        let opt = Some(2);
        let y = 2;

        match opt {
            Some(x) if x == 2 => println!("A"),
            Some(x) if y == 2 && x == 1 => println!("B"),
            Some(x @ (3 | 4)) => println!("C {x}"),
            Some(x @ 5..=10) => println!("C {x}"),
            Some(_) => println!("D"),
            None => println!("None"),
        }

        match y {
            1 | 2 => println!("x"),
            t @ (3 | 4) if t * y == 10 => println!("{t} {y}"),
            _ => println!("z"),
        }
    }

    #[test]
    fn variable_and_subpattern_binding() {
        // Some nice example of pattern matching and variable @ subpattern binding
        let arr = [1, 2, 3, 4];
        match arr {
            // NOTE: must understand the ownership here.
            // `whole` borrows the whole slice of arr as ref
            // `head` and `last` tried to move but instead copied (since i32)
            // `tail` borrows the partial slice of arr as ref (since .. operator results in borrow)
            whole @ [head, tail @ .., last] => {
                println!("{} {:?} {:?} {}", head, tail, whole, last)
            }
        };
    }
}
```

### Thoughts

**Big picture**
- Matching a **value** with a binding pattern **moves** (or copies) the matched parts out of it. Matching a **reference** binds by reference. That's the core of this file.
- The "pushed down reference" on L28–31 has an official name: **default binding modes** (often called "match ergonomics"). When a non-reference pattern like `Some(s)` meets a reference like `&Option<String>`, the compiler switches the bindings inside to `ref` mode, so `s: &String`.
- **Refutable** patterns can fail to match (`Some(x)`), and **irrefutable** ones can't (`x`, or `(a, b)`). `let` and function parameters need irrefutable patterns. `if let`, `while let` and `match` arms accept refutable ones.
- `@` binds a name to a value while also testing it against a sub-pattern (L62–63). A guard (`if ...`) adds a condition the pattern itself can't express.

**Corrections**
- L12: "value on the heap is moved to s". Nothing on the heap moves. What moves is the `String` struct itself (pointer, length, capacity). Ownership of the heap buffer passes to `s`, and the heap bytes stay where they are. A move is always a shallow copy of the stack part, after which the source can't be used anymore.
- L49: "This works, just the compiler warns about refutability". The warning you saw is **`unused variable: x`**. `if let` exists for refutable patterns, so it never warns about them. The refutability warning goes the other way: `if let x = 5 { }` warns "irrefutable `if let` pattern".
- L80–84: this comment is wrong, and it's worth revisiting. `arr` is an **array** `[i32; 4]`, not a slice, and it's `Copy`. Matching it **by value** gives:
  - `whole: [i32; 4]`, a **copy** of the array, not a reference
  - `tail: [i32; 2]`, also a copy. `..` does **not** create a borrow.
  - `head`, `last: i32`, copies

  I compiled this and printed the types: `[i32; 4] i32 [i32; 2] i32`. To get borrows, match on a reference: `match &arr { ... }` or `match &arr[..] { ... }`. Default binding modes then make `whole: &[i32; 4]` (or `&[i32]`), `tail: &[i32]` and `head: &i32`.

**Worth adding**
- **let-else** (stable since Rust 1.65) is the modern answer to the commented-out `let Some(x) = opt;` on L45: `let Some(x) = opt else { return; };`. The `else` block must diverge (`return`, `break`, `panic!`, ...).
- A guard applies to the **whole** arm: `1 | 2 if cond` means `(1 | 2) if cond`.
- `matches!(value, Some(3..=5))` gives you a `bool` without writing a full `match`.

---

## 3. `packages_crates.rs` (Ch 7, plus a note from Ch 14)

```rust
pub mod back_of_house {
    // in structs, each fields are still private unless explicitly mentioned
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub struct Point(pub i32, i32); // can't access p.1 outside

    // whereas, in enums, once enum is public, all the variants are public as well
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast_name: &str) -> Breakfast {
            // static method as it doesn't take self
            Breakfast {
                toast: String::from(toast_name),
                seasonal_fruit: String::from("Apple"),
            }
        }
    }
}

// Re-exporting as different name
// (in rust terms, public re-export with an alias, or also known as API forwarding)
pub use back_of_house::Appetizer as HouseAppetizer;

/* NOTE: use -> importing a name into local namespace
 * pub use -> re-exporting that name from your module, so others can import */

pub fn eat_at_rest() {
    let breakfast = back_of_house::Breakfast::summer("brown");
    println!("{:?}", breakfast);
    // println!("{}", breakfast.seasonal_fruit) // this will break since seasonal_fruit is a private field

    // let initial_order = back_of_house::Appetizer::Soup; // OR
    let initial_order = HouseAppetizer::Soup;
    println!("{:?}", initial_order);
}

/*
// use are scoped
use back_of_house::Breakfast; // local namespace

mod some_mod { // some_mod namespace
   // here you cannot use Breakfast directly, because Breakfast is valid in local namespace but not
   // some_mod namespace
}
*/

/*
use std::io;
use std::io::Write;

// merge these two:
use std::io::{self, Write};

// include all :
use std::collections::*;
 */

/*
 * Release profiles: predefined and customizable profiles for different build configurations
 * Workspace: set of packages that share the same Cargo.lock and output dir (better to refer the
 * docs when in doubt)
 */
```

### Thoughts

**Big picture**
- Privacy is at the **module** level, not the type level. Anything in the same module (or a child module) can see private items. Struct fields are private by default. Enum variants are public once the enum is public (L11).
- A struct with any private field **can't be built with a literal** outside its module. That's why `Breakfast::summer` exists: a constructor is the only way in.
- `use` makes a name available **in the current module (or block) only**. Child modules don't inherit it (L47–53). Inside `some_mod` you'd need `use super::Breakfast;` or `super::back_of_house::Breakfast`.
- `pub use` re-exports: you can expose a clean public path that doesn't mirror your internal file layout. `std::collections::HashMap` is itself a re-export.

**Corrections**
- L20: "static method as it doesn't take self". The Rust term is **associated function**. It's called with `Type::f()`. A method takes `self` and can be called with `value.f()`.

**Worth adding** (the file name says "packages & crates" but never defines them)
- **Package**: a folder with a `Cargo.toml`. It holds **at most one library crate** (`src/lib.rs`) and any number of binary crates (`src/main.rs`, `src/bin/*.rs`).
- **Crate**: one compilation unit, with a tree of modules rooted at `main.rs` or `lib.rs`.
- **Paths**: `crate::` (from the crate root), `self::`, `super::` (from the parent module). Visibility options include `pub`, `pub(crate)` and `pub(super)`. `pub(crate)` is very common in real code.

---

## 4. `str_fuss.rs` (Ch 8.2, building on Ch 4.3 slices)

```rust
// All the fuss about str in a single doc
#[cfg(test)]
mod str_fuss {
    /*
    `str` is **Unsized primitive type** (represents utf-8 encoded strings)
     Other examples of Unsized are `[T]`, `str`, `dyn Trait`.

     #[lang = "str"] // this tells it is compiler iternals
     struct str {  } // stores u8 but guarantees utf-8

     And like all other unsized types you can't work with them directly,
     instead you use fat pointers to get hold of them {Dynamically Sized Type (DST) mechanics}
     Hence, &str isn't just a pointer to a str struct, instead it's a fat pointer.
     (
        *const u8, // actual memory address of the first u8 byte
        usize // number of bytes (not characters!)
     )

     Import point to note here is that technically str actually leads to u8 slice i.e. [u8]
     just that the u8's in str are bound to be utf-8 compatible and the invariant
     is maintained and assumed everywhere in the compiler as well.

     `.chars()` Iterator on &str emits chars, which won't just iterate on byte level offsets
     but rather valid utf-8 chars! for byte level iteration, we can use `.bytes()`

     indexing on &str is invalid because of utf-8 guarantee violations

     `.as_ptr()` gives `*const u8`, access to the actual memory address of the first byte

     // let s: &str = "hello";
     // let p: *const u8 = s.as_ptr();

     All string literals have `static lifetime since they are heap allocated
     (and stored in a read-only memory) and they stay alive till the program completes
    */

    /* ====================================================================
     * Quick NOTE about == fat pointers ==
     * low-level compiler generated representation of a pointer which
     * carriers some metadata along with it
     *
     * Eg.
     * # fat pointer for &[T] (reference to a slice of T array)
     *   (
     *      data_ptr,  // actual memory address,
     *      length     // length of the slice
     *   )
     *
     * # fat pointer of &dyn Trait (reference to trait object)
     *   (
     *      data_ptr,  // actual memory address of the underlying trait object,
     *      vtable_ptr // used for dynamic dispatch
     *   )
     * ====================================================================
     */

    /*
     `String` on the other-hand is heap allocated vector encapsulation of u8 with utf-8 guarantees
     struct String {
       vec: Vec<u8>;
     }

     We can pass a &String even when &str is expected &str because String struct
     implements Deref trait which allows String to be dereferenced to some other type.
     This is just a pointer and length pass-through and involves no data copy.

     // impl Deref for String {
     //   type Target = str;
     //   fn deref(&self) -> &Self::Target {
     //       &self[..] // slice of the whole string
     //   }
     // }
    */
}
```

### Thoughts

**Big picture** (this is one of your strongest files)
- `str` is an **unsized** (dynamically sized, DST) type. You always handle it through a pointer: `&str`, `Box<str>`, `Rc<str>`.
- `&str` is a **fat pointer**: (address, length in **bytes**). `&[T]` is (address, length). `&dyn Trait` is (address, vtable pointer).
- `String` is a `Vec<u8>` that guarantees valid UTF-8. The `(ptr, capacity, len)` part lives wherever the `String` value lives. The bytes live on the heap.
- `&String → &str` is **deref coercion**, and it's free: the pointer and length are reused, and nothing is copied.

**Corrections**
- L33–34: "string literals have `'static` lifetime since they are **heap allocated**". This is wrong, and you repeat it in `concurrency.rs`. String literals are **never on the heap**. The compiler stores their bytes in the binary's **read-only data section**, which exists for the whole run of the program. That's why `&'static str` works.
- L26: "indexing on &str is invalid". It's half right:
  - `s[0]` (a single integer index) doesn't compile, because `str` doesn't implement `Index<usize>`. A byte isn't a character, and a character isn't a fixed width.
  - `&s[0..3]` (a range) **does** compile. It **panics at runtime** if a range end isn't on a character boundary. This catches people all the time.
- L23–24: `.chars()` yields **Unicode scalar values** (`char`, always 4 bytes), not "characters" as a person reads them. For example, `"é"` can be two `char`s: `e` followed by a combining accent. User-visible characters (grapheme clusters) need the `unicode-segmentation` crate.
- L8–9: the `struct str {}` is a useful mental model only. `str` is a built-in primitive type, and nothing like that struct exists in `std`.

**Worth adding** (quick reference)
- For function parameters, prefer `&str` over `&String`. It accepts both `&String` (through deref) and string literals.
- To get an owned string: `String::from("x")`, `"x".to_string()`, `"x".to_owned()`, or `format!(...)`.
- `s1 + &s2` **moves** `s1`. `Add` is implemented as `String + &str -> String`. Use `format!("{s1}{s2}")` when you want to keep both.
- `Cow<'_, str>` (covered in the smart pointers notes) means "borrowed if I didn't need to change it, owned if I did".

---

## 5. `lifetimes.rs` (Ch 10.3)

```rust
#[cfg(test)]
mod lifetimes {
    #[test]
    fn annotations_for_functions() {
        // validating references with lifetimes
        // every reference in a rust has a lifetime
        /*
         // ERROR: This function returns a ref which means a borrowed value:
         // Now it has to be borrowed either from x OR y (OR global?), why?
         // because it cannot return a borrowed value from the current scope as it will outlive it.
         // So if the returned value if borrowed from x OR y, the return value has to be intersection of
         // the lifetimes of x AND y (i.e. the lifetime in which both were valid).
        fn longest(x: &str, y: &str) -> &str {
            if x > y {
                x
            } else {
                y
            }
        }
        */
        // Generic lifetime parameters -> must start with apostrophe (')
        //
        // declare just like generic types inside <>
        // this will take the smaller (intersection) lifetime as 'a
        // also, this doesn't mean that returned value has EXACTLY the lifetime 'a, but instead ATLEAST 'a
        // This isn't to enforce this lifetime, instead just help the borrow checker invalidate the usage
        // which breaks ATLEAST 'a
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let x = String::from("Hello world");
        let z;
        {
            let y = String::from("bye world");
            // remember deref coerc, hence &String -> &str conversion
            z = longest(&x, &y);
            // y is dropped here, hence anything that borrows from y is invalidated after this scope ends
            println!("{z}"); // This is fine
            assert_eq!(z, "Hello world");
        }

        // println!("{z}");
        // ^ ERROR: here z borrows a value which has the lifetime that
        // is same as of y which is already dropped when its scope ended, and z can be possibly
        // holding an invalid reference at this point
    }

    #[test]
    fn annotations_for_structs() {
        // Lifetime annotions for structs -> if the value held by the struct is a reference (and NOT owned by the struct itself)
        #[derive(Debug)]
        struct ImportantExcerpt<'a> {
            part: &'a str, // helds a reference to a str, hence ALWAYS needs a lifetime specifier!
        }

        impl<'a> ImportantExcerpt<'a> {
            fn get_first_dot_splitted(from: &'a str) -> Option<Self> {
                match from.split('.').next() {
                    Some(valid_first_str) => Some(ImportantExcerpt {
                        part: valid_first_str,
                    }),
                    _ => None,
                }
            }
        }

        let x = String::from("Hello world. How are you doing?");
        let ie_instance = ImportantExcerpt::get_first_dot_splitted(&x);
        println!("{:?}", ie_instance);

        pub fn first_word(input: &str) -> &str {
            // let x = String::from("World").as_str(); // ERROR: references a temp value that is dropped as soon as the scope ends
            let x = "Hello"; // but this has 'static lifetime as by default strings in rust have 'static lifetime
                             // 'static -> lives in entire lifetime
            return x;
        }

        // Three rules for lifetimes elision(omitting):
        //  1. Compiler assigns different lifetime parameters to lifetime in the input
        //    - fn f(foo: &'_ i32) //  one lifetime paramter by default
        //    - fn f(foo: &'_ i32, bar: &'_ f32) //  two lifetime parameters by default
        //    - fn f(foo: &'_ ImportantExcerpt<'_>) // NOTE: two lifetime parameters by default
        //  2. If exactly, one input lifetime param, than that's used for all output lifetime params as well
        //    - fn f<'a>(foo: &'a i32) -> &'a i32
        //  3. If multiple input lifetime params, one with self is used for all output lifetime params

        // ** Ref ** : https://doc.rust-lang.org/reference/lifetime-elision.html

        // Lifetime annotations for method definitions

        impl<'a> ImportantExcerpt<'a> {
            // here output lifetime paramter, Option<&'_ str> is allowed to be omitted and is same as of self parameter
            fn strip_prefix_from_excerpt(&self, with_str: &str) -> Option<&str> {
                // Some(with_str) ERROR: was supposted to return the data with lifetime of self
                self.part.strip_prefix(with_str)
            }
        }

        // Static lifetime
        // All string literals (hard-coded strings in the program) have static lifetime, and they live for the entirity of the program
        const STR: &'static str = "Hello world";
        // NOTE: The above 'static just means that the memory allocated for this reference will never be deallocated
        // NOT that it is in static part of the memory region or something!
    }
}
```

### Thoughts

**Big picture**
- Lifetime annotations **describe relationships between references. They don't change how long anything lives.** `longest<'a>` says the result borrows from something that both `x` and `y` outlive.
- At a call site, the compiler picks the shortest region that fits all the arguments. That's your "intersection" intuition, and it's a good way to think about it.
- The borrow checker **only looks at signatures, never at values.** On L44, `z` actually points into `x` (`"Hello world"` is longer), yet L47 is still rejected. The signature says the result may borrow from `y`, and the checker doesn't run the `if`.
- The elision rules on L83–90 are right. Every reference parameter gets its own lifetime. If there's exactly one input lifetime, the output gets it. If there's a `&self` or `&mut self`, the output gets self's lifetime.

**Corrections**
- L78: "by default **strings** in rust have `'static` lifetime". This applies only to **string literals**. `String` values and most `&str` values aren't `'static`.
- L97–101: the omitted output lifetime is the lifetime of the **`&self` borrow**, not `'a`. Those two are different, and it's a trap:

  ```rust
  fn strip(&self, p: &str) -> Option<&str>      // tied to the borrow of the ImportantExcerpt
  fn strip(&self, p: &str) -> Option<&'a str>   // tied to the original text: can outlive the excerpt
  ```

  `self.part` is `&'a str`, so the second version compiles too and is usually more useful. Write `'a` explicitly when callers need the result to live longer than the struct.
- L107–108: "`'static` just means ... never be deallocated, NOT that it is in static memory". The direction is right: `&'static T` means "valid until the program ends", and a leaked `Box` (`Box::leak`) also qualifies. But string literals **are** in static (read-only) memory.
- L106: `'static` is implied in `const` and `static` items, so `const STR: &str = "..."` means the same thing.

**Worth adding** (it will matter in the threads and async notes)
- **`T: 'static` is not the same as `&'static T`.** `T: 'static` means "`T` contains no borrowed data that could expire". Every **owned** type meets it: `String`, `Vec<i32>`, `Box<dyn Fn()>`. It does **not** mean the value lives forever. `thread::spawn` and `tokio::spawn` require `F: 'static`, and that's why moving owned data into the closure works.

---

## 6. `ownership_inventory.rs` (Brown University edition quizzes)

These quizzes come from the interactive edition at rust-book.cs.brown.edu. The official book doesn't have them. `module_4` needs lifetimes, `impl Trait` and trait objects, so this file sits after Ch 10.

```rust
#[cfg(test)]
mod ownership_inventory {
    mod module_3 {
        #[test]
        fn struct_fields() {
            struct TestResult {
                /// Student's scores on a test
                scores: Vec<usize>,

                /// A possible value to curve all scores
                curve: Option<usize>,
            }

            impl TestResult {
                pub fn get_curve(&self) -> &Option<usize> {
                    &self.curve
                }

                /// If there is a curve, then increments all scores by the curve
                pub fn apply_curve(&mut self) {
                    // if let Some(curve) = self.get_curve() {
                    // ^ ERROR: self.get_curve already borrows the whole Self immutably, and hence iter_mut
                    // cannot borrow self mutably. Instead, if we use self.curve, compiler will know
                    // that we are doing partial borrow limited to curve.
                    // NOTE: This is a commonly used approach for borrow checker issues with structs.
                    if let Some(curve) = self.curve {
                        for score in self.scores.iter_mut() {
                            // *score += *curve;
                            *score += curve;
                        }
                    }
                }
            }
        }

        #[test]
        fn sliced_refs() {
            fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
                // elems.sort();
                /* ^ ERROR: cannot borrowed as mutable since elems is behind & ref
                 * impl<T> [T] pub fn sort(&mut self) where T: Ord, */

                let mut elem_refs: Vec<&T> = elems.iter().collect();
                // collect the refs in another vector and sort that instead
                elem_refs.sort();

                let t = &elems[n];
                return t.clone();
            }
        }
    }

    mod module_4 {
        #[test]
        fn lifetime_annotation_basic() {
            fn concat_all(
                iter: impl Iterator<Item = String>,
                s: &str,
            ) -> impl Iterator<Item = String> {
                // iter.map(move |s2| s2 + s)

                /* ^Error: breaks because lifetime captured with s isn't appearing the bounds of the
                 * returned type. Compiler needs to know that since you are using s: &str, the
                 * returned value lives long enough as s lives.*/

                let s = s.to_owned(); // So either we can own this and use the &str from
                                      // the stack memory OR annotate the lifetimes (see concat_all2)
                iter.map(move |s2| s2 + &s)
            }

            // NOTE: Above example with lifetime annotation, which is better because this doesn't
            // have any runtime overhead like to_owned has
            fn concat_all2<'a>(
                iter: impl Iterator<Item = String> + 'a,
                s: &'a str,
            ) -> impl Iterator<Item = String> + 'a {
                iter.map(move |s2| s2 + s)
            }
        }

        #[test]
        fn lifetime_annotation_trait_objects() {
            fn trait_object() {
                use std::fmt::Display;

                // fn add_displayable<T: Display>(v: &mut Vec<Box<dyn Display>>, t: T) {

                /* ^ Error: This breaks because when a trait object from `t` is pushed in the vector,
                 * compiler wants to ensure that the trait object outlives the vector. */
                fn add_displayable<'a, T: Display + 'a>(v: &mut Vec<Box<dyn Display + 'a>>, t: T) {
                    // NOTE: that we have specified lifetime for the trait object and not the vec mut ref
                    v.push(Box::new(t));
                }
            }
        }
    }
}
```

### Thoughts

**Big picture**
- Calling a method that takes `&self` borrows the **whole** struct for as long as its result is used. Accessing a field directly (`self.curve`) borrows only that field, so the compiler can see that `self.curve` and `self.scores` don't overlap. That's the **disjoint field borrow**, the term `CONTEXT.md` asks you to use instead of "partial borrow".
- If you can't mutate data you only borrow, sort **references** to it instead (L43–45). You get the ordering without cloning or mutating anything.
- `impl Trait` in return position is an opaque type, and it must say which borrows it holds (the `+ 'a` in `concat_all2`).

**Corrections**
- L22–24: the explanation is right, but in this code the fix works for an even simpler reason. `self.curve` is `Option<usize>`, which is `Copy`, so `if let Some(curve) = self.curve` **copies** the `usize` out. No borrow of `self` stays alive during the loop. The disjoint-field version would be `if let Some(curve) = &self.curve { ... *score += *curve }`, and it compiles too.
- L47: **there's a logic bug.** The function sorts `elem_refs` and then ignores it: `&elems[n]` indexes the **unsorted** input. It should be `let t = elem_refs[n];` (already a `&T`) followed by `t.clone()`. The tests pass only because they declare the inner functions and never call them. Most tests in these notes only check that the code **compiles**.
- L62–64: right for **edition 2021**, which your crate uses. In **edition 2024**, a return-position `impl Trait` automatically captures every lifetime in scope. I compiled the commented-out `concat_all` under `--edition 2024` and it builds. Rust 1.82 also added explicit capture syntax: `-> impl Iterator<Item = String> + use<'a>`.
- L66–67: "own this and use the &str from the **stack memory**". That's not what happens. `to_owned()` makes a new **heap** `String`, the `move` closure takes ownership of it, and `&s` borrows from the closure's own captured `String`. Nothing points into stack memory.
- L88–89: "compiler wants to ensure that the trait object outlives the vector". The exact reason: `Box<dyn Display>` with no lifetime written **defaults to `Box<dyn Display + 'static>`** (the default trait-object lifetime). A generic `T` could hold short-lived references, so `T` doesn't qualify. You can fix it either with `T: Display + 'static` or with the `'a` version you wrote.

---

## 7. `closures_and_fn_pointers.rs` (Ch 13, plus fn pointers from Ch 20)

```rust
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
```

### Thoughts

**Big picture**
- Which `Fn*` trait a closure implements depends on **what its body does with the captured values**:
  - It **moves them out**: `FnOnce` only.
  - It **mutates** them: `FnMut` (and `FnOnce`).
  - It **only reads** them: `Fn` (and `FnMut`, `FnOnce`).

  Every `Fn` is also `FnMut`, and every `FnMut` is also `FnOnce`.
- **`move` changes *how* values are captured, not which trait the closure implements.** A `move || println!("{v:?}")` closure owns `v` and is still `Fn`, because it only reads `v`.
- For a *parameter bound*, `FnOnce` is the most permissive choice for the **caller** (L26): any closure qualifies. It's the strictest for the **callee**, which may call it only once. Pick the weakest bound your function actually needs.
- Iterators are lazy. Adapters like `map` do nothing until a consumer like `collect`, `sum` or `for` pulls items (L146–147).

**Corrections**
- L7: "FnOnce ... moves captured values out of closure". That describes a closure that is `FnOnce` **only**. As you say in the same sentence, every closure is at least `FnOnce`.
- L64–69: the reasoning is right. On the wording: `F: 'static` means the closure holds no **borrowed** data that could expire (see the `T: 'static` note in the lifetimes section). "Owned by the closure" meets that requirement. Also see `std::thread::scope`, which lets threads borrow local data safely.
- L81–97: this comparator never returns `Equal`, which isn't a valid total order. Since Rust 1.81, `sort_by` **may panic** if it detects an inconsistent comparator. It's fine with `[1, 2, 3]` because there are no equal elements, but in real code write `x.cmp(y)`.
- L107–109, your open question ("what happens without the lifetimes?"). In **edition 2021**, `fn make_cloner(s_ref: &str) -> impl Fn() -> String` fails with *"hidden type captures lifetime that does not appear in bounds"*, so you need `+ '_` or `+ 'a`. In **edition 2024**, it compiles as written, because the new capture rules add the lifetime automatically. I checked both editions.
- L159–160: "closures ... don't have concrete opaque types, so the return types are **trait objects** instead". **This is wrong.** Every closure has its **own concrete type**, which is anonymous and can't be named. `impl Fn(...)` is an **opaque type**: the caller can't see the concrete type, but it is still static dispatch with no allocation. Only `dyn Fn` is a trait object.
- L170–173: the two `impl Fn` types really are different types, and that part is right. They are **not** trait objects, though, and their sizes **are** known at compile time. The reason `vec![...]` fails is that a `Vec` needs **one element type**. `Box<dyn Fn>` **erases** the concrete types into a single one, and that's why it works.

**Worth adding**
- A closure that **captures nothing** coerces to a function pointer: `let v: Vec<fn(i32) -> i32> = vec![|x| x + 1, |x| x - 1];` compiles, with no `Box` needed. It fails once the closures are hidden behind `impl Fn`, because the opaque type hides the fact that nothing is captured.
- L207: the Rust term is **initializer function**: `Status::Value` is a function `fn(i32) -> Status`. Tuple struct names work the same way.
- `iter()` yields `&T`, `iter_mut()` yields `&mut T`, and `into_iter()` yields `T`. But `(&v).into_iter()` yields `&T`, and that's what `for x in &v` uses.

---

## 8. `smart_pointers.rs` (Ch 15)

```rust
#[cfg(test)]
mod tests {
    mod basics {
        // == Smart pointers ==
        /* Rust already has & (references, borrows the value they point to)
         * Smart pointers have additional metadata and functionalities over & and pointers
         * - references just borrow values, smart pointers in most cases owns the value they point to.
         * Eg. String and Vec<_>
         *
         * == Box ==
         * Box<T> -> lightest smart pointer, storing value of type T, keeping it in heap. only the pointer stays on
         * the stack. No performance overhead. Useful when:
         *
         * - size not known at compile time, but use a value of that type in a context where you need the
         * exact size (like recursive type)
         * - large amount of data, but don't want to copy the data when transfering the ownership
         * - when you want to own a value, but all you can is that type implements certain traits rather
         * than being of a specific type (also known as trait object) (? ToDo: need more explanation)
         *
         * Box implements Deref, and Drop traits -- useful as smart pointers (ToDo: More on this later!)
         * */
        #[test]
        fn test() {
            let b = Box::new(4);
            // after b's scope end, both the the pointer (on the stack) and the value (on the heap) are dropped

            println!("{:?}", *b);

            let mut n = 5;
            let x = &mut n;
            {
                let _b2 = Box::new(x); // b2 is on stack, value of x is moved.
                                       // b2 is dropped, the value of x, i.e. moved mutable reference to n on heap is also dropped
            }
            // println!("{}", x); // Error: This breaks value of x was moved out
            assert_eq!(n, 5); // This still works
        }
    }

    mod my_box {
        use std::ops::Deref;

        struct MyBox<T>(T); // a generic tuple struct

        impl<T> MyBox<T> {
            pub fn new(t: T) -> Self {
                MyBox(t) // still haven't figured out the heap allocation part
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T; // Associated type

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[test]
        fn test_deref() {
            let x = 5;
            let b = MyBox::new(x);
            // println!("{}", b);
            assert_eq!(5, x);
            assert_eq!(5, *b); //
            let _t = *(b.deref()); // similar to *b
            let c = |x: &str| println!("{}", x);

            let s = String::from("Hello");
            let bs = MyBox::new(s);

            c(&bs); // passes &MyBox<String>, but c expects &str
                    // &MyBox<String> -> &String -> &str: rust does this for us, using Deref coercion
                    // We have already provided deref for &MyBox<T> to &T
                    // Rust provides deref for &String to &str

            let ss = &(*bs)[..]; // if rust didn't give deref for &String to &str
                                 // *bs (&String) // & [..] -> str slice for whole string
            c(ss);

            /*
             * NOTE: good part is that all deref is compile time computation from rust compiler hence no runtime cost
             *
             * Also, deref cannot be implemented for more than one Target.
             * Hence, rust compiler has exactly one deref path to try till it reached the desired method parameters.
             *
             * We can also use DerefMut for mutable dereferences, i.e. &mut self -> &mut Self::Target
             *
             * NOTE:
             * &T and &mut T -> can be dereffed to &U, when T: Deref<Target=U>
             * &mut T -> can be dereffed to &mut U, when T: DerefMut<Target=U>
             */
        }

        // Drop Trait -> what happens to the value when the owner dies (value goes out of the scope)
        //
        // why implement Drop trait for smart pointers?
        // customize what happens to the referenced value when the pointers goes out of the scope,
        // like in Box<T>, who will have to free-up the memory allocated on the heap
        //
        impl<T> Drop for MyBox<T> {
            fn drop(&mut self) {
                println!("Drop for MyBox called for MyBox");
            }
        }
        /* Some pointers about Drop Trait:
         * we cannot call .drop(). This is enforced since rust anyway calls the drop for the value at the end of its scope (RAII from C++),
         * and it can cause double free error (freeing already freed memory)
         * instead we can use std::mem::drop() for intentional drop (some use cases are freeing up locks when used, and not waiting for it to go out of scope),
         */
        #[test]
        fn test_drop() {
            let b = MyBox::<i32>::new(3);

            drop(b); // moves

            // drop(b); // NOTE: You cannot call drop twice! drop moves the value here since MyBox<T>
            // doesn't implement Copy trait

            /*
             * But wait:
             * does it make sense? A struct cannot implement both Copy and Drop trait together by design.
             * Why? First, it's very important to understand the difference between Copy and Clone.
             * Copy is bit-wise copy (i.e. memory is duplicated), it might look fine for primitives but
             * think about a MyTupleStruct(Box<T>). It's copy will copy the value of Box<T> pointer bitwise.
             * Dropping will lead to drop on the same memory ref.
             *
             * Hence rust doesn't allow Copy and Drop trait to be implemented simultaneously
             */

            /*
             * Some ways to drop: (// anything that moves out the value and/or drop due to out of scope or explicit drop)
             * let mut s = String::new();
             *
             * drop(s);
             * (|_| ())(s);
             * { s };
             */
        }
    }

    mod reference_counted_sp {
        // == Rc<T> The Reference counted Smart Pointer ==
        // - Multiple possible owners of the same value (similar to immutable borrows)
        // - Useful when can't decide the last ownership at compile time (mostly an espace hatch for lifetimes hell)
        // - Only for single-threaded systems

        use std::{fmt::Display, rc::Rc};

        /* Linkedlist:
         *
        // enum List {
        //     Cons(i32, Box<List>),
        //     Nil,
        // }
        // let nil = Box::new(List::Nil);
        // let a = Box::new(List::Cons(4, nil));
        // let b = Box::new(List::Cons(3, a));
        // let c = Box::new(List::Cons(2, a)); // breaks because a was moved
         */
        #[test]
        fn test_box_ref() {
            // So instead we can keep references here, and also have to specify lifetimes
            enum ListRef<'a> {
                Cons(i32, &'a Box<ListRef<'a>>),
                Nil,
            }
            let nil = Box::new(ListRef::Nil);
            let a = Box::new(ListRef::Cons(4, &nil));
            let b = Box::new(ListRef::Cons(3, &a));
            let c = Box::new(ListRef::Cons(2, &a));
            let d = Box::new(ListRef::Cons(5, &c));
            // This works fine as far as we aren't going to mutate anything (since we are borrowing immutably),
            // Currently it looks trivial because everything in a single scope.
            // But 'a should live long enough, and cases, where you would want to return this data-structure, pass to different threads,
            // This won't scale very well and we will be stuck in lifetime hell.
        }

        // So we will use Rc<T>
        #[test]
        fn test_rc() {
            enum List {
                Cons(i32, Rc<List>),
                Nil,
            }

            impl Display for List {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    if let List::Cons(x, next) = self {
                        write!(f, "{} -> {}", x, *next)
                    } else {
                        write!(f, "Nil")
                    }
                }
            }

            let a = Rc::new(List::Cons(4, Rc::new(List::Nil)));
            // clone the smart pointer of 'a' for shared reference, and pass it to 'b'
            // NOTE: We should not do a manual a.clone() here, since Rc::clone will also handle the
            // reference counting logic. We should follow their API to keep the invariant maintained
            // Also, one interesting take,
            // when figuring out perf issues, we can safely ignore Rc::clone calls, but not _.clone()
            let b = Rc::new(List::Cons(3, Rc::clone(&a)));
            let c = Rc::new(List::Cons(1, Rc::clone(&a)));

            let get_ref_counts = || {
                [
                    Rc::strong_count(&a),
                    Rc::strong_count(&b),
                    Rc::strong_count(&c),
                ]
            };
            assert_eq!(get_ref_counts(), [3, 1, 1]);
            {
                let _d = Rc::new(List::Cons(2, Rc::clone(&a)));
                assert_eq!(get_ref_counts(), [4, 1, 1]); // increased strong ref count for a
            }
            assert_eq!(get_ref_counts(), [3, 1, 1]); // reference count reduce since _d died
        }
    }

    mod interior_mutability {
        // == Interior Mutability Pattern == (allow mutation inside an immutable value)
        /* allow mutating the value, even when the value is borrowed immutably, against the rust's
         * borrowing rules (unsafe code behind the scenes but giving safe APIs).
         * Ensure the checker rules are manually handled by us correctly, since the borrowing rules
         * are checked at runtime (instead of compile-time) and program will panic if rules are breached.
         * Static Analysis of the rust compiler is conservative and more restrictive and might reject
         * safe programs but never accept any unsafe program.
         */

        /* smart pointers:
         * // compile-time borrow checker:
         * Box<T> (value on heap, pointer on stack, no perf overhead, data size not known at compile time,
         * both mut/immutable borrow),
         * Rc<T> (reference counted, like immutable borrow but with multiple owners, NO interior mutability, single threaded),
         * // run-time borrow checker:
         * RefCell<T> (reference counted, interior mutability, single threaded, mut/immutable borrows at
         * runtime, single owner)
         */
        mod limit_tracker {
            pub trait Messenger {
                fn send(&self, msg: &str);
            }

            pub struct LimitTracker<'a, T: Messenger> {
                messenger: &'a T,
                value: usize,
                max: usize,
            }

            impl<'a, T> LimitTracker<'a, T>
            where
                T: Messenger,
            {
                pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
                    LimitTracker {
                        messenger,
                        value: 0,
                        max,
                    }
                }

                pub fn set_value(&mut self, value: usize) {
                    self.value = value;

                    let percentage_of_max = self.value as f64 / self.max as f64;

                    if percentage_of_max >= 1.0 {
                        self.messenger.send("Error: You are over your quota!");
                    } else if percentage_of_max >= 0.9 {
                        self.messenger
                            .send("Urgent warning: You've used up over 90% of your quota!");
                    } else if percentage_of_max >= 0.75 {
                        self.messenger
                            .send("Warning: You've used up over 75% of your quota!");
                    }
                }
            }
        }

        use std::cell::RefCell;

        use limit_tracker::{Messenger, *};
        struct MockMessenger {
            // sent_messages: Vec<String>,
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> Self {
                MockMessenger {
                    // sent_messages: vec![],
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, msg: &str) {
                // self.sent_messages.push(String::from(msg));
                // // since self is borrowed immutably, we can't push, hence we need interior mutability.
                self.sent_messages.borrow_mut().push(String::from(msg));
            }
        }
        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
            limit_tracker.set_value(80);

            // assert_eq!(mock_messenger.sent_messages.len(), 1);
            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        }
    }

    mod list_with_rc_refcell {
        // == Rc<RefCell<T>> ==, Multiple owners of mutable data
        /* Rc<T> -> provides immutable borrows with multiple owners
         * RefCell<T> -> provides interior mutability (mutability to immutable value)
         *
         * so we can get multiple owners mutability access to an immutable value using Rc<RefCell<T>>
         */

        use std::{cell::RefCell, rc::Rc};

        // Aim: List which has mutability on both values and structure
        enum List {
            // Cons(i32, Box<List>), // this would break when value will move out
            // Cons(i32, Rc<List>), // This does allow multiple owners but only immutable borrows
            Cons(i32, Rc<RefCell<List>>),
            Nil,
        }

        impl List {
            fn get_wrapped_nil() -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(Self::Nil))
            }

            fn get_wrapped_list(value: i32, next: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
                // value is copied, and we Rc clone the SP given through next
                Rc::new(RefCell::new(Self::Cons(value, Rc::clone(next))))
            }

            fn update_next(&mut self, new_next: &Rc<RefCell<Self>>) -> Result<(), anyhow::Error> {
                match self {
                    Self::Cons(_, ref_cell) => {
                        // - clone the Rc and use it
                        // - drop the existing Rc (otherwise memory leak) -- happens automatically RAII
                        *ref_cell = Rc::clone(new_next);
                        Ok(())
                    }
                    Self::Nil => Err(anyhow::anyhow!("Can't update_next on Nil instance of List")),
                }
            }

            fn update_value(&mut self, new_value: i32) -> Result<(), anyhow::Error> {
                match self {
                    Self::Cons(old_value, _) => {
                        *old_value = new_value;
                        Ok(())
                    }
                    Self::Nil => Err(anyhow::anyhow!(
                        "Can't update_value on Nil instance of List"
                    )),
                }
            }
        }

        impl Drop for List {
            fn drop(&mut self) {
                println!(
                    "Drop called for {}",
                    if let List::Cons(value, _) = self {
                        value.to_string()
                    } else {
                        String::from("Nil")
                    }
                );
            }
        }

        #[test]
        fn test() {
            let nil = List::get_wrapped_nil();
            let a = List::get_wrapped_list(1, &nil);
            let b = List::get_wrapped_list(2, &a);
            let c = List::get_wrapped_list(3, &a);
            let get_ref_counts = || {
                [
                    Rc::strong_count(&a),
                    Rc::strong_count(&b),
                    Rc::strong_count(&c),
                ]
            };

            // b -> a <- c
            // change it to b <- a <- c, i.e. b's next to nil, a's next to b
            assert_eq!(get_ref_counts(), [3, 1, 1]);
            (*b).borrow_mut().update_next(&nil);
            assert_eq!(get_ref_counts(), [2, 1, 1]);
            (*a).borrow_mut().update_next(&b);
            assert_eq!(get_ref_counts(), [2, 2, 1]);
            (*c).borrow_mut().update_value(3);
        }
    }

    mod list_with_refcell_rc {
        use std::{cell::RefCell, rc::Rc};

        // better way, RefCell<Rc<List>>
        enum List {
            Cons(RefCell<i32>, RefCell<Rc<List>>),
            Nil,
        }

        impl List {
            fn update_value(&self, next_value: i32) -> Result<(), anyhow::Error> {
                match self {
                    Self::Cons(value, _) => {
                        *value.borrow_mut() = next_value;
                        Ok(())
                    }
                    Self::Nil => Err(anyhow::anyhow!(
                        "Can't update_value on Nil instance of List"
                    )),
                }
            }

            fn update_next(&self, new_next: &Rc<Self>) -> Result<(), anyhow::Error> {
                match self {
                    Self::Cons(_, ref_cell) => {
                        *ref_cell.borrow_mut() = Rc::clone(new_next);
                        // this should drop the existing ref_cell
                        Ok(())
                    }
                    Self::Nil => Err(anyhow::anyhow!("Can't update_next on Nil instance of List")),
                }
            }

            fn get_wrapped_list(value: i32, next: &Rc<Self>) -> Rc<Self> {
                Rc::new(Self::Cons(
                    RefCell::new(value),
                    RefCell::new(Rc::clone(next)),
                ))
            }

            fn get_wrapped_nil() -> Rc<List> {
                Rc::new(Self::Nil)
            }
        }

        impl Drop for List {
            fn drop(&mut self) {
                println!(
                    "Drop called for List with value {}",
                    if let Self::Cons(value, _) = self {
                        (*value).borrow().to_string()
                    } else {
                        String::from("Nil")
                    }
                );
            }
        }

        #[test]
        fn test() {
            let nil = List::get_wrapped_nil();

            let a = List::get_wrapped_list(1, &nil);
            let b = List::get_wrapped_list(2, &a);
            let c = List::get_wrapped_list(3, &a);

            let get_ref_counts = || {
                [
                    Rc::strong_count(&a),
                    Rc::strong_count(&b),
                    Rc::strong_count(&c),
                ]
            };

            // b -> a <- c
            // change it to b <- a <- c, i.e. b's next to nil, a's next to b
            assert_eq!(get_ref_counts(), [3, 1, 1]);
            (*b).update_next(&nil);
            assert_eq!(get_ref_counts(), [2, 1, 1]);
            (*a).update_next(&b);
            assert_eq!(get_ref_counts(), [2, 2, 1]);
            (*c).update_value(4);
        }
    }

    mod directed_tree_node_with_refcell_rc {
        use std::{cell::RefCell, rc::Rc};

        // TreeNode
        struct TreeNode<T> {
            value: RefCell<T>,
            children: RefCell<Vec<Rc<TreeNode<T>>>>,
        }

        impl<T> TreeNode<T> {
            fn new(value: T) -> Self {
                Self {
                    value: RefCell::new(value),
                    children: RefCell::new(vec![]),
                }
            }

            fn add_child(&self, child: &Rc<TreeNode<T>>) {
                (*self.children.borrow_mut()).push(Rc::clone(child));
            }

            fn change_value(&self, value: T) {
                *self.value.borrow_mut() = value;
            }
        }

        #[test]
        fn tree_nodes_have_right_strong_counts() {
            let vertices_rc = (1..=6)
                .map(|value| Rc::new(TreeNode::new(value)))
                .collect::<Vec<_>>();

            let edges = [(1, 2), (1, 5), (2, 3), (2, 4), (5, 6)];

            edges.iter().for_each(|(u, v)| -> () {
                match vertices_rc.get(u - 1) {
                    Some(rc_u) => match vertices_rc.get(v - 1) {
                        Some(rc_v) => {
                            rc_u.add_child(rc_v);
                        }
                        None => todo!(),
                    },
                    None => todo!(),
                }
            });

            let strong_counts = vertices_rc
                .iter()
                .map(|rc_u| Rc::strong_count(rc_u))
                .collect::<Vec<usize>>();

            assert_eq!(strong_counts, vec![1, 2, 2, 2, 2, 2]);
        }
    }

    mod weak_pointers {
        use std::{
            cell::RefCell,
            rc::{Rc, Weak},
        };

        // Creating ref cycles: Generally due nested use of interior mutability and reference counters
        //
        // == Preventing ref cycles using weak references (Weak<T>) ==
        // Rc::clone : share ownership using strong references
        // (strong_count needs to be 0 for drop)
        //
        // Rc::downgrade(&self) -> Weak<T> : doesn't express an ownership using weak references
        // (weak_count, doesn't need to be 0 for drop)
        //
        // but ofcourse, that means we need to check manually if value through weak reference is NOT
        // dropped. This can be done using rc::Weak::upgrade(&self<T>) -> Option<Rc<T>>.
        // If we are successful in upgrading the weak reference to a strong reference, strong count
        // also increases.
        //
        // NOTE: MUST see the below test: test_fundamentals for better understand
        #[test]
        fn test_fundamentals() {
            let r1 = Rc::new(0); // reference counted allocation for 0 on heap
            let r4 = {
                let r2 = Rc::clone(&r1); // r2 is clone, points to the same
                                         // underlying allocation as r1 does, but strongly referenced

                Rc::downgrade(&r2) /* returned value is a weak reference of the
                                    * allocation pointed by r2 (i.e. same as pointed by r1) */
            }; // r2 is dropped, hence reducing the strong count by 1

            assert_eq!([Rc::strong_count(&r1), Rc::weak_count(&r1)], [1, 1]);

            let r5 = Rc::clone(&r1); // r5 strong references the allocation pointed by r1

            assert_eq!([Rc::strong_count(&r1), Rc::weak_count(&r1)], [2, 1]);

            let r6 = r4.upgrade(); /* r6 tries to upgrade the weak reference r4 to Rc
                                    * and is successful since strong_count != 0, hence
                                    * increasing the strong count by 1 // Some(rc: Rc<T>) = r4.upgrade() */
            assert_eq!([Rc::strong_count(&r1), Rc::weak_count(&r1)], [3, 1]);
        }

        enum Parent<T> {
            Yes(T),
            No,
        }

        struct TreeNode<T> {
            pub value: RefCell<T>,
            /* node -> children should be strong reference, should reduce strong count of children if parent node drops */
            pub children: RefCell<Vec<Rc<TreeNode<T>>>>,
            /* node -> parent, should be a weak reference, even if node is dropped,
             * parent's strong count shouldn't change, instead just the weak count should */
            pub parent: RefCell<Parent<Weak<TreeNode<T>>>>,
        }

        impl<T> TreeNode<T> {
            pub fn new(value: T) -> Self {
                Self {
                    value: RefCell::new(value),
                    children: RefCell::new(vec![]),
                    parent: RefCell::new(Parent::No),
                }
            }

            fn add_child(&self, child: &Rc<TreeNode<T>>) {
                (*self.children.borrow_mut()).push(Rc::clone(child));
            }

            fn add_parent(&self, parent: &Rc<TreeNode<T>>) {
                *self.parent.borrow_mut() = Parent::Yes(Rc::downgrade(parent));
            }

            fn change_value(&self, value: T) {
                *self.value.borrow_mut() = value;
            }

            pub fn join(parent: &Rc<TreeNode<T>>, child: &Rc<TreeNode<T>>) {
                (*parent).add_child(child);
                (*child).add_parent(parent);
            }
        }

        impl<T: Copy> TreeNode<T> {
            pub fn get_values_till_root(&self) -> Vec<T> {
                let mut v = vec![*self.value.borrow()];

                if let Parent::Yes(ref p) = *self.parent.borrow() {
                    if let Some(rc_p) = &p.upgrade() {
                        v.append(&mut rc_p.get_values_till_root().clone());
                    }
                }
                v
            }
        }

        #[test]
        fn test_strong_and_weak_counters() {
            let vertices_rc = (1..=6)
                .map(|value| Rc::new(TreeNode::new(value)))
                .collect::<Vec<_>>();

            let edges = [(1, 2), (1, 5), (2, 3), (2, 4), (5, 6)];

            edges.iter().for_each(|(u, v)| -> () {
                match vertices_rc.get(u - 1) {
                    Some(rc_u) => match vertices_rc.get(v - 1) {
                        Some(rc_v) => {
                            TreeNode::join(rc_u, rc_v);
                        }
                        None => todo!(),
                    },
                    None => todo!(),
                }
            });

            let get_strong_and_weak_counts = || {
                vertices_rc
                    .iter()
                    .map(|rc_u| (Rc::strong_count(rc_u), Rc::weak_count(rc_u)))
                    .collect::<Vec<(usize, usize)>>()
            };

            assert_eq!(
                get_strong_and_weak_counts(),
                vec![(1, 2), (2, 2), (2, 0), (2, 0), (2, 1), (2, 0)]
            );

            {
                let rc_7 = Rc::new(TreeNode::new(7));
                match vertices_rc.get(5) {
                    Some(rc_6) => {
                        TreeNode::join(rc_6, &rc_7);
                    }
                    None => todo!(),
                }
                assert_eq!(rc_7.get_values_till_root(), vec![7, 6, 5, 1]);
            }
            assert_eq!(
                get_strong_and_weak_counts(),
                vec![(1, 2), (2, 2), (2, 0), (2, 0), (2, 1), (2, 1)]
            );
        }

        #[test]
        fn test_parent_dropped() {
            let leaf = Rc::new(TreeNode::new(2));
            {
                let root = Rc::new(TreeNode::new(1));
                leaf.add_parent(&root);
                root.add_child(&leaf);

                assert_eq!(Rc::weak_count(&root), 1);
                assert_eq!(Rc::strong_count(&root), 1);

                assert_eq!(Rc::strong_count(&leaf), 2);
                assert_eq!(Rc::weak_count(&leaf), 0);
                // node dropped here
            }

            assert_eq!(Rc::strong_count(&leaf), 1);
            assert_eq!(Rc::weak_count(&leaf), 0);
            assert_eq!(
                if let Parent::Yes(ref p) = *leaf.parent.borrow() {
                    p.upgrade().is_none()
                } else {
                    false
                },
                true
            );
            // since node was dropped, weak pointer leads to None.
            // NOTE: This doesn't lead to Parent::No (obviously!)
        }
    }

    mod other_smart_pointers {
        mod cell {
            // == Cell<T: Copy> == same family of RefCell,
            /* - Copy based interior mutability. You never get reference to the inner value:
             * --- get() -> T, T impl Copy, get returns copy of value
             * --- set(T), overwrites the old value
             * --- replace(T), overwrite the old value with new and returns the Copy of old value
             * --- take()
             *
             * - NEITHER Send NOR Sync - thread-unsafe
             * - Zero-runtime borrow checking (unlike RefCell)
             */

            use std::cell::Cell;

            #[test]
            fn basic_cell_usage() {
                struct X {
                    item: Cell<Option<i32>>,
                }

                let x = X {
                    // immutable ownership
                    item: Cell::new(Option::Some(4)),
                };

                let opt = x.item.get(); // copy of value inside x.item
                x.item.set(Option::None);

                assert_eq!(opt, Option::Some(4)); // Old value returned as Copy stays the same
                assert_eq!(x.item.get(), Option::None); // new value is written (copy of that value is checked in assert_eq)
            }
        }

        mod cow {
            use std::borrow::Cow;

            // == Cow<T> == Clone On Write
            /*
             * pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned {
             *      Borrowed(&'a B),
             *      Owned(<B as ToOwned>::Owned),
             * }
             *
             * - Borrowed(&'a B) -> Borrowed reference of B, which is cheap to copy and store
             * (bitwise copy of the pointer i.e. the memory address)
             *
             * - Owned(<B as ToOwned>::Owned) -> Clone of B is heap allocated
             *
             * How it helps? Assume you have a function whose return type is Cow<T>.
             * The consumer, can know (transparently) whether you returned a borrowed value of the
             * input or owned value of the input. It makes the consumption more explicit.
             */

            /* // NOTE: ?Sized (trait bound), means B can be Sized or NOT, that is Size of B can be known at compile time or NOT.
             *
             * Why is this needed? By default, rust assumes all generic types to be Sized.
             * But if your functions support unsized types as well,
             * then you must explicitly opt out of the default by T: ?Sized
             *
             * Known size at compile time helps in
             * -- placing them on stack,
             * -- allocating space,
             * -- knowing how to pass them by value (stack of other fn call)
             */

            /* // NOTE: Why is ToOwned needed?
             *
             * For all ?Sized Bs, we need ToOwned to be implemented (call .to_owned()) for cloning
             *
             * B can be ?Sized, like str, [T], dyn Trait, Path..
             * and the Owned type for them are String, Vec<T>, Box<dyn Trait>, PathBuf resp...
             */
            #[test]
            fn basic_cow() {
                // let x = Cow::new();
            }
        }

        mod unsafe_cell {}
    }
}
```

### Thoughts

**Big picture** (your summary on L232–240 is good to memorise; here's a slightly sharper version)

| Type | Owners | Mutation | Checked | Threads |
|------|--------|----------|---------|---------|
| `Box<T>` | 1 | through `&mut` | compile time | Send/Sync if `T` is |
| `Rc<T>` | many | none (shared only) | compile time | neither |
| `RefCell<T>` | 1 | through `&self` (`borrow_mut`) | **runtime** (panics) | Send if `T: Send`, not Sync |
| `Cell<T>` | 1 | through `&self` (get/set, never hands out a reference) | nothing to check | Send if `T: Send`, not Sync |
| `Rc<RefCell<T>>` | many | yes | runtime | neither |
| `Arc<Mutex<T>>` | many | yes (lock) | runtime | yes |
| `Weak<T>` | 0 (observer) | - | `upgrade()` → `Option` | like `Rc` |

- `Deref` powers `*x` and **deref coercion**: `&MyBox<String> → &String → &str`. The compiler inserts the `deref` calls, and they're usually inlined away.
- `Drop` is RAII: cleanup runs when the owner goes out of scope. `std::mem::drop(x)` is just a function that takes ownership and returns, which ends the value early.
- Reference cycles between `Rc`s leak memory. Point child→parent links at a `Weak`.

**Corrections**
- L17–18, the ToDo on trait objects: `Box<dyn Trait>` owns a value whose concrete type has been **erased**. Its size isn't known at compile time, so it has to sit behind a pointer. `Box` is the owning pointer that makes this possible. The `oops.rs` notes cover this in more detail.
- L32–35: your comment is right, and there's a subtle reason behind it. `&mut i32` isn't `Copy`, so passing `x` to `Box::new(x)` **moves** it. Passing a `&mut` to a function whose parameter type is explicitly `&mut T` would **reborrow** it instead, and `x` would still be usable. `Box::new` takes a generic `T`, so no reborrow happens. That matches the definition in your `CONTEXT.md`.
- L47: "still haven't figured out the heap allocation part". `MyBox` **doesn't allocate at all**. It's a plain wrapper, and the book says so. Only the `Deref` behaviour is being copied. A real `Box` gets heap memory from the allocator.
- L77–78: `*bs` is a `String` place (a location holding a `String`), not `&String`. `*bs` expands to `*Deref::deref(&bs)`, and `deref` returns `&String`, so the extra `*` gives you the `String` place. `[..]` then gives `str`, and `&` gives `&str`.
- L82: "all deref is compile time computation". What's decided at compile time is **which** `deref` calls to insert. They do run, but they're usually inlined to nothing.
- L122–128: the Copy+Drop rule is right, but the example can't happen: `MyTupleStruct(Box<T>)` could never be `Copy` anyway, because `Box` isn't `Copy`. The real reason is that a `Copy` value is duplicated *implicitly*, so a `Drop` type that was `Copy` would run `drop` once for each copy. The compiler forbids implementing `Copy` on any type that implements `Drop`.
- L199–200: "we should not do a manual `a.clone()` here, since `Rc::clone` will also handle the reference counting". **This is wrong.** `a.clone()` on an `Rc` calls **the same** `Rc::clone`: it increments the count and never deep-copies. Writing `Rc::clone(&a)` is a **readability convention**, and your next sentence gives the real reason: it makes cheap reference-count bumps easy to tell apart from expensive deep clones.
- L238: "RefCell<T> (**reference counted**...)". RefCell doesn't count owners. It tracks **active borrows** (how many `Ref`s, or whether a `RefMut` exists) at runtime. It has a single owner.
- L434: "this should drop the existing ref_cell". It drops the **old `Rc`** that was stored inside the `RefCell` (which lowers that node's strong count). The `RefCell` itself stays.
- L728 and L734: two fixes to the `Cell` notes:
  - `Cell<T>` works for **any** `T`. Only `get()` needs `T: Copy`. `set`, `replace`, `take` (which needs `T: Default`) and `into_inner` work for any `T`.
  - `Cell<T>` **is `Send`** (when `T: Send`). It's only `!Sync`, like `RefCell`. I checked this with a compile-time assertion.
- L795–796: `dyn Trait → Box<dyn Trait>` isn't a `ToOwned` pair in std. The real pairs are `str → String`, `[T] → Vec<T>`, `Path → PathBuf`, `OsStr → OsString`, `CStr → CString`, plus any `T: Clone → T`.

**Worth adding**
- A gotcha with `(*b).borrow_mut()` (L400). The explicit `*` is usually unnecessary, but it guards against a known trap. If `use std::borrow::BorrowMut;` is in scope, then `rc.borrow_mut()` resolves to the **trait** method on the `Rc` instead of `RefCell::borrow_mut`, and you get a confusing type error.
- `Parent<T>` (L592) is a hand-rolled `Option<T>`. The usual idiom is a plain `Weak<TreeNode<T>>`, starting as `Weak::new()`, which always upgrades to `None`. That's what the book does.
- L639: `.clone()` on a temporary `Vec` before `append` is wasted work. `v.extend(rc_p.get_values_till_root())` does the same thing.
- `Cow`, for the empty `basic_cow` test:

  ```rust
  fn normalize(s: &str) -> Cow<'_, str> {
      if s.contains(' ') { Cow::Owned(s.replace(' ', "_")) } else { Cow::Borrowed(s) }
  }
  ```

  It allocates only when something has to change. `cow.to_mut()` is the "clone on write" step: it clones the borrowed data the first time you mutate it.

---

## 9. `concurrency.rs` (Ch 16)

```rust
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
```

### Thoughts

**Big picture**
- There are three tools, in increasing order of sharing:
  - `thread::spawn` + `move`: hand data to a thread.
  - Channels (`mpsc`): transfer ownership of values between threads.
  - `Arc<Mutex<T>>`: shared mutable state.
- A channel **iterator ends only when every `Sender` is dropped.** If one `tx` stays alive anywhere, for example still in `main`, `rx.iter()` blocks forever. Your tests work because each sender is moved into a thread (or a `move` closure) and dropped when that thread ends. It's the most common channel bug.
- `Send` means ownership can move to another thread. `Sync` means `&T` can be shared between threads. The compiler implements both automatically for any type whose parts are `Send`/`Sync`.

**Corrections**
- L18–19: when `main` returns, the whole process exits and **kills** the spawned threads. Without `join`, the thread may simply **not finish**. For threads spawned from any function other than `main`, the lifetime concern is real, and that's why `spawn` needs `'static`.
- L41–43: two of the three bullets are off:
  - "`'static`, like `&str` **on heap**": string literals aren't on the heap (see `str_fuss`).
  - "variable holds values that implement `Copy`": being `Copy` doesn't make a value `'static` (`&'a T` is `Copy`!). A `Copy` value is simply *copied into* the `move` closure, and that copy is owned. Without `move`, even a `Copy` value is captured **by reference** and the code fails.

  The accurate rule: everything captured must satisfy `'static`, which means it's owned or `'static` data.
- L72: "channels cannot relay only a single type" is a double negative. You mean a channel carries **exactly one** type `T`, so use an enum for several kinds of message.
- L96–98, your ToDo on `tx.clone()` vs `Rc::clone(&rc)`: the `Rc::clone(&x)` style exists because `Rc<T>` derefs to `T`, so a reader can't tell whether `x.clone()` clones the pointer or the value. `Sender` doesn't deref to anything, so `tx.clone()` can't be misread. `Arc` follows the `Rc` convention.
- L118–121, your guess: the `Receiver` iterator uses **blocking `recv()`**, not `try_recv()`. `rx.try_iter()` is the non-blocking version. Also, `Receiver` doesn't implement `Iterator` itself. It implements `IntoIterator`, which is why `for x in rx` works.
- L197: `lock()` returns `Err(PoisonError)` if **another thread panicked while holding the lock**. That's what your `Err(_) => todo!()` would catch.
- L280–281: "T is Sync, if &T implements Sync". **This is the wrong definition.** The actual rule: **`T: Sync` if and only if `&T: Send`.** In words: a type is `Sync` exactly when it's safe to send a reference to it to another thread.
- L310–319: two fixes:
  - `Mutex<T>` is `Sync` when **`T: Send`**, not when `T: Sync`. The lock guarantees only one thread touches `T` at a time, so `T` only needs to be sendable.
  - **`MutexGuard` is `!Send` but it *is* `Sync`** (when `T: Sync`). I checked this with a compile-time assertion. The sentence that breaks off on L318 would go on: moving the guard to another thread would unlock the mutex from a thread that didn't lock it, and OS mutexes (such as pthreads) forbid that.

**Worth adding**
- `Arc<T>` is `Send`/`Sync` only if `T: Send + Sync`. So `Arc<RefCell<T>>` can't go to another thread, and that's exactly why you pair `Arc` with `Mutex`.
- `std::thread::scope(|s| { s.spawn(|| ...); })`: the threads may **borrow** local data, including a plain `&Mutex<i32>` without any `Arc`, because every scoped thread is joined before `scope` returns.

---

## 10. `async_docs.rs` (Ch 17)

```rust
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
```

### Thoughts

**Big picture**
- A `Future` is a **state machine** that the compiler builds from an `async` block or fn. It does nothing until it is **polled**, and awaiting it is what drives the polling. A **runtime** (tokio) keeps polling it and parks it when it returns `Pending`.
- **Tasks switch only at `.await` points.** Code between two awaits runs without interruption, so any blocking call (`thread::sleep`, heavy CPU work) stalls every other future on that thread. Your `yield_now` test shows exactly this.
- Dropping a future **cancels** it. When `select` or `race` returns, the losing future is dropped: it stops wherever it last paused and never resumes.
- `Stream` is the async version of `Iterator`: `poll_next` returns `Poll<Option<Item>>`.

**Corrections**
- L20: "future -> standard library for Future". `futures` is a **separate crate** (maintained by the rust-lang org). std has only the `Future` trait and a few basics. `join`, `select`, `Stream` and `StreamExt` come from `futures` or `tokio-stream`.
- L73 and L204: comments still refer to `run` / `helpers::run`, which were renamed to `tokio_rt_block_on`. Normally you'd write `#[tokio::main]` or `#[tokio::test]`, but those need tokio's `macros` feature, which your `Cargo.toml` doesn't enable. That's presumably why you wrote the helper.
- L83–84 and L471, your ToDos on pinning. `future::select` and `StreamExt::next` require **`Unpin`**, because they poll the same future or stream repeatedly through `&mut`. The futures that `async` blocks and fns produce are **`!Unpin`**, since they may point into themselves across an `.await`. `pin!(x)` fixes it: it gives you a `Pin<&mut X>`, and `Pin<&mut X>` **is** `Unpin`, so it satisfies the bound. The same goes for the merged stream, which contains tokio's `Sleep`, and `Sleep` is `!Unpin`.
- L108: more precisely, an `async fn` that takes references returns a future that **borrows** them, like `-> impl Future<Output = ...> + 'a` where `'a` is the lifetime of `url`. You can't spawn `page_title(&local)` as a `'static` task.
- L183 and L193: `try_lock().unwrap()` can **panic at random**. Your runtime has several threads, so the spawned task and the main block can try to take the lock at the same moment. Use `.lock().await`. Also, tokio's `Mutex` is only needed when you **hold the guard across an `.await`**. You deliberately don't (L185), so `std::sync::Mutex` would have been fine, and tokio's own docs recommend it in that case.
- L231: "fairness can't be achieved using join if this was 100ms". `join` is fair: it polls every future whenever it's woken. With different sleep times, the *output order* just follows the timing. That's a scheduling effect, not unfairness.
- L249–254: to finish your reasoning:
  - Every `async` block has its own unique type, so a `Vec` of them needs a trait object, `dyn Future`.
  - A trait object needs a pointer: `Box` or `&mut`.
  - `Pin<P>` implements `Future` whenever the pointer `P` points to a future. `Box<dyn Future>` implements `Future` only when that future is `Unpin`.

  So you need `Pin<Box<dyn Future>>` or, without any heap allocation, your `Pin<&mut dyn Future>`.
- L291–292: the comment has it backwards. The yields **are** in place, and that's what lets `fut_b` win. **Commenting them out** makes `fut_a` run all 4 seconds in its first poll and win, which triggers the panic.
- L536–538: "**Pin** is implemented by default for every type". You mean **`Unpin`**. `Pin` is a struct (a pointer wrapper), and `Unpin` is an **auto trait**, implemented for almost every type. The exceptions (`!Unpin`) are compiler-generated futures and types that contain `PhantomPinned`. Pinning an `Unpin` value does nothing: you can still move it.

**Worth adding**
- Blocking work inside async code belongs in `tokio::task::spawn_blocking`, not in `thread::sleep` inside a future.
- `tokio::time::timeout` is the built-in version of your `custom_timeout`.
- `futures::StreamExt` and `tokio_stream::StreamExt` are two different traits with overlapping method names. Import only one of them in a module.

---

## 11. `oops.rs` (Ch 18)

```rust
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
```

### Thoughts

**Big picture**
- Rust's version of OOP:
  - **Encapsulation**: privacy at the module level.
  - **No data inheritance**: build types by composition, and share behaviour through trait default methods.
  - **Polymorphism**: generics or `impl Trait` (static dispatch), or `dyn Trait` (dynamic dispatch).
- **Enum or trait object?** An enum is a **closed** set: adding a *variant* means editing every `match`, while adding an *operation* is easy. A `dyn Trait` is an **open** set: adding a *type* is easy, while adding a *method* means editing every impl. This is known as the expression problem.
- The State pattern example: `self: Box<Self>` lets a state **consume itself** and return the next state. `Option::take()` is how you move a value out of `&mut self`.

**Corrections**
- L22–26: an important split that the notes don't make. `impl Trait` means two different things depending on where it appears:
  - **In an argument**, it's an anonymous generic parameter, and the **caller** chooses the type.
  - **In a return type**, it's an opaque type, and the **callee** chooses **one** concrete type. So `if c { A } else { B }` with different types doesn't compile.
- L29: `dyn Tr` itself is unsized, but `&dyn Tr` and `Box<dyn Tr>` are ordinary sized fat pointers.
- L60–61, your ToDo: you can downcast through `Any`. `Box<dyn Any>` has `.downcast::<T>()`, and `&dyn Any` has `.downcast_ref::<T>()`. Both need `T: 'static`. Since Rust 1.86, **trait upcasting** is stable (`&dyn MyTrait → &dyn Any` when `Any` is a supertrait of `MyTrait`), which makes this much easier.
- L63–67, the object-safety rules. First, the Rust docs now call this "**dyn compatibility**". Three fixes to the rules:
  - "can't use Self ... **by value**": a method that takes `self` by value is allowed. I compiled a `dyn` trait with `fn consume(self)`. What's not allowed is `Self` in **other positions** (return type or arguments). You can exclude such a method from the vtable with `where Self: Sized`.
  - "should NOT be generic": this applies to **type** generics. Lifetime generics are fine. Generic methods marked `where Self: Sized` are fine too.
  - "Only `&self`, `&mut self`, `Box<Self>`": also `Rc<Self>`, `Arc<Self>` and `Pin<&mut Self>`. That last one is how `dyn Future` works (see `poll`).
- L159–160: adding a new state isn't limited to the start of the chain. The real cost is that the states are **coupled**. Inserting a state between `PendingReview` and `Published` means editing `PendingReview::approve`. The book points out this weakness itself.

**Worth adding**
- The alternative on L169–170 is the **typestate pattern**: `DraftPost`, `PendingReviewPost` and `Post` are separate types, and each transition consumes one and returns the next. Invalid transitions (like reading the content of a draft) become **compile errors**. The book presents it as the more idiomatic Rust approach.
- `std::mem::replace(&mut self.x, new)` and `std::mem::take(&mut self.x)` do the same job as `Option::take`, for any field.

---

## 12. `design_tradeoffs.rs` (Brown University edition)

```rust
#[cfg(test)]
mod design_tradeoffs {
    #[test]
    fn references() {
        /*
         * Context: You are designing a simple asset manager for a game engine.
         * Functionality: An API client will provide paths of assets to load, and gets back access to the loaded assets.
         * Assumptions: You may assume that loads happen synchronously and instantaneously.
         * Designs: Below are several proposed designs to implement the functionality.

        pub struct AssetManager {
            // ...
        }

        // Option 1: return a reference
        impl AssetManager {
        pub fn load(&mut self, path: PathBuf) -> &Asset;
        }

        // Option 2: return a mutable reference
        impl AssetManager {
        pub fn load(&mut self, path: PathBuf) -> &mut Asset;
        }

        // Option 3: return a reference-counted pointer
        impl AssetManager {
        pub fn load(&mut self, path: PathBuf) -> Rc<Asset>;
        }

        // Option 4: return a copyable handle that can be turned into a reference
        #[derive(Copy, Clone)]
        pub struct AssetHandle(usize);

        impl AssetManager {
        pub fn load(&mut self, path: PathBuf) -> AssetHandle;
        pub fn get(&self, handle: AssetHandle) -> &Asset;
        }
        */

        /*
        == Requirement == Once loaded, an asset should be permanently immutable.
        == Solution == Option 1, 3, 4 - They return immutable borrow of the asset. Rc too is a
        multi-borrowed immutable ownership of the underlying value.
        */

        /*
        == Requirement == Clients of the asset manager need to retain access to assets
        across several short-term borrows of the asset manager.
        == Solution == Here, "across several short-term borrows of the asset manager" means
        asset manager is going to get allocated and deallocated often, and still we should be
        able to use loaded asset. In Option 1 & 2, the lifetime of the &Asset / &mut Asset is
        tied to the lifetime of &mut self of AssetManager, hence they aren't valid.
        Option 3 returns a Rc<Asset> which is a smart pointer, and lifetime is NOT tied.
        Doubt: in Option 4
        */

        /*
        == Requirement == It is important that all assets be deallocated at a single, predictable time.
        == Solution == Option 1, 2, 4. Here the lifetimes of the all the assets are tied with
        the AssetManager, so all gets deallocated when AssetManager is dropped.
        In Option 3, since the Asset is captured via Rc, AssetManager cannot guarantee of all
        the live strong references of the returned smart pointer.
        */
    }

    #[test]
    fn trait_trees() {
        /*
         * Context: You are designing a simple user interface framework that consists of a tree of widgets, such as text and buttons.
         * Functionality: The API provides a Widget trait that defines how a widget works.
         *                The API client implements the Widget trait, and calls it to render a UI.
         * Designs: Below are several proposed designs to implement the functionality.

        // Option 1: children must be Self
        pub trait Widget: Sized {
        fn render(&self) -> Vec<Self>;
        }

        // Option 2: children are a trait parameter
        pub trait Widget<Children> {
        fn render(&self) -> Vec<Children>;
        }

        // Option 3: children are an associated type
        pub trait Widget {
        type Children: Widget;
        fn render(&self) -> Vec<Self::Children>;
        }

        // Option 4: children are a reference trait object
        pub trait Widget {
        fn render(&self) -> Vec<&dyn Widget>;
        }

        // Option 5: children are a boxed trait object
        pub trait Widget {
        fn render(&self) -> Vec<Box<dyn Widget>>;
        }
        */

        /*
        == Requirement == The API client is expected to provide a single WidgetImpl enum representing all
        possible widgets, and implement the Widget trait for WidgetImpl.
        == Solution == Option 1 is correct since we have only one WidgetImpl enum for all widget types,
        Option 2 is unnecessary because generics aren't useful anymore with single enum capturing all,
        Option 3 again similar to above, Option 4 and 5 No need of dynamic dispatches
        */

        /*
         == Requirement == The API client is expected to provide a unique struct for each possible widget,
         and implement the Widget trait for each struct. Each widget can return a vector containing widgets of any possible type.
        == Solution == Option 4 and 5 can capture trait objects which will allow each possible widget type as part of children vec.
        Option 1, 2 and 3, doesn't allow heterogenous children container.
        */

        /*
        == Requirement == Only types that implement the Widget trait should be returned from render.
        == Solution == Option 1, 3, 4, 5 are correct because they all ensure the return type is Vec contained
        type which implements Widget, but in Option 2, we can have any Children without any bounds on it.
        */
    }

    #[test]
    fn dispatch() {
        /*
        Context: You are designing a simple event system that calls callback functions in response to events.
        Functionality: An Events struct provides functions to register callbacks. A callback is either parallelizable
                       (runnable across multiple threads) or sequential (must be run only on the main thread).
        Designs: Below are several proposed designs to implement the functionality.

           pub struct Events {
               // ...
           }

           // Option 1: parallel and sequential are two separate methods
           impl Events {
               pub fn register<E, F: Fn(E)>(&mut self, f: F) { /* .. */ }
               pub fn register_sequential<E, F: Fn(E)>(&mut self, f: F) { /* .. */ }
           }

           // Option 2: parallel and sequential are two members of an enum
           pub enum Callback<F> {
               Parallel(F),
               Sequential(F)
           }
           impl Events {
               pub fn register<E, F: Fn(E)>(&mut self, f: Callback<F>) { /* .. */ }
           }

           // Option 3: parallel and sequential are markers in a trait method
           pub trait Register<Marker, F, E> {
               fn register(&mut self, f: F);
           }
           pub struct Parallel;
           pub struct Sequential;
           impl<F, E> Register<Parallel, F, E> for Events
           where F: Fn(E) {
               fn register(&mut self, f: F) { /* .. */ }
           }
           impl<F, E> Register<Sequential, F, E> for Events
           where F: Fn(Sequential, E) {
               fn register(&mut self, f: F) { /* .. */ }
           }
        */

        /*
        == Requirement == A callback should be considered parallelizable by default, and the API should reflect that default behavior.
        == Solution ==
        In Option 1:
        events.register(|e: OnClick| { /* .. */ })
        events.register_sequential(|e: OnClick| { /* .. */ })

        In Option 2:
        events.register(Callback::Parallel(|e: OnClick| { /*..*/ }))
        events.register(Callback::Sequential(|e: OnClick| { /*..*/ }))

        In Option 3:
        events.register(|e: OnClick| { /* .. */ })
        events.register(_: Sequential, |e: OnClick| { /* .. */ })

        As, it can be seen, both in Option 1 and Option 3, we need to explicitly use `_sequential` suffix or
        (_: Sequential) Marker for sequential callbacks.
        In Option 2, both parallel and sequential are treated equally
        */

        /*
        == Requirement == The API should export as few methods as possible.
        == Solution ==
        Option 2, and Option 3 exports only a single method.
        */

        /*
        == Requirement == The API should lean on the compiler's type inference system as little as possible.
        == Solution == Option 3 heavily relies on the overloaded `register` method
        */
    }

    #[test]
    fn intermediates() {
        /*
         Context: You are designing a serialization library that converts Rust data types into formats like JSON.
         Functionality: A Serialize trait that can be implemented by serializable types, and a to_json function that converts serializable types into JSON.
         Designs: Below are several proposed designs to implement the functionality.

            // Option 1: types serialize to a `Value` enum
            pub enum Value {
                String(String),
                Number(isize)
            }

            pub trait Serialize {
                fn serialize(&self) -> Value;
            }

            fn value_to_json(value: Value) -> String {
                /* .. */
            }

            pub fn to_json(data: impl Serialize) -> String {
                let value = data.serialize();
                value_to_json(value)
            }


            // Option 2: types serialize via calls to a `Serializer` interface
            pub trait Serialize {
                fn serialize<S: Serializer>(&self, serializer: &mut S);
            }

            pub trait Serializer {
                fn serialize_string(&mut self, s: &str);
                fn serialize_number(&mut self, n: isize);
            }

            struct JsonSerializer { buffer: String };
            impl Serializer for JsonSerializer {
                /* .. */
            }

            pub fn to_json(data: impl Serialize) -> String {
                let mut serializer = JsonSerializer { buffer: String::new() };
                data.serialize(&mut serializer);
                serializer.buffer
            }
        */

        /*
        == Requirement == It should be possible to add a new data format without needing to modify code in existing implementations
                          of the Serialize trait.
        == Solution == For both Option 1 and Option 2, we need to implement Serialize for any new data format
        */

        /*
        == Requirement == The process of serialization should consume as little memory as possible.
        == Solution == For Option 1, data is serialized to intermediate value, and then to a json string
                       whereas, in Option 2, data is serialized directly in the buffered string
         */

        /*
        == Requirement == When the same data type is serialized into multiple formats, that should increase the size
                          of the compiled binary as little as possible.
        == Solution == It is important to understand that Option 2 depends on compile time polymorphism using generics,
                       which will create multiple copies of the code, replacing each generic with a type at compilation
                       (which is called Monomorphization -- polymorphic to monomorphic function conversion).
                       `to_json` has two generics, `Serialize` and `Serializer` whereas in Option 1, there's only one generic, `Serialize`.
        */
    }
}
```

### Thoughts

**Big picture**
- API design in Rust mostly comes down to three questions:
  - **Who owns the data?** Borrowing, `Rc`, or a handle.
  - **Is the set of types open or closed?** Enums, generics, or trait objects.
  - **Where does the cost go?** Monomorphization means bigger binaries. Intermediate values mean more memory. Dynamic dispatch means indirect calls.

**Corrections**
- L47–55, the second `references` requirement. **You misread it.** "Across several short-term borrows of the asset manager" doesn't mean the manager is created and destroyed often. It means the client calls `manager.load(...)` several times. Each call is a short `&mut` borrow of the manager, and the client wants to **keep the assets it got earlier** while calling `load` again.
  - With Options 1 and 2, the returned `&Asset` keeps `&mut self` borrowed, so the second `load` doesn't compile.
  - Option 3 (`Rc`) holds no borrow of the manager.
  - For your "Doubt: in Option 4": it **also works**. `AssetHandle` is a `Copy` index that holds no borrow. You call `get(handle)` for a short `&Asset` whenever you need one.

  Answer: **Options 3 and 4.**
- L167–184, the first `dispatch` requirement. The conclusion is never stated. Parallel is the default in **Options 1 and 3**: the plain `register` means parallel, and sequential needs extra effort. Option 2 treats both the same, so it **fails** the requirement. Also, L179 isn't valid syntax. In Option 3, the marker is a **closure parameter**: `events.register(|_: Sequential, e: OnClick| { ... })`. Type inference then picks the `Sequential` impl, which is exactly why the last requirement penalizes Option 3.
- L248–251, the first `intermediates` requirement. The answer (both options satisfy it) is right, but **the reason is backwards**. In both options, `Serialize` impls **don't know about any format**:
  - In Option 1, types produce a `Value`, and you'd just write a new `value_to_xml`.
  - In Option 2, you'd write a new `XmlSerializer: Serializer`.

  In neither case do existing `Serialize` impls change, and that's the point of the requirement.
- L262–265: the binary growth in Option 2 comes from `fn serialize<S: Serializer>`. Every data type gets one copy **per format**, so copies = types × formats. In Option 1, `serialize` isn't generic, so every data type has one copy no matter how many formats there are.

---

## 13. `advanced_features.rs` (Ch 20)

```rust
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
```

### Thoughts

**Big picture**
- `unsafe` doesn't switch anything off. It unlocks **five extra operations**, and **you** become responsible for keeping the rules the compiler can no longer check. The borrow checker keeps running on references as usual.
- **Associated type or generic parameter?** A generic parameter is an **input**: one type can implement `Trait<A>`, `Trait<B>` and so on. An associated type is an **output**: exactly one choice per implementing type.
- **Newtype** (`struct Km(i32)`) gives you a new, distinct type, and it's how you get around the **orphan rule**: you can only implement a trait for a type if the trait or the type belongs to your crate. A **type alias** (`type Km = i32`) is only another name for the same type.

**Corrections**
- L58–59: "This gives seg fault". Dereferencing an invalid pointer is **undefined behaviour**. A segfault is one possible outcome, and silently reading garbage is another. You can't rely on it failing loudly.
- L69–70: "You cannot borrow num as mutable, so rust borrow-checker is still working". The error really happens because **`num` isn't declared `mut`**, so `&raw mut num` is rejected by the mutability check. The claim "the borrow checker still works in unsafe" is true, but this example shows the mutability rule, not aliasing.
- L108–113: the real downside of a generic `Iterator<T>` is that `X` could implement it **several times**. At a call like `x.next()`, the compiler may not know which impl you mean, so it needs a type annotation. You don't have to annotate *every* call. Also, `impl Iterator<T> for X` would need `impl<T>` or a concrete type to compile.
- L183–184: the sentence is garbled. The point is that `Deref` on a newtype **exposes the inner type's whole API** and undoes the "distinct type" guarantee you wanted. Many people consider `Deref` on non-pointer newtypes an anti-pattern.

**Worth adding** (Ch 20 topics these notes don't cover)
- **Fully qualified syntax**, `<Type as Trait>::method(&x)`. You need it when two traits have methods with the same name.
- **Supertraits**, as in `trait Pretty: Display { ... }`.
- The **never type `!`**. You've already seen it in `continue` inside a `match`, back in the guessing game.
- **Unsafe**: `static mut`, which edition 2024 warns about on any reference to it (the `static_mut_refs` lint), `unsafe impl Send`, and FFI with `extern "C"`.
- In edition 2024, unsafe operations inside an `unsafe fn` need their own `unsafe {}` block. You already did this on L68.

---

## 14. `macros.rs` (Ch 20.5 + Appendix C)

```rust
#[cfg(test)]
mod macros {
    // === Macros ==
    // - Declarative
    //   Match-like macro pattern syntax
    //
    // - Procedural
    //   simple theoritically, flow is:
    //   use the TokenStream -> AST -> TokenStream, and all the functionalities of `syn` and `quote`.
    //
    // --- custom (derive)
    // --- attribute-like
    // --- function-like

    // == Derivable Traits from std library ==  Procedural Macros
    // - NOTE: Display trait cannot be derived, must be implemented for "{:?}" formating
    //
    // == Debug trait == use-cases in assert_eq, general debugging, converting your DS to a string
    //   for doing logging, comparisions etc.
    //
    // == PartialEq trait ==  used for == and != with itself. -> should be preferred for assert_eq if implemented
    //
    // == Eq trait ==  no methods, purpose is for saying all instances of a structs/enums implementing
    //   this trait should be treated as equals
    //
    // == PartialOrd trait ==  partial_cmp returns Option<Ordering> <, >, <=, >=.
    //   (if a valid ordering may or may NOT be present, eg. floats and NaN)
    //
    // == Ord ==  cmp return Ordering. If Valid ordering between two values of the type always exist.
    //
    // == Clone trait ==  deep copy of the value (not bit-wise copy)
    //   all fields inside must derive Clone trait as well
    //
    // == Copy trait == bit wise copy, if Copy implemented, it's used as a default Clone too
    //
    // IMPORTANT NOTE: Rust doesn't allow us to implement both Copy and Drop trait for a type at
    // the same time? Why? Because since Copy can lead to bitwise copy of pointers which were
    // meant to be freed(once) using the Drop trait. Copying can lead to freeing the same memory
    // twice.
    //
    // == Hash trait == hash method, all fields must derive Hash trait too
    //    Usages like in HashMap<K, V>, K must implement Hash trait
    //
    // == Default trait == ::default() method, all fields must implement Default trait
    //   usages as simple as ::default() or in Option::<T>::unwrap_or_default(..) where T : Default
}
```

### Thoughts

**Big picture**
- **Declarative macros** (`macro_rules!`) match on **token patterns** and expand to code.
- **Procedural macros** are Rust functions of type `TokenStream → TokenStream`. They must live in a separate crate with `proc-macro = true`, and usually use `syn` to parse and `quote` to generate code. There are three kinds: custom `#[derive]`, attribute-like (`#[route(...)]`) and function-like (`sql!(...)`).
- `#[derive(X)]` writes the obvious impl for you, **field by field**. That's why every field must implement `X`.

**Corrections**
- L15: the std derives (`Debug`, `Clone`, ...) are **built into the compiler**, not proc-macro crates. They behave like derive macros from your side, though.
- L16: "Display trait cannot be derived, must be implemented for **`{:?}`** formatting". The format strings are swapped: **`{}` is `Display`** (not derivable) and **`{:?}` is `Debug`** (derivable).
- L18–19: Debug output isn't guaranteed to stay the same between versions, so don't use it for comparisons or stored formats.
- L21: `assert_eq!` **requires** both `PartialEq` (to compare) and `Debug` (to print the values when it fails). `PartialEq<Rhs>` can also compare with **other** types, for example `String == &str`.
- L23–24: "Eq ... all instances ... should be treated as equals". **This is wrong.** `Eq` is a marker saying that `==` is a **full equivalence relation**. The extra guarantee over `PartialEq` is **reflexivity**: `a == a` is always true. `f32` and `f64` are only `PartialEq` because `NaN != NaN`. `HashMap` keys need `Eq + Hash`.
- L31–32: "Clone: deep copy". `Clone` means **an explicit duplicate**, with whatever meaning the type gives it. For `Rc`, `clone` only increments a counter (see `smart_pointers`). Fields need to *implement* `Clone`, not necessarily *derive* it. Also, `#[derive(Clone)]` on `Foo<T>` adds a `T: Clone` bound.
- L34: "if Copy implemented, it's used as a default Clone too". It's the other way around. **`Copy` requires `Clone`** (`Clone` is a supertrait of `Copy`), so you must write `#[derive(Copy, Clone)]` together. Copy has no methods; it only changes what `let b = a` does, from a move to a copy.

**Worth adding**
- Derived `PartialOrd` and `Ord` compare **fields in declaration order** (lexicographically), and for enums, **earlier variants are smaller**. Field order in the struct matters.
- `Ord` requires `Eq + PartialOrd`. You need it for `sort()`, `BTreeMap` keys and `max()`.
- `Hash` has a rule you must keep: `a == b ⇒ hash(a) == hash(b)`. If you implement `PartialEq` by hand, implement `Hash` by hand too.
- `Default` also lets you write `Foo { x: 1, ..Default::default() }`.
