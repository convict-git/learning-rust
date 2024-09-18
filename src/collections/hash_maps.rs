use std::collections::HashMap;

pub fn basics() {
    let mut h_map: HashMap<String, i32> = HashMap::new();
    h_map.insert("convict".to_string(), 26);
    h_map.insert("wildcat".to_string(), 20);

    let v = h_map.remove(&"Hello".to_string()).unwrap_or(-1);
    println!("{v}");

    let s_key = String::from("this");
    let s_value = 34;
    h_map.insert(s_key, s_value);

    // println!("{s_key}");  // ERROR: s_key moved but s_value
    println!("{s_value}"); // but s_value was copied
}

pub fn ref_lifetime_vague() -> () /* HashMap<&String, &String> */ {
    let mut h_map: HashMap<&String, &String> = HashMap::new();
    let s_key_1 = String::from("hello");
    let s_value_1 = String::from("world");

    h_map.insert(&s_key_1, &s_value_1);

    let s_key_2 = String::from("you");
    let s_value_2 = String::from("cruel");

    h_map.insert(&s_key_2, &s_value_2);

    println!("{s_key_1}: {s_value_1}");
    println!("{s_key_2}: {s_value_2}");
    println!("{h_map:?}");

    // return h_map; // ERROR: it will complaint h_map references local variable s_key_1, etc...
}

pub fn update_value() {
    let mut h_map = HashMap::new();
    let input = String::from("Hello from the other side! Hello, how have you been.");

    input
        .split([' ', ','])
        // .filter(|x| -> !x.is_empty())
        .for_each(|input_slice| {
            let count = h_map.entry(input_slice).or_insert(0);
            *count += 1;
        });

    println!("{h_map:?}");
}
