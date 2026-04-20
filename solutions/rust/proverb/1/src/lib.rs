pub fn build_proverb(list: &[&str]) -> String {
    //if list.len() % 2 == 0 {
    //    eprintln!("Not correct list");
    //    return String::new();
    //}
    if list.len() == 0 {
        return String::new();
    }
    let mut ret_str = Vec::<String>::new();
    for idx in (0..list.len()).step_by(1) {
        if idx == list.len() - 1 {
            break
        }
        println!("{}",idx);
        let x = String::from(format!("For want of a {} the {} was lost.\n", list[idx], list[idx+1]));
        //println!("{}",x);
        ret_str.push(x)
    }
    ret_str.push(format!("And all for the want of a {}.", list.first().unwrap_or(&"")));
    ret_str.into_iter().collect()
}
