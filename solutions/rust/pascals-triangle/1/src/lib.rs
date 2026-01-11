pub struct PascalsTriangle {
    row_count: u32,
}
fn factorial(f: u32) -> u32 {
    let mut ret = 1;
    for i in 1..f+1 {
        ret = ret * i;
    }
    ret
}
impl PascalsTriangle {
    pub fn calcTriangle(&self, x: u32, y: u32) -> Option<u32> {
       // def calc(n,k): 
       // return (factorial(n))/(factorial(k)*factorial(n-k))
       if x-y < 0 
       {
            return None;
            //panic!("bad choose");
       }
       //dbg!(factorial(12));
       Some((( factorial(x as u32) ) / ( factorial(y as u32) * factorial(x as u32-y as u32) )) as u32)
 
    }
    pub fn new(row_count: u32) -> Self {
        return Self { row_count };
        //todo!("create Pascal's triangle with {row_count} rows");
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        dbg!( self.calcTriangle(5,3) );
        let mut ret = Vec::<Vec::<u32>>::new();
        for i in 0..self.row_count/*.rev()*/ {
            let mut tmp = Vec::<u32>::new();
            for y in 0..i+1 {
                dbg!(i,y);
                tmp.push(self.calcTriangle(i,y).unwrap_or(1));
            }
            ret.push(tmp);
        }
        dbg!(ret.clone());
        ret
    }
}
