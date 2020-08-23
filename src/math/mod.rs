use hashbrown::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub struct FibonacciSequence {
    previous: (u64, u64),
    n: u8,
}

pub fn fib(n: u64) -> Option<u64> {
    let mut fib_it = FibonacciSequence::new();
    fib_it.nth(n as usize)
}

pub fn fib_inverse_rounded_up(n: u64) -> Option<u64> {
    let mut fib_it = FibonacciSequence::new();
    match fib_it.position(|value| value >= n) {
        Some(v) => Some(v as u64),
        None => None,
    }
}

/// Finds the number v such that fib(v) = n given n
/// # Example
/// ```
/// let fib_num = fib_inverse(55).unwrap();
/// assert_eq!(fib_num, 10);
/// ```
pub fn fib_inverse(n: u64) -> Option<u64> {
    let mut fib_it = FibonacciSequence::new();
    match fib_it.position(|value| value == n) {
        Some(v) => Some(v as u64),
        None => None,
    }
}

impl FibonacciSequence {
    pub fn new() -> FibonacciSequence {
        FibonacciSequence {
            previous: (1, 1),
            n: 0,
        }
    }
}

impl Iterator for FibonacciSequence {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.n += 1;
        match self.n {
            1 => Some(0),
            2 | 3 => Some(1),
            _ => match self.previous.0.checked_add(self.previous.1) {
                Some(v) => {
                    self.n = 3;
                    self.previous.0 = self.previous.1;
                    self.previous.1 = v;
                    Some(v)
                }
                None => None,
            },
        }
    }
}

pub fn word_freq<S: Into<String>>(text: S) -> HashMap<String, u32> {
    let mut freq = HashMap::new();
    let s = text.into();
    let words = s.unicode_words().collect::<Vec<&str>>();
    for w in words {
        let count = freq.entry(w.to_lowercase()).or_insert(0);
        *count += 1;
    }
    freq
}

pub fn letter_freq<S: Into<String>>(text: S) -> HashMap<String, u32> {
    let mut freq = HashMap::new();
    let s = text.into();
    let graphemes = UnicodeSegmentation::graphemes(s.as_str(), true).collect::<Vec<&str>>();
    for l in graphemes {
        let count = freq.entry(l.to_lowercase()).or_insert(0);
        *count += 1;
    }
    freq
}

pub fn next_factor_of_three(x: i32) -> i32 {
    let mut factor = x + 1;
    while factor % 3 != 0 {
        factor += 1;
    }
    factor
}

pub fn do_n_times(f: fn(i32) -> i32, n: u32, v: i32) -> i32 {
    let mut a = v;
    for _ in 0..n {
        a = f(a);
    }
    a
}

pub fn to_do_n_times(f: fn(i32) -> i32, n: u32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| do_n_times(f, n, x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_next_sequence_is_correct() {
        let expected_list = vec![0u64, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        let mut iterator = FibonacciSequence::new();
        for expected in expected_list {
            let actual = iterator.next().unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn fib_n_overflows() {
        let n = FibonacciSequence::new().nth(94);
        assert_eq!(n, None);
    }

    #[test]
    fn fib_max() {
        let n = FibonacciSequence::new().nth(93).unwrap();
        assert_eq!(n, 12200160415121876738);
    }
}
