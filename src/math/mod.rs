pub struct FibonacciSequence {
    previous: (u32, u32),
    n: u8,
}

pub fn fib(n: u32) -> Option<u32> {
    let mut fib_it = FibonacciSequence::new();
    fib_it.nth(n as usize)
}

pub fn fib_inverse_rounded_up(n: u32) -> Option<u32> {
    let mut fib_it = FibonacciSequence::new();
    return match fib_it.position(|value| value >= n) {
        Some(v) => Some(v as u32),
        None => None,
    };
}

/// Finds the number v such that fib(v) = n given n
/// # Example
/// ```
/// let fib_num = fib_inverse(55).unwrap();
/// assert_eq!(fib_num, 10);
/// ```
pub fn fib_inverse(n: u32) -> Option<u32> {
    let mut fib_it = FibonacciSequence::new();
    return match fib_it.position(|value| value == n) {
        Some(v) => Some(v as u32),
        None => None,
    };
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
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        self.n += 1;
        return match self.n {
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
        };
    }
}
