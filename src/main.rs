extern crate unicode_segmentation;

mod greetings;
mod math;
mod vehicles;

use glam::Vec2;
use greetings::english::greet;
use hashbrown::HashMap;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let a: [bool; 5] = [true, false, true, false, false];
    let s = String::from("Hello everybody!");
    let hello = &s[0..5];
    let everybody = &s[6..15];
    let s2 = &s;
    println!("{} {} {} {}", hello, everybody, s2, a[2]);
    for n in (0..10).rev() {
        let fib_description = match math::fib_perfect_inverse(n) {
            Some(_) => "perfect inverse",
            None => "inverse",
        };
        println!(
            "{} has a fib of {} and {} fib of {}",
            n,
            math::fib(n).unwrap_or(0),
            fib_description,
            math::fib_closest_inverse(n),
        );
    }
    println!("Let's go!");
    let score = guessing_game();
    let mut car = get_prize(score);
    println!(
        "{} You won! Your prize is a brand new {}",
        greet(),
        car.name()
    );
    if confirm("Go for a drive?") {
        test_drive(&mut car);
    }
    let story = prompt("What will you do with your winnings?");
    let words = word_freq(&story);
    let letters = letter_freq(&story);
    let lorem_ipsum_japanese = letter_freq("旅ロ京青利セムレ弱改フヨス波府かばぼ意送でぼ調掲察たス日西重ケアナ住橋ユムミク順待ふかんぼ人奨貯鏡すびそ。");
    let lorem_ipsum_russian = letter_freq("Лорем ипсум долор сит амет, пер цлита поссит ех, ат мунере фабулас петентиум сит. Иус цу цибо саперет сцрипсерит,");
    println!("Word frequency\n{:#?}", words);
    println!("Letter frequency\n{:#?}", letters);
    println!("Lorem Ipsum Japanese\n{:#?}", lorem_ipsum_japanese);
    println!("Lorem Ipsum Russian\n{:#?}", lorem_ipsum_russian);
}

fn get_prize(score: u32) -> vehicles::cars::Car {
    match score {
        0 => vehicles::cars::Car::new_ferrari(),
        1 => vehicles::cars::Car::new_mercedes(),
        2 => vehicles::cars::Car::new_mustang(),
        3 => vehicles::cars::Car::new_sedan(),
        4 => vehicles::cars::Car::new_pickup_truck(),
        _ => vehicles::cars::Car::new_go_kart(),
    }
}

fn word_freq<S: Into<String>>(text: S) -> HashMap<String, u32> {
    let mut freq = HashMap::new();
    let s = text.into();
    let words = s.unicode_words().collect::<Vec<&str>>();
    for w in words {
        let count = freq.entry(w.to_lowercase()).or_insert(0);
        *count += 1;
    }
    freq
}

fn letter_freq<S: Into<String>>(text: S) -> HashMap<String, u32> {
    let mut freq = HashMap::new();
    let s = text.into();
    let graphemes = UnicodeSegmentation::graphemes(s.as_str(), true).collect::<Vec<&str>>();
    for l in graphemes {
        let count = freq.entry(l.to_lowercase()).or_insert(0);
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

fn guessing_game() -> u32 {
    let mut guess_count = 0;
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
        guess_count += 1;
    }
    math::fib_closest_inverse(guess_count)
}

fn confirm<S: Into<String> + std::fmt::Display>(text: S) -> bool {
    let mut input = String::new();
    println!("{} (y/n)", text);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input.trim().eq_ignore_ascii_case("y")
}

fn prompt<S: Into<String> + std::fmt::Display>(text: S) -> String {
    let mut input = String::new();
    println!("{}", text);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input.trim().to_string()
}
