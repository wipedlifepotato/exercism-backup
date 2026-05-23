pub fn collatz(n: u64) -> Option<u64> {
    let mut our_num = n;
    let mut iterations: u64 = 0;
    const MAX_ITERATIONS: u64 = 1000;
    while our_num != 1 {
        dbg!(iterations);
        if our_num % 2 == 0 {
            our_num /= 2;
        } else {
            our_num = our_num * 3 + 1;
        }
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return None;
        }
    }
    return Some(iterations);
}

