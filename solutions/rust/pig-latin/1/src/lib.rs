const VOWELS: [char; 5] = ['a','e','i','o','u'];
fn is_vowel(ch :char) -> bool {
    for c in VOWELS {
        if ch == c {
            return true;
        }
    }
    return false;
}
fn is_starts_with_vowel(ch: char) -> bool {
    return is_vowel(ch);
}
fn is_starts_with_xryt(input: &str)  -> bool {
    let v = input.chars();
    if v.clone().count() < 2 {
        return false;
    }
    let f = match v.clone().nth(0) {
        Some(x) => {
            x
        },
        None => {
            panic!("not found first");
        }
    };
    let s = match v.clone().nth(1) {
        Some(x) => {
            x
        },
        None => {
            panic!("not found second");
        }
    };
    dbg!(f,s);
    if (f == 'x' && s == 'r') || (f=='y' && s == 't') {
        return true;
    }
    return false;
}
fn is_rule4(input: &str) -> Rule3 {
    let mut idx = 0;

    for c in input.chars() {
        if c == 'y' {
            return Rule3::new(true, idx);
        }
        idx += 1;
    }

    Rule3::new(false, 0)
}

struct Rule3 {
    flag: bool,
    pos: usize,
}
impl Rule3 {
    fn new(flag: bool, pos: usize) -> Self {
        Self{flag,pos}
    }
}
fn is_rule3(input: &str) -> Rule3 {
    let mut v = input.chars();
    let mut idx = 0;
    let size = v.clone().count();
    while idx < size {
        let ch = v.next();
        match ch {
            Some(x) => {
                if is_vowel(x) {
                  return Rule3::new(false,0);
                }
            }, None => {}
        }
        if ch == Some('q') {
            if v.clone().next() == Some('u') {
                return Rule3::new(true, idx);
            }
        }
        idx += 1;
    }
    //if v.clone().nth(0) == Some('q') && v.clone().nth(1) == Some('u') {
    //    return true;
    //}
    Rule3::new(false,0)
}

pub fn translate(input: &str) -> String {
    if input.contains(' ') {
        let mut res = Vec::<String>::new();
        for w in input.to_string().split(" ") {
            res.push( translate(w) );
        }
        return res.join(" ");//into_iter().collect();
    }
        
    let mut i_vec = input.chars();
    let mut out = Vec::<char>::new();
    if is_rule3(input).flag {
        //let size = i_vec.clone().count();
        let mut reserv = Vec::<char>::new();
        for _ in 0..=is_rule3(input).pos + 1 {
            match i_vec.next() {
                Some(x) => {
                    reserv.push(x);
                },
                None => {}
            }
        }
        /*
        for c in i_vec {
            out.push(c);
        }*/
        while let Some(c) = i_vec.next() {
            //dbg!(c);
            out.push(c);
        }
        for c in reserv {
            out.push(c);
        }
        out.push('a');
        out.push('y');
        return out.into_iter().collect();
    } else if is_starts_with_vowel(i_vec.clone().next().expect("zero string")) || 
    is_starts_with_xryt(input) {
        for c in i_vec {
            out.push(c);
        }
        out.push('a');
        out.push('y');
        return out.into_iter().collect();
    }
    else if i_vec.clone().next() == Some('t')
    && i_vec.clone().nth(1) == Some('h')
    && i_vec.clone().nth(2) == Some('r')
    {
        i_vec.next();
        i_vec.next();
        i_vec.next();
        for c in i_vec {
            out.push(c);
        }
        out.push('t');
        out.push('h');
        out.push('r');
        out.push('a');
        out.push('y');
        return out.into_iter().collect();
    } 
    else if i_vec.clone().next() == Some('t') && i_vec.clone().nth(1) == Some('h') {
       i_vec.next(); i_vec.next();
        for c in i_vec {
            out.push(c);
        }
        out.push('t');out.push('h');
        out.push('a');
        out.push('y');
        return out.into_iter().collect();
    }
    else if i_vec.clone().next() == Some('y') {
        i_vec.next();
        for c in i_vec {
            out.push(c);
        }
        out.push('y');
        out.push('a');
        out.push('y');
        return out.into_iter().collect();
    } 
    else if is_rule4(input).flag {
        let pos = is_rule4(input).pos;
      //  out.push('y');
       /* match i_vec.clone().nth(pos+1) {
            Some(x) => {
                dbg!("rule4", x);
        //        out.push(x);
            }, 
            None => {
            }
        };*/
        let size = i_vec.clone().count();
        let mut idx = 0;
        let mut reserv = Vec::<char>::new();
        while idx < pos {
            
            match i_vec.next() {
                Some(x) => {
                    // ythmrh
                    reserv.push(x)
                },
                None => {}
            }
            idx+=1;
        }
        while idx < size {
             match i_vec.next() {
                Some(x) => {
                    // ythmrh
                    out.push(x)
                },
                None => {}
            }
            idx+=1;       
        }
        for c in reserv {
            out.push(c);
        }
        out.push('a');
        out.push('y');
        return out.into_iter().collect();
        //todo!("is rule4");
    } 
    else {
        // rule 2
        let mut reserv = Vec::<char>::new();
        loop {
            let c = i_vec.next() ;
            match c {
                Some(x)=> {
                    if is_vowel(x) {
                        out.push(x);
                        break;
                    }
                    reserv.push(x);
                },
                None=>{
                    break;
                }
            };
        }
        loop {
            let c = i_vec.next();
            match c {
                Some(x)=> {
                    out.push(x);
                },
                None=>{
                    break;
                }
            };        
        }
        for c in reserv {
            out.push(c);
        }
        out.push('a');
        out.push('y');
        return out.into_iter().collect();
    }
    //for ch in i_vec {
    //    dbg!(ch);
    //}
   // todo!("Using the Pig Latin text transformation rules, convert the given input '{input}'");
}

fn main() {
    dbg!(translate("apple"));
    dbg!(translate("xray"));
    dbg!(translate("yttria"));


    dbg!(translate("quick"));
    dbg!(translate("square"));

    dbg!(translate("my"));
    dbg!(translate("rhythm"));

    dbg!(translate("pig"));
    dbg!(translate("chair"));
    dbg!(translate("thrush"));
let input = "therapy";
let output = translate(input);
let expected = "erapythay";
assert_eq!(output, expected);
let input = "quick fast run";
let output = translate(input);
let expected = "ickquay astfay unray";
assert_eq!(output, expected);
      
}
