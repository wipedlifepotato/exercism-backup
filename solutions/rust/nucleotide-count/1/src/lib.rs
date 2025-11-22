use std::collections::HashMap;
// ACGT
macro_rules! is_allowed_nucl {
    ($ch:expr) => {
        if $ch != 'A' && $ch != 'C' && $ch != 'G' && $ch != 'T' {
            return Err($ch);
        }
    }
}
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    is_allowed_nucl!(nucleotide);
    let mut count: usize = 0;
    for ch in dna.chars() {
        if ch == nucleotide {
            count += 1;
        }
        is_allowed_nucl!(ch);
    }
    Ok( count )
    //todo!("How much of nucleotide type '{nucleotide}' is contained inside DNA string '{dna}'?");
}
macro_rules! insert_nucl {
    ($ch:expr, $res:expr) => {
           $res.insert($ch, 0);
    }
}
pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut res = std::collections::HashMap::<char,usize>::new();
    insert_nucl!('A',res);
    insert_nucl!('C',res);
    insert_nucl!('T',res);
    insert_nucl!('G',res);
    for ch in dna.chars() {
        is_allowed_nucl!(ch);
        res.entry(ch).and_modify(|ch| *ch += 1).or_insert(1);
    }
    //todo!("How much of every nucleotide type is contained inside DNA string '{dna}'?");
    Ok(res)
}
/*

fn main() {
    assert_eq!(count('C', "CCCCC"), Ok(5));
    assert_eq!(count('X', "A"), Err('X'));
        
    let output = nucleotide_counts("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC");
    let mut expected = HashMap::new();
    expected.insert('A', 20);
    expected.insert('C', 12);
    expected.insert('G', 17);
    expected.insert('T', 21);

    assert_eq!(output, Ok(expected));
let output = nucleotide_counts("G");
let mut expected = HashMap::new();
expected.insert('A', 0);
expected.insert('C', 0);
expected.insert('G', 1);
expected.insert('T', 0);
assert_eq!(output, Ok(expected));
}
*/