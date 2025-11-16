struct Student {
    grade: u32,
    name: String,
}
pub struct School {
    students: Vec<Student>,
}

impl School {
    pub fn new() -> School {
        Self {students: Vec::<Student>::new() }
        //todo!()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        for stud in &self.students {
            if stud.name == student {
                return;
            }
        }
        self.students.push( Student{grade: grade, name: student.to_string()} )
        //todo!("Add {student} to the roster for {grade}")
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut ret = std::collections::HashSet::<u32>::new();
        for stud in self.students.iter() {
            ret.insert(stud.grade);
        }
        //ret.sort();
        let mut r = ret.into_iter().collect::<Vec<u32>>();
        r.sort();
        r
        //todo!()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut ret = std::collections::HashSet::<String>::new();//Vec::<String>::new();
        for stud in &self.students {
            if stud.grade == grade {
                ret.insert(stud.name.clone()); //push(stud.name.clone());
            }
        }
        let mut r: Vec<String> = ret.into_iter().collect();
        r.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        //todo!("Return the list of students in {grade}")
        r
    }
}

fn main() {
    let s = School::new();
    assert_eq!(s.grade(1), Vec::<String>::new());
let mut s = School::new();
s.add(2, "Blair");
s.add(2, "James");
s.add(3, "James");
s.add(3, "Paul");
assert_eq!(s.grade(3), vec!["Paul"])
}