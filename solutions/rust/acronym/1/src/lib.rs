pub fn abbreviate(phrase: &str) -> String {
    let it = phrase.split(" ");
    let mut ret_str = Vec::<char>::new();
    for word in it {
            let mut before_is_up = false;
            let mut it_ = word.chars();
            if let Some(x) = it_.next() {
                if let Some(z) = x.to_uppercase().next() {
                    if z.is_alphabetic() {
                        ret_str.push(z);
                    }
                }
                if x.is_uppercase() {
                    before_is_up = true;
                }
            }
            let mut next_is_ab = false;
            while let Some(x) = it_.next() {
                if before_is_up && x.is_uppercase() {
                    continue;
                }
                if before_is_up {
                    before_is_up = false;
                }
                if next_is_ab {
                    next_is_ab = false;
                    if let Some(z) = x.to_uppercase().next() {
                         if z.is_alphabetic() {
                            ret_str.push(z);
                        }
                    }
                }
                if x.is_uppercase() {
                    ret_str.push(x);
                }
                if x == '-' {
                    next_is_ab = true;
                }
                //println!("{}",x);
            }
    }
    ret_str.iter().collect::<String>().to_string()
}
