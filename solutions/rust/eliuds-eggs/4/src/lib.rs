fn get_bits(mut inp: u32) -> Vec::<char> {
    let mut ret = Vec::<char>::new();
    loop {
        if inp % 2 == 1 {
            ret.push('1');
        } else {
            ret.push('0');
        }
        inp /= 2;
        if inp == 0 {
            break;
        }
    }
    ret.into_iter().rev().collect()
}
pub fn egg_count(display_value: u32) -> usize {
    let bits = get_bits(display_value);
    bits.iter().fold(0, |mut acc,x| {if *x == '1' { acc+=1; }; acc })
}
