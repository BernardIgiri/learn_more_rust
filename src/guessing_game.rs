use super::math;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessing_game(min: u32, max: u32) -> u32 {
    let mut guess_count = 0;
    let secret = rand::thread_rng().gen_range(min..max + 1);
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
    let r = math::fib_inverse_rounded_up((guess_count - 1).into()).unwrap_or(0) as u32;
    if r > 2 {
        r - 1
    } else {
        r
    }
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
