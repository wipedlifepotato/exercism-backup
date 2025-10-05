pub fn factors(n: u64) -> Vec<u64> {
    let mut candidate = 2;
    let mut tmp = n;
    let mut ret_vec = Vec::<u64>::new();
    while tmp > 0 {
        if tmp % candidate == 0 {
            ret_vec.push(candidate);
            tmp /= candidate;
            continue;
        } 
        candidate += 1;
        if candidate > tmp {
            eprintln!("Not found candidate");
            break
        }
    }
    ret_vec
}
