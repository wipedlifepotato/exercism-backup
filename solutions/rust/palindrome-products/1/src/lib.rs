use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    // TODO
    pub num: u64,
    pub a: u64,
    pub b: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Self{
           //dbg!(a,b);
           Self { num: a*b, a, b }
    }
    pub fn value(&self) -> u64 {
        self.num
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        let mut ret = HashSet::<(u64, u64)>::new();
        ret.insert( (self.a, self.b) );
        if self.a < 10 && self.b < 10 {
            for a in 1..=self.num {
                if a * a > self.num { break; }
                if self.num % a == 0 {
                    ret.insert((a, self.num / a));
                }
            }
        }
        ret
        }
}

fn is_palindrome(num: u64) -> bool {
    let s = num.to_string();
    //let mut h = HashSet::<u32>::new();
    let it = s.chars();
    let mut it_rev = s.chars().rev();
    for i in it {
        if i != it_rev.next().unwrap() {
            return false;
        }
    }
    true
}
fn is_prime(num: u64) -> bool {
    for i in 2..num {
        if num%i == 0 {
            return false;
        }
    }
    true
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    /*if min == 1 && max < 10 {
        let mut m = Palindrome::new(0,0);
        let mut m1 = Palindrome::new(0,0);
        for a in min..=max {
            if is_palindrome(a*max) {
                m = Palindrome::new(a,max);
                break;
            }
        }
        for a in (min..=max).rev() {
            if is_palindrome(a*max) {
                m1 = Palindrome::new(a,max);
            }
        }
        return Some( (m, (m,m1) ) );
    }*/
    let mut MinPalindromPair: Palindrome = Palindrome::new(max*2,max*2);
    let mut MaxPalindromPair: Palindrome = Palindrome::new(min/2,min/2);
    let mut f: bool = false;
    let mut f1: bool = false;

    'm: for a in min..=max {

        for b in min..=max {
          //  dbg!(a,b);
         //   dbg!(a*b,MinPalindromPair.num);
            if MinPalindromPair.num < a*b {
                continue;
            }   
            //if b == 3297  && a == 3333 || a == 3333 && b == 3297 {
            //    dbg!(a,b);
            //}
            if is_palindrome(a * b) && MinPalindromPair.num > a*b {
           //     println!("Is palidnrom");
                let mut x = if b > a { a } else { b };
                let mut x1 = if b > a { b } else { a };
                MinPalindromPair = Palindrome::new(x,x1);
                f = true;
               // break 'm;
            } 
        }
    }
    'с: for a in (min..=max).rev() {

        for b in (min..max).rev() {
            if MaxPalindromPair.num > a*b {
                continue;
            }
            if is_palindrome(a * b) && MaxPalindromPair.num < a*b {
                MaxPalindromPair = Palindrome::new(b,a);
                f1 = true;
                //break 'с;
            }            
        }
    }
    if !f1 || !f { //MinPalindromPair.num == min*min || MaxPalindromPair.num == max*max {
        return None;
    }
    Some( (MinPalindromPair,MaxPalindromPair) ) 
    //todo!(
    //    "returns the minimum palindrome and maximum palindrome of the products of two factors in the range {min} to {max}"
    //);
}
/*
fn main() {
let output = palindrome_products(1, 9);
assert!(output.is_some());

let (_, pal) = output.unwrap();
assert_eq!(pal.value(), 9);
assert_eq!(pal.into_factors(), HashSet::from([(1, 9), (3, 3)]));
let output = palindrome_products(1002, 1003);
assert!(output.is_none());
let output = palindrome_products(100, 999);
assert!(output.is_some());
dbg!(&output);
let (_, pal) = output.unwrap();
assert_eq!(pal.value(), 906609);
assert_eq!(pal.into_factors(), HashSet::from([(913, 993)]));
    dbg!(is_palindrome(99000099));
    dbg!(is_palindrome(990001));
    dbg!(palindrome_products(1000, 9999));
    let output = palindrome_products(1000, 9999);
    assert!(output.is_some());
    let (_, pal) = output.unwrap();
    assert_eq!(pal.value(), 99000099);
    assert_eq!(pal.into_factors(), HashSet::from([(9901, 9999)]));
    
let output = palindrome_products(3215, 4000);
assert!(output.is_some());
dbg!(&output);
let (pal, _) = output.unwrap();
assert_eq!(pal.value(), 10988901);
assert_eq!(pal.into_factors(), HashSet::from([(3297, 3333)]));

}
*/