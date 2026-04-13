#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    } else {
        println!("{} >= 2", from_base);
    }
    if number.is_empty() {
        return Ok([0].to_vec());
    }
    if to_base <= 1 { 
                return Err(Error::InvalidOutputBase);
    }
    if number.iter().all(|&x| x == 0) {
            return Ok([0].to_vec());
    }
    
    
    let mut numbers: u32 = 0;
    let number2vec = number.to_vec();
    let mut idx = number2vec.len();
    for num in number2vec {
            println!("{}",num);
            if num >= from_base {
                return Err(Error::InvalidDigit(num));
            } else {
                println!("{} < {}", num, from_base);
            }
            numbers += num * (from_base.pow((idx-1) as u32));
            idx-=1;
    }
    let mut ret = Vec::<u32>::new();
    loop {
        if numbers == 0 {
            break;
        }
        ret.push(numbers%to_base);
        numbers = numbers / to_base;
    }
    dbg!(numbers);
    //dbg!();
    Ok(ret.into_iter().rev().collect::<Vec<u32>>())
    //todo!("Convert {number:?} from base {from_base} to base {to_base}")
}



