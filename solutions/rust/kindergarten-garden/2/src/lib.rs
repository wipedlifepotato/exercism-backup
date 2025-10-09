use std::collections::HashMap;
macro_rules! push_plants {
    ($line:expr, $student_idx:expr, $table:expr, $ret:expr) => {{
        for ch in $line.chars().skip($student_idx * 2).take(2) {
            if let Some(plant) = $table.get(&ch) {
                $ret.push(*plant);
            }
        }
    }};
}
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let table: HashMap<char, &str> = vec![
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ].into_iter().collect();

    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    let student_idx = match students.iter().position(|&s| s == student) {
        Some(idx) => idx,
        None => return Vec::new(),
    };

    let cleaned: String = diagram.replace(" ", "").replace("\n\n", "\n").trim().to_string();
    let lines: Vec<&str> = cleaned.split('\n').collect();

    let mut ret = Vec::new();

    for line in lines {
        push_plants!(line, student_idx, table, ret);
    }

    ret
}
