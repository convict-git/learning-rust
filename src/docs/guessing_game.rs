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
