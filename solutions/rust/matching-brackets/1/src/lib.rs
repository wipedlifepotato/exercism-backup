pub fn brackets_are_balanced(string: &str) -> bool {
    let mut count_open_b1 = 0;
    let mut count_open_b2 = 0;
    let mut count_open_b3 = 0;
    let mut count_close_b1 = 0;
    let mut count_close_b2 = 0;
    let mut count_close_b3 = 0;
    let mut last_opened = Vec::<char>::new();

    for letter in string.chars() {
        match letter {
            '{' => {
                count_open_b1 += 1;
                last_opened.push('{');
            }
            '[' => {
                count_open_b2 += 1;
                last_opened.push('[');
            }
            '(' => {
                count_open_b3 += 1;
                last_opened.push('(');
            }
            '}' => {
                if last_opened.is_empty() || *last_opened.last().unwrap() != '{' {
                    return false;
                }
                last_opened.pop();
                count_close_b1 += 1;
            }
            ']' => {
                if last_opened.is_empty() || *last_opened.last().unwrap() != '[' {
                    return false;
                }
                last_opened.pop();
                count_close_b2 += 1;
            }
            ')' => {
                if last_opened.is_empty() || *last_opened.last().unwrap() != '(' {
                    return false;
                }
                last_opened.pop();
                count_close_b3 += 1;
            }
            _ => {}
        }
    }

    count_open_b1 == count_close_b1
        && count_open_b2 == count_close_b2
        && count_open_b3 == count_close_b3
        && last_opened.is_empty() 
}