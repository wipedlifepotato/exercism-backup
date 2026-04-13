//by chatgpt
fn factors_excluding_self(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    if n <= 1 {
        return factors;
    }

    let limit = (n as f64).sqrt() as u64;

    for i in 1..=limit {
        if n % i == 0 {
            let pair = n / i;

            // add i if it's not n itself
            if i != n {
                factors.push(i);
            }

            // add paired divisor if it's distinct and not n
            if pair != i && pair != n {
                factors.push(pair);
            }
        }
    }

    factors.sort_unstable();
    factors
}
#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
   if num == 0 {
       return None;
   }
    let mut sum = 0;

    for n in factors_excluding_self (num) {
        sum += n;
   }
   if sum == num {
        return Some(Classification::Perfect);
   }
   if sum > num {
        return Some(Classification::Abundant);
   }

   Some(Classification::Deficient)
}

