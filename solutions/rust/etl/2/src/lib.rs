use std::collections::BTreeMap;
//const one_point: &[char; 10] = &['a','e','i','o','u','l','n','r','s','t'];

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    //todo!("How will you transform the tree {h:?}?")
    //https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
    let mut ret = BTreeMap::<char,i32>::new();
    for (idx, val) in h {
        dbg!(idx,val);
        let vals: Vec<char> = val.iter().flat_map(|ch| ch.to_lowercase()).collect();
        for ch in vals {
            dbg!(ch);
            ret.insert(ch, *idx);
        }
    }
    ret
    //todo!("...");
}

/*
fn main() {
   let mut map = BTreeMap::<i32, Vec<char>>::new();
   map.insert(1, vec!['a','b','c']);
   dbg!(transform(&map));
   
   let input = BTreeMap::from([(1, vec!['A', 'E', 'I', 'O', 'U'])]);
   let expected = BTreeMap::from([('a', 1), ('e', 1), ('i', 1), ('o', 1), ('u', 1)]);
   assert_eq!(expected, transform(&input));              
}*/