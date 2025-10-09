pub fn series(digits: &str, len: usize) -> Vec<String> {
    let d:Vec<char> = digits.chars().collect();//to_vector();
    let mut idx = 0;
    let mut ret = Vec::<String>::new();
    while len + idx <= d.len() {
            let word: String = d[idx..idx+len].iter().collect();
            if word.len() != len {
                break;
            }
            ret.push(word);
            idx += 1;
    }
    ret
}
