pub struct Allergies {
    score: u32,
    alerges :Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}
macro_rules! AddAlergenIfTrue {
    ($score:expr, $score_wait:expr, $allergen: expr, $vec:expr) => {
      if($score & $score_wait == $score_wait) {
          $vec.push($allergen);
      }  
    };
}
impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allerges = Vec::<Allergen>::new();
        AddAlergenIfTrue!(score, 1, Allergen::Eggs, allerges);
        AddAlergenIfTrue!(score, 2, Allergen::Peanuts, allerges);
        AddAlergenIfTrue!(score, 4, Allergen::Shellfish, allerges);
        AddAlergenIfTrue!(score, 8, Allergen::Strawberries, allerges);
        AddAlergenIfTrue!(score, 16, Allergen::Tomatoes, allerges);
        AddAlergenIfTrue!(score, 32, Allergen::Chocolate, allerges);
        AddAlergenIfTrue!(score, 64, Allergen::Pollen, allerges);
        AddAlergenIfTrue!(score, 128, Allergen::Cats, allerges);
        return Allergies{score: score, alerges: allerges};
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        for a in &self.alerges { // maybe clone there? but check though memory is good way
            if *a == *allergen {
                return true;
            }
        }
        return false;
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        return self.alerges.clone();
    }
}
