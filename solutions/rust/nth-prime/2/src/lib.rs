fn is_prime(n: u32) -> bool {
     if n < 2 {
         return false;
     }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u32;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
//static mut NTHS: Vec<u32> = Vec::new();

pub fn nth(n: usize) -> u32 {
  /*  unsafe {
        if n < NTHS.len() {
            return NTHS[n];
        }
        let mut candidate = NTHS.last().cloned().unwrap_or(2);
        while NTHS.len() <= n {
            if is_prime(candidate) {
                NTHS.push(candidate);
            }
            candidate += 1;
        }
        NTHS[n]
    }*/
    let mut count = 0;
    let mut candidate = 2;
    while count != n {
        if is_prime(candidate) {
            if count == n {
                return candidate;
            }
            count += 1
        }
        candidate += 1;
    }
    loop {
        if is_prime(candidate) {
            if count == n {
                return candidate;
            }
            count += 1;
        }
        candidate += 1;
    }
}
