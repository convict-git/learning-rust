pub fn push_down_reference() {
    let opt1 = Option::Some(String::from("Hello world")); // Here s is moved when pattern matching
                                                          // type of Scrutinee expression opt1 is Option<String>
    match opt1 {
        // NOTE: if we use a wildcard, values aren't moved/copied/borrowed
        // Some(_) => println!("Got something"),
        Some(s) => println!("{}", s), // value on the heap is moved to s out of the opt1
        None => println!("Something else"),
    }

    // Hence this is invalid, because we are trying to borrow opt1 which already has a partial moved value since String doesn't implement copy trait
    // println!("{:?}", opt1);

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

    // Some nice example of pattern matching and variable @ subpattern binding
    let arr = [1, 2, 3, 4];
    match arr {
        whole @ [head, tail @ .., last] => println!("{} {:?} {:?} {}", head, tail, whole, last),
    };
}
