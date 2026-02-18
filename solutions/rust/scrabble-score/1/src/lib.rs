//#[derrive(Debug)]
#[derive(Debug,Eq, Hash, PartialEq)]
struct ScrabblePoint {
    Letters: Vec<char>,
    val: u8,
}
impl ScrabblePoint {
    fn New(letters: &str, val: u8) -> Self {
        let mut ret = Vec::<char>::new();//std::collections::HashSet::<char>::new();
        for ch in letters.chars() {
            dbg!(ch);
            ret.push(ch);
        }
        Self { Letters: ret, val: val }
    }
    fn contains(s: &Self, ch: u8) -> bool {
        //s.Letters.contains(ch as char) 
        s.Letters.contains(&(ch as char))
    }
}
/*
macro_rules! scrabblepointer {
    ($alph:expr, $val: expr) => {
        println!("value = {:?}", $expr);
    };
}*/
/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut result: u64 = 0;
    dbg!(word);
    let f_1 = ScrabblePoint::New("AEIOULNRST", 1);
   // dbg!(f_1);
    let f_2 = ScrabblePoint::New("DG", 2);
    let f_3 = ScrabblePoint::New("BCMP", 3);
    let f_4 = ScrabblePoint::New("FHVWY", 4);
    let f_5 = ScrabblePoint::New("K", 5);
    let f_6 = ScrabblePoint::New("JX", 8);
    let f_7 = ScrabblePoint::New("QZ", 10);
    let mut alphabits = std::collections::HashSet::<ScrabblePoint>::new();
    alphabits.insert(f_1);
    alphabits.insert(f_2);
    alphabits.insert(f_3);
    alphabits.insert(f_4);
    alphabits.insert(f_5);
    alphabits.insert(f_6);
    alphabits.insert(f_7);
    'uu: for ch in word.chars() {
        if !ch.is_ascii() {
            continue;
        }
        let mut found = false;
        for alph in &alphabits {
            let c = ch.to_uppercase().next().unwrap_or('0') as u8;
            dbg!(c);
            if c == '0' as u8 {
                continue 'uu;
            }
            if ScrabblePoint::contains(&alph, c) {
                dbg!(found);
                dbg!(c as char);
                result += alph.val as u64;
                found = true;
            }
        }
        if !found {
            println!("not found");
            result -= 1;
        }
    };
    dbg!(result);
    //todo!("Score {word} in Scrabble.");
    result
}

fn main() {
    let input = "a";
    let output = score(input);
    let expected = 1;
    assert_eq!(output, expected);
let input = "STRAßE";
let output = score(input);
let expected = 5;
assert_eq!(output, expected);
}
