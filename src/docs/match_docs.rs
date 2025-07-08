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
