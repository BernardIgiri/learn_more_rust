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
    for n in (0..11).rev() {
        let inverse = match math::fib_inverse(n) {
            Some(v) => " and an inverse fib of ".to_string() + &v.to_string(),
            None => "".to_string(),
        };
        println!(
            "{} has a fib of {}{}",
            n,
            math::fib(n).unwrap_or(0),
            inverse,
        );
    }
    println!("Let's go!");
    let score = guessing_game(1, 50);
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

fn guessing_game(min: u32, max: u32) -> u32 {
    let mut guess_count = 0;
    let secret = rand::thread_rng().gen_range(min, max + 1);
    println!("Guess a number from {} to {}.", min, max);
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
    calc_score(guess_count)
}

fn calc_score(guess_count: u32) -> u32 {
    let r = math::fib_inverse_rounded_up(guess_count - 1).unwrap_or(0);
    return if r > 2 { r - 1 } else { r };
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calc_score_uses_fibonacci() {
        let guesses_to_score_map: [u32; 21] = [
            0, 1, 2, 3, 4, 4, 5, 5, 5, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7,
        ];
        //                             1  1  1  1  1  1  1  1  1  1  2  2
        //  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5  6  7  8  9  0  1
        let mut guess_count = 1;
        for expected_score in guesses_to_score_map.iter() {
            let actual_score = calc_score(guess_count);
            assert_eq!(
                *expected_score, actual_score,
                "guess_count = {}",
                guess_count
            );
            guess_count += 1;
        }
    }
}
