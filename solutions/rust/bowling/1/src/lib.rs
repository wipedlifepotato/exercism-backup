#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct BowlingGame {
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        dbg!(&self);
        if pins == 6 && *self.rolls.last().unwrap_or(&0) == 5 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if pins == 10 && *self.rolls.last().unwrap_or(&0) == 6 {
             return Err(Error::NotEnoughPinsLeft);
        }
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }

        let frame = self.current_frame();

        if frame < 9 && self.is_second_in_frame() {
            let last = *self.rolls.last().unwrap();
            if last + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
        }

        self.rolls.push(pins);
        Ok(())
    }

    fn current_frame(&self) -> usize {
        let mut i = 0;
        let mut frame = 0;
        while i < self.rolls.len() && frame < 10 {
            if self.rolls[i] == 10 { i += 1; } else { i += 2; }
            frame += 1;
        }
        frame
    }

    fn is_second_in_frame(&self) -> bool {
        if self.rolls.is_empty() { return false; }
        let mut i = 0;
        let mut frame = 0;
        while i < self.rolls.len() && frame < 9 {
            if self.rolls[i] == 10 { i += 1; } else { i += 2; }
            frame += 1;
        }
        i == self.rolls.len() - 1 && self.rolls[self.rolls.len() - 1] != 10
    }

    fn is_game_complete(&self) -> bool {
        let mut i = 0;
        for frame in 0..10 {
            if i >= self.rolls.len() { return false; }
            let first = self.rolls[i];
            let second = *self.rolls.get(i + 1).unwrap_or(&0);
            if frame < 9 {
                i += if first == 10 { 1 } else { 2 };
            } else {
                if first == 10 || first + second == 10 {
                    return self.rolls.len() >= i + 3;
                } else {
                    return self.rolls.len() >= i + 2;
                }
            }
        }
        true
    }

    pub fn score(&self) -> Option<u16> {
        let mut total = 0;
        let mut i = 0;

        for _ in 0..10 {
            if i >= self.rolls.len() { return None; }

            let first = self.rolls[i];
            let second = *self.rolls.get(i + 1).unwrap_or(&0);

            if first == 10 {
                if i + 2 >= self.rolls.len() { return None; }
                total += 10 + self.rolls[i + 1] + self.rolls[i + 2];
                i += 1;
            } else if first + second == 10 {
                if i + 2 >= self.rolls.len() { return None; }
                total += 10 + self.rolls[i + 2];
                i += 2;
            } else {
                total += first + second;
                i += 2;
            }
        }
        Some(total)
    }
}

