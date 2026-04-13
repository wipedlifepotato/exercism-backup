pub fn encode(source: &str) -> String {
      if source.len() == 0 {
          return String::new();
      }
      let mut r = Vec::<String>::new();
      let mut count = 0;
      let mut last_char: char = '\0';
      let mut is_encoding = false;
      // TODO: fix logic with is encoding flag
      for ch in source.chars() {
            dbg!(last_char, ch);
            if last_char != ch {
                dbg!("no equal");
                if is_encoding {
                    let mut s:String = String::new();
                    if count > 1 {
                        s+= &count.to_string();
                    }
                    s+= &last_char.to_string();
                    r.push(s.to_string());
                    count = 0;
                    //is_encoding = false;
                }
                last_char = ch;
                is_encoding = true;
            }
            if is_encoding {
                count += 1;
            }
      }
 let mut s:String = String::new();
                    if count > 1 {
                        s+= &count.to_string();
                    }
                    s+= &last_char.to_string();
                    r.push(s.to_string());
/* 
      let c = match source.chars().next_back() {
            Some(x) => {
                x
            },
            None => { panic!("wrong input"); }
      };
      dbg!(c, last_char);
      let c1 = match r.last().expect("wrong").chars().next_back() {
        Some(x) => {
            x
        },
        None => { panic!("wrong input"); }
      };
      dbg!(c1);
      if last_char != c1 {
        r.push(c.to_string());
      }
      */
      //dbg!(r);
      return r.into_iter().collect();
    //todo!("Return the run-length encoding of {source}.");
}
fn mul(x: u32, y: u32) -> u32 {
    if y == 0 || x == 0 {
        return 1;
    }
    let mut ret = 0;
    for n in 0..y {
        dbg!(n,x,y);
        ret += x * y;
    }
    ret/y
}
fn s_to_num(s:String) -> u32 {
    if s.len() == 0 {
        return 1;
    }
    dbg!(s.clone());
    let mut ret = 0;
    let mut exp = s.len() - 1;
    for ch in s.bytes() {
        //dbg!( ch-48, exp, 10, mul(exp as u32,10) );
        ret += ((ch-48) as u32) * mul(exp as u32, 10);
        if exp != 0 { exp -= 1 }
    }
    //dbg!(ret);
    ret
}
pub fn decode(source: &str) -> String {
    if source.len() == 0 {
        return String::new();
    }
    let mut num = Vec::<char>::new();
    let mut ret = Vec::<char>::new();
    let mut ch:char = '\0';
    for byte in source.bytes() {
        
        // 48 and 48+9
        if byte >= '0' as u8 && byte <= '9' as u8 {
            // then is number
            num.push(byte as char);
        }
        else {
            ch = byte as char;
            dbg!(ch, &num);
            let s: String = num.clone().into_iter().collect();
            for i in 0..s_to_num(s) {
                ret.push(ch);
            }
            num.clear();
            dbg!("ret now", ret.clone());
        }
    }
    ret.into_iter().collect()
    //todo!("Return the run-length decoding of {source}.");
}

