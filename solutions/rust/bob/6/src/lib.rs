pub fn reply(message: &str) -> &str {
    let mut is_only_uppercase = true;
    let mut is_have_ask = false;
    let mut is_have_letters = false;
    //let mut is_have_numbers = false;
    if message.trim().ends_with('?') { //message.trim().chars().last() == Some('?') {
        is_have_ask = true;
    }
    for letter in message.chars() {
        let code = letter as u8;
        if code.is_ascii_lowercase() { //code >= b'a' && code <= b'z' {
            is_only_uppercase = false;
        }
       // if (b'0'..b'9').contains(&code) {
       //     is_have_numbers = true;
       // }

        if !is_have_letters && ((65..90).contains(&code) 
        || (97..122).contains(&code)) {
            is_have_letters = true;
        }

    }
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }
    if is_only_uppercase && is_have_letters && is_have_ask {
        "Calm down, I know what I'm doing!"
    } else if is_only_uppercase && is_have_letters {
        "Whoa, chill out!"
    } else if is_have_ask {
        "Sure."
    } else {
        "Whatever."
    }
}