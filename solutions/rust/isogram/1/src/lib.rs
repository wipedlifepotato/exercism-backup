
pub fn check(candidate: &str) -> bool {
    let mut letters = std::collections::HashSet::<char>::new();
    let mut found = false;
    candidate.chars().flat_map(|ch| ch.to_lowercase()).for_each(|ch| { if !found { if letters.contains(&ch) && ch.is_alphabetic() { found = true; } letters.insert(ch); } } );
    return !found;
}
