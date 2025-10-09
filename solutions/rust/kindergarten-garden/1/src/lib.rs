pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let table: std::collections::HashMap<char, &str> = vec![
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ].into_iter().collect();
    let students = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana", "Joseph", 
            "Kincaid", "Larry"];
    /*let mut student_idx: u8 = 0;
    let mut found = false;
    for s in students {
        if String::from(s) == String::from(student) {
            found = true;
            break;
        }
        student_idx+=1;
    }
    dbg!(student_idx);
    dbg!(found);
    if !found {
        return Vec::<&'static str>::new();
    }*/
    let mut student_idx: u8 = 0;
    if let Some(index) = students.iter().position(
        |x| String::from(*x) == String::from(student)
    ) {
        student_idx = index as u8;
    } else {
        println!("not found");
        return Vec::<&'static str>::new();
    }
    let cleaned = diagram.replace(" ", "").replace("\n\n", "\n").replace("\n\n\n","\n").trim().to_string();
    let mut d = cleaned.split('\n');
    //let mut array: [Option<String>;4] = [None, None, None, None];
    dbg!(d.clone());
    dbg!(d.clone().count());

    let mut ret = Vec::<&'static str>::new();
    let mut count_chars: u8 = 0;
    match d.clone().count() {
        1 => {
            println!("one lines");
            let f = Some(d.next());
            dbg!(f);
            if let Some(f_str) = f {
                for ch in f_str.expect("Error").chars()
                    .skip((student_idx*2).into()) {
                    if count_chars == 2 {
                        break;
                    }
                    count_chars += 1;
                    println!("{}",ch);
                }
            }
        }
        2 => {
            println!("Two lines");
            let f = Some(d.next());
            let s = Some(d.next());
            /* todo to macros */
            count_chars = 0;                        
            if let Some(f_str) = f {
                for ch in f_str.expect("Error").chars()
                        .skip((student_idx*2).into()) {
                    if count_chars == 2 {
                        break;
                    }
                    count_chars += 1;
                    if let Some(plant) = table.get(&ch) {
                        ret.push(*plant);
                    }
                    println!("{}:{}",count_chars, ch);
                }
            }
            count_chars = 0;
            if let Some(s_str) = s {
                for ch in s_str.expect("Error").chars()
                        .skip((student_idx*2).into()) {
                    if count_chars == 2 {
                        
                        break;
                    }
                    if let Some(plant) = table.get(&ch) {
                        ret.push(*plant);
                    }
                    count_chars += 1;
                    println!("s:{}:{}",count_chars,ch);
                }
            } else {
                    println!("Error");
            }

            dbg!(f);
            dbg!(s);
        }
        _ => {
            println!("Not one, not two");
            return Vec::<&'static str>::new();
        }
    }
    dbg!(diagram);
    dbg!(student);
    dbg!(&ret);
    ret
}
