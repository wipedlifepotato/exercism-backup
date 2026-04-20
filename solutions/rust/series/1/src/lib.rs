pub fn series(digits: &str, len: usize) -> Vec<String> {
    let d:Vec<char> = digits.chars().collect();//to_vector();
    let mut idx = 0;
    let mut ret = Vec::<String>::new();
    loop {
            if len + idx > d.len() {
                break;
            }
            let word: String = d[idx..idx+len].iter().collect();
            if word.len() != len {
                break;
            }
            ret.push(String::from(word));
            idx += 1;
    }
    ret
}
