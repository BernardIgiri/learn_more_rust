use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod greetings;

fn main() {
    let a: [bool; 5] = [true, false, true, false, false];
    let s = String::from("Hello everybody!");
    let hello = &s[0..5];
    let everybody = &s[6..15];
    let s2 = &s;
    println!("{} {} {} {}", hello, everybody, s2, a[2]);
    for n in (1..10).rev() {
        println!("{}", n);
    }
    println!("Let's go!");
    let secret = rand::thread_rng().gen_range(1, 101);
    println!("Guess a number from 1 to 100.");
    let mut done = false;
    while !done {
        let mut input = String::new();
        println!("Enter your guess:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let (prompt, done_response) = match guess.cmp(&secret) {
            Ordering::Less => ("less than", false),
            Ordering::Greater => ("greater than", false),
            Ordering::Equal => ("equal to", true),
        };
        println!("Your guess {} was {} the secret.", guess, prompt);
        done = done_response;
    }
    println!("{} You won!", greetings::english::greet());
}
