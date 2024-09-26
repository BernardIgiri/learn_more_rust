#![allow(clippy::vec_init_then_push)]

mod data_types;
mod greetings;
mod guessing_game;
mod math;
mod my_macros;
mod story;
mod vehicles;

use byteorder::{BigEndian, ReadBytesExt};
use greetings::english::greet;
use hashbrown::HashSet;
use math::*;
use std::io;
use std::{thread, time};
use story::read_story;
use tokio::sync::oneshot;
use vehicles::drive::test_drive;

const DATA_SOURCE_URL: &str =
    "https://raw.githubusercontent.com/BernardIgiri/learn_more_rust/master/data/book.txt";

fn main() {
    {
        let v = my_vec![1, 2, 3];
        for n in v {
            print!("{n}");
        }
        println!();
    }
    {
        let has_duplicates = [
            (1, 1),
            (1, 2),
            (1, 2),
            (1, 3),
            (2, 1),
            (2, 1),
            (2, 2),
            (2, 3),
            (2, 3),
            (2, 4),
            (2, 4),
            (2, 1),
        ];
        let mut set = HashSet::new();
        let mut has_no_duplicates = Vec::new();

        for item in has_duplicates {
            if set.insert(item) {
                has_no_duplicates.push(item);
            }
        }
        println!("De-duplicated {:?}", has_no_duplicates);
    }
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
    let score = guessing_game::guessing_game(1, 50);
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
    wait_for_story();
    println!(
        "Match test\n{}\n{}\n{}\n{}\n",
        match_stuff(3, "cat", -0.1),
        match_stuff(8, "cat", 1.1),
        match_stuff(2, "dog", 100.0),
        match_stuff(0, "stick", -1.1)
    );
    println!("Factors of three test:");
    play_with_numbers();
}

#[tokio::main]
async fn wait_for_story() {
    let delay_time = time::Duration::from_millis(50);
    let (tx, mut rx) = oneshot::channel();
    tokio::spawn(async move {
        let result = match read_story(DATA_SOURCE_URL.into()).await {
            Ok(story) => story,
            Err(e) => e.to_string(),
        };
        tx.send(result).unwrap();
    });
    while match rx.try_recv() {
        Err(_) => true,
        Ok(story) => {
            println!("{}", story);
            false
        }
    } {
        println!("Waiting for story...");
        thread::sleep(delay_time);
    }
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

fn play_with_numbers() {
    let f = to_do_n_times(next_factor_of_three, 3);
    for n in 0..100 {
        println!("{}\t{}\t{}", n, next_factor_of_three(n), f(n));
    }
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
