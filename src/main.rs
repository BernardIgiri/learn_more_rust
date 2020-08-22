extern crate unicode_segmentation;

mod data_types;
mod greetings;
mod math;
mod vehicles;

use byteorder::{BigEndian, ReadBytesExt};
use glam::Vec2;
use greetings::english::greet;
use hashbrown::HashMap;
use rand::Rng;
use std::cmp;
use std::cmp::Ordering;
use std::io;
use std::{thread, time};
use unescape::unescape;
use unicode_segmentation::UnicodeSegmentation;

const DATA_SOURCE_URL: &str =
    "https://raw.githubusercontent.com/BernardIgiri/learn_more_rust/master/data/book.txt";

fn main() {
    {
        let buffer = [0u8, 10u8, 1u8, 1u8];
        let mut reader = io::Cursor::new(buffer);
        let num_float = reader.read_f32::<BigEndian>().unwrap_or(0.0);
        reader.set_position(0);
        let num_int = reader.read_i32::<BigEndian>().unwrap_or(0);
        println!("float {} int {}", num_float, num_int);
    }
    {
        let a: [bool; 5] = [true, false, true, false, false];
        let s = String::from("Hello everybody!");
        let hello = &s[0..5];
        let everybody = &s[6..15];
        let s2 = &s;
        println!("{} {} {} {}", hello, everybody, s2, a[2]);
    }
    for n in (0..11).rev() {
        let inverse = match math::fib_inverse(n) {
            Some(v) => format!(" and an inverse fib of {}.", v),
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
    println!("Letter frequency\n{:?}", letters);
    println!("Lorem Ipsum Japanese\n{:?}", lorem_ipsum_japanese);
    println!("Lorem Ipsum Russian\n{:?}", lorem_ipsum_russian);
    let my_box = data_types::MyBox::new(5);
    println!("Box: {}", *my_box);
    println!("Story Time!");
    match read_story() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error {:?}", e),
    }
    println!(
        "Match test\n{}\n{}\n{}\n{}\n",
        match_stuff(3, "cat", -0.1),
        match_stuff(8, "cat", 1.1),
        match_stuff(2, "dog", 100.0),
        match_stuff(0, "stick", -1.1)
    );
}

fn match_stuff(n: u32, s: &str, f: f32) -> String {
    match (n, s, f < 0.0) {
        (1..=5, "cat", true) => {
            "N is between 1 and 5, the string is \"cat\" and f is negative.".into()
        }
        (1..=10, "cat", false) => {
            "N is between 1 & 10, the string is \"cat\" and f is positive.".into()
        }
        (2..=4, "dog", false) => {
            "N is between 2 & 4, the string is \"dog\" and f is negative.".into()
        }
        _ => "something else".into(),
    }
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

fn format_vector(v: &Vec2) -> String {
    format!("({x:0>7.03}, {y:0>7.03})", x = v.x(), y = v.y())
}

fn animate_drive<S: Into<String>>(car: &mut vehicles::cars::Car, label: S, start: u32, end: u32) {
    let delay_time = time::Duration::from_millis(50);
    let label = label.into();
    for n in start..end {
        print!("\x1B[2J\x1B[1;1H");
        let v = format_vector(car.velocity());
        let h = format_vector(car.heading());
        println!(
            "{} performing {} test.\nFrame {}\nVelocity {}\nHeading  {}\n",
            car.name(),
            label,
            n,
            v,
            h,
        );
        thread::sleep(delay_time);
        car.animate(0.23);
    }
}

fn test_drive(car: &mut vehicles::cars::Car) {
    car.set_state(vehicles::cars::State::Driving);
    animate_drive(car, "acceleration", 1, 100);
    car.set_state(vehicles::cars::State::Idle);
    animate_drive(car, "braking", 100, 200);
    car.set_state(vehicles::cars::State::Driving);
    animate_drive(car, "handling", 200, 220);
    car.set_state(vehicles::cars::State::Turning(Vec2::new(0.0, 1.0)));
    animate_drive(car, "steering", 220, 300);
    car.set_state(vehicles::cars::State::Parked);
    animate_drive(car, "final braking", 300, 330);
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

#[derive(Debug)]
struct StringCount(String, u32);

impl cmp::Ord for StringCount {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

impl cmp::PartialOrd for StringCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl cmp::PartialEq for StringCount {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl cmp::Eq for StringCount {}

fn top_n_entries(n: usize, map: &HashMap<String, u32>) -> Vec<StringCount> {
    let mut top = Vec::new();
    top.reserve_exact(n);
    for (index, entry) in map.iter().enumerate() {
        if index < n {
            top.push(StringCount(entry.0.to_string(), entry.1.clone()));
        } else {
            if index == n {
                top.sort();
            }
            if entry.1 > &top.get(0).unwrap().1 {
                top.pop();
                top.insert(0, StringCount(entry.0.to_string(), entry.1.clone()));
            }
        }
    }
    top
}

#[tokio::main]
async fn read_story() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(DATA_SOURCE_URL)
        .await?
        .text_with_charset("utf-8")
        .await?;
    let story = unescape(&resp).unwrap();
    let words = word_freq(&story);
    let letters = letter_freq(&story);
    let words = top_n_entries(5, &words);
    let letters = top_n_entries(5, &letters);
    let out = format!("{}\n{:#?}\n{:#?}", story, words, letters);
    Ok(out)
}
