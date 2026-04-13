
//use regex::Regex;
/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    /*let re = Regex::new(r"^\d-\d{3}-\d{5}-\d$").unwrap();
    let Some(_) = re.captures(isbn) else {
        return false;
    };*/
    if isbn.len() == 0 {
        return false;
    }
   
    let s:Vec::<char> = isbn.chars().collect();
    let size_nums = s.len();
    if size_nums == 13 {
        if s[1] != '-' || s[5] != '-' || s[11] != '-' {
            return false;
        }
    } else if size_nums != 10 {
        return false;
    }
    
    let mut nums = Vec::<u32>::new();
    //let mut idx = 10;
    //let mut tmp = 0;
    let mut idx = 0;
     
    for ch in s {
        dbg!(ch);
        if ch == '-' {
            idx += 1;
            //idx -= 1;
            continue;
        }
        let n = ch as u8;
        if n < '0' as u8 || n > '9' as u8 {
            if n == b'X' && idx+1 == size_nums {
                nums.push( 10 );
                continue;
            } else {
                return false;
            }
        } 
        nums.push( (n-('0' as u8)) as u32 );
       // dbg!(n-('0' as u8),idx);
       // tmp += (n-('0' as u8))*idx;
       // idx -= 1;
       idx += 1;
    }
    dbg!(&nums);
    if nums.len() != 10 {
        return false;
    }
    //(d₁ * 10 + d₂ * 9 + d₃ * 8 + d₄ * 7 + d₅ * 6 + d₆ * 5 + d₇ * 4 + d₈ * 3 + d₉ * 2 + d₁₀ * 1) mod 11 == 0
   // let calculate = nums[0] * 10 + 
   //     nums[1] * 9 + nums[2] * 8 + nums[3] * 7 + nums[4] * 6 + nums[5] * 5 + nums[4] * 4 + nums[3] * 3 + nums[2] * 2 + nums[1] * 1;
    //dbg!(calculate);
    //tmp % 11 == 0
    //calculate % 11 == 0
    //todo!("Is {isbn:?} a valid ISBN number?");
    let mut calculate: u32 = 0;
    let mut idx = 10;
    nums.iter().for_each(|x| { calculate += x * idx; idx-= 1 } );
    dbg!(calculate);
    calculate % 11 == 0
}

fn main() {
    assert!(is_valid_isbn("3-598-21508-8"));
    assert!(is_valid_isbn("3598215088"));
    assert!(is_valid_isbn("359821507X"));
    assert!(!is_valid_isbn("3-598-2X507-9"));
}