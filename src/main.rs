mod greetings;
mod vehicles;

use glam::Vec2;
use greetings::english::greet;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

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
    guessing_game();
    let mut car = vehicles::cars::Car::new_ferrari();
    println!("{} You won! Your prize is a {:?}", greet(), car);
    if confirm("Go for a drive?".to_string()) {
        test_drive(&mut car);
        println!("{:?}", car);
    }
    let story = prompt("What will you do with the rest of your winnings?".to_string());
    let freq = word_freq(story);
    println!("Word frequency\n{:?}", freq);
}

fn word_freq(text: String) -> HashMap<String, u32> {
    let words = text.split_whitespace();
    let mut freq = HashMap::new();
    for w in words {
        let count = freq.entry(w.to_lowercase()).or_insert(0);
        *count += 1;
    }
    freq
}

fn test_drive(car: &mut vehicles::cars::Car) {
    println!("Vrrm!");
    car.set_state(vehicles::cars::State::Driving);
    for n in 1..100 {
        car.animate(0.23);
        println!(
            "Frame {} velocity {:?} heading {:?}",
            n,
            car.velocity(),
            car.heading()
        )
    }
    println!("Skrr!");
    car.set_state(vehicles::cars::State::Idle);
    for n in 100..200 {
        car.animate(0.23);
        println!(
            "Frame {} velocity {:?} heading {:?}",
            n,
            car.velocity(),
            car.heading()
        )
    }
    println!("Donuts! Brap! Brap!");
    car.set_state(vehicles::cars::State::Driving);
    for n in 200..220 {
        car.animate(0.23);
        println!(
            "Frame {} velocity {:?} heading {:?}",
            n,
            car.velocity(),
            car.heading()
        )
    }
    car.set_state(vehicles::cars::State::Turning(Vec2::new(0.0, 1.0)));
    for n in 220..300 {
        car.animate(0.23);
        println!(
            "Frame {} velocity {:?} heading {:?}",
            n,
            car.velocity(),
            car.heading()
        )
    }
    println!("Stop! Skrr!");
    car.set_state(vehicles::cars::State::Parked);
    for n in 300..330 {
        car.animate(0.23);
        println!(
            "Frame {} velocity {:?} heading {:?}",
            n,
            car.velocity(),
            car.heading()
        )
    }
}

fn guessing_game() {
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
}

fn confirm(text: String) -> bool {
    let mut input = String::new();
    println!("{} (y/n)", text);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input.trim().eq_ignore_ascii_case("y")
}

fn prompt(text: String) -> String {
    let mut input = String::new();
    println!("{}", text);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input
}
