pub fn try_outs_1() {
    let a = [1, 2, 3]; // fixed size array, on stack
    let v1 = vec![1, 2, 3]; // a vec on heap initialized with value
    let mut v2: Vec<u32> = Vec::new(); // a vec on heap with no initial value and hence type
                                       // annotation

    v2.push(3); // need to make v2 mutable since push(&mut self), borrows vec mutably
    let el0 = &v2[0];
    println!("{el0}");
    // let el1 = &v2[1]; // panics // run-time error
    // let el1 = v2.get(1).expect("Why is it not present"); // controlled panik

    let v_iter = v1.iter(); // borrows v1 immutably
    let n1 = v1.iter().next().unwrap(); // n1 also borrows v1 immutably

    /*
    let n3 = vec![1, 2].iter().next().unwrap();
    // NOTE that vector is dropped here but
    // n3 tried to hold on to a reference to it and borrows later

    println!("{n3}");
     */

    /*
     // Same issue what's happening above ^
    let v3 = vec![1, 2, 3];
    let n2 = v3.iter().next().unwrap(); // n2 also borrows v1 immutably
    drop(v3);

    println!("{n2}");
     */

    let v4 = vec![String::from("Hello"), String::from("World")];
    let mut range_iter = 0..v4.len(); // Iterator for a range vec![0, 1], yes which sits
                                      // on the heap!
    let first_index = range_iter.next().unwrap();
    // let mut first_elem = v4[first_index];
    /* cannot move out of index since String doesn't implement Copy trait */
    let first_elem = &v4[first_index];
    println!("{:?}", v4);

    println!("{first_elem}");
}

pub fn try_outs_2() {
    let mut s_vec: Vec<String> = Vec::new();
    s_vec.push(String::from("Hello world"));
    s_vec.push(String::from("How are you doing!"));

    /*
    for x in s_vec {
        println!("{x}");
    }
    println!("{:?}", s_vec.get(0).unwrap()); // can't borrow first element through get since
                                             // already moved
     */

    /*
    for x in &s_vec {
        println!("{x}");
        s_vec.push(String::from("This isn't good!")); // can't borrow s_vec as mut ref because x
                                                      // already borrows s_vec
    }
    println!("{:?}", s_vec.get(0).unwrap());
     */
    for x in &mut s_vec {
        println!("{x}");
        *x = String::from("This look fine");
    }
    println!("{:?}", s_vec);
}
