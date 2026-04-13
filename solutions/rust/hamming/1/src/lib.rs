/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    if s1.contains(s2) {
        return Some(0);
    }
    if s1.len() == 1 {
        return Some(1);
    }
    let mut s = s1.len();
    let fc = s2.chars().next().unwrap();
    for ch in s1.chars() {
        if ch == fc { //s2[0 as usize] {
            s = s - 1;
            break
        }
        s = s - 1;
    }
    Some(s)
    //todo!("What is the Hamming Distance between {s1} and {s2}");
}