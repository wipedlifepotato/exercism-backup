/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    dbg!( sentence.len() );
    if sentence.len() == 0 {
        return false;
    }
    const ALPH_SIZE:u8 = 25;
    let mut count = 0;
    let mut alph = std::collections::HashSet::new();
    for i in 'a'/* as u8*/..'z'/* as u8*/{
        //dbg!(i);
        alph.insert( i /*as char*/ );
    }
    for ch in sentence.chars() {
        count += 1;
        let lowerch = match ch.to_lowercase().next() {
            Some(x) => {
                x
            },
            None => {
                panic!("wrong input");
            }
        };
        alph.remove(&lowerch);
    }
    dbg!(alph.len());
    dbg!(count);
    if count-1 - ALPH_SIZE != 0 {
        //return false;
    }
    alph.len() == 0
    //todo!();
}

