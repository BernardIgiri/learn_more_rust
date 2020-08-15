pub struct FibIterator {
    previous: u32,
    second_previous: u32,
    result: Option<u32>,
}

pub fn fib(n: u32) -> Option<u32> {
    let mut fib_it = FibIterator::new();
    fib_it.nth(n as usize)
}

pub fn fib_closest_inverse(n: u32) -> u32 {
    let fib_it = FibIterator::new();
    let mut previous = 0;
    let mut count = 0;
    for value in fib_it {
        if value >= n {
            return if value - n > n - previous {
                count - 1
            } else {
                count
            };
        }
        previous = value;
        count += 1;
    }
    previous
}

pub fn fib_perfect_inverse(n: u32) -> Option<u32> {
    let mut fib_it = FibIterator::new();
    return match fib_it.position(|value| value == n) {
        Some(v) => Some(v as u32),
        None => None,
    };
}

impl FibIterator {
    pub fn new() -> FibIterator {
        FibIterator {
            previous: 1,
            second_previous: 0,
            result: Some(0),
        }
    }
}

impl Iterator for FibIterator {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let output = self.result;
        self.result = self.previous.checked_add(self.second_previous);
        self.second_previous = self.previous;
        self.previous = self.result.unwrap_or(self.previous);
        output
    }
}
