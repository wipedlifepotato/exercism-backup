pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut v = array.to_vec();
    v.sort();
    //let mut idx = 0;
    //v.into_iter().for_each(|val| { if val == key { return Some(idx); }; idx++ });
    for (idx,val) in v.iter().enumerate() {
        if key == *val {
            return Some(idx);
        }
    }
    None
}
