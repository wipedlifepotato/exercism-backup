pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() { return None; }
    let mut v = array.to_vec();
    v.sort();

    let mut low: isize = 0;
    let mut high: isize = (v.len() as isize) - 1;

    while low <= high {
        
        let mid = low + (high - low) / 2;
        let val = v[mid as usize];
        
        dbg!(mid, val);
        
        if val == key {
            return Some(mid as usize);
        } 
        
        if val < key {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}
