use std::collections::HashSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut nums = HashSet::<u32>::new();
    
    for num in factors {
         if *num == 0 {
            continue;
        }
        let mut multiple = *num;
        while multiple < limit {
            nums.insert(multiple);
            multiple += *num;
        }
    }
   // dbg!(&nums);
    let mut sum: u32 = 0;
    for num in nums {
        sum += num;
    }
    sum
}