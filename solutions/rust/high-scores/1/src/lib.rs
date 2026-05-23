#[derive(Debug)]
pub struct HighScores {
     scores: Vec::<u32>,
}
impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut s = Vec::<u32>::new();
        for score in scores {
            s.push(*score);
        }
        
        HighScores {
            scores: s
        }
        
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores[..]
        //todo!("Return all the scores as a slice")
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.len() == 0 {
            return None;
        }
        Some(self.scores[self.scores.len()-1])
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.len() == 0 {
            return None;
        }
        let mut max = self.scores[0];
        for score in self.scores.clone() {
            if score > max {
                max = score;
            }
        }
        Some(max)
        //todo!("Return the highest score")
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.scores.clone();
        sorted.sort();
        sorted.iter().rev().take(3).cloned().collect()
        //todo!("Return 3 highest scores")
    }
}

fn main() {
    let s = HighScores::new(&[1,2,3,4]);
    dbg!(s.scores());
    dbg!(s.latest());
    dbg!(s.personal_best());
}