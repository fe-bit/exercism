struct Roll {
    pins: i32
}

struct Frame {
    rolls: Vec<i32>
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            rolls: Vec<i32>::new()
        }
    }
    pub fn status(&self) -> BowlingStatus {
        todo!();
    }

    pub fn add_roll(&mut self, pins:i32) -> Result<bool>{
        todo!
    }
}

enum BowlingStatus {
    Strike,
    Spare(v, x),
    OpenFrame(z, y),
    RollLeft
}

struct BowlingGame {
    rolls: Vec<Frame>
}

impl BowlingGame {
    fn roll(&mut self, pins:int) -> Result<()>{
        let last_frame = self.rolls.last();
        if last_frame.status() != BowlingStatus::RollLeft && self.rolls.len() == 10 {
            return Err("Not allowed to roll another time");
        }
        if self.rolls.last().status() == BowlingStatus::RollLeft {
            self.rolls.last().add_roll(pins).unwrap();
        } else {
            let frame = Frame::new();
            frame.add_roll(pins);
            self.rolls.push(frame);
        }        
        Ok(())
    }


    fn score(&self) -> i32 {
        
    }
}

fn main() {
    println!("Hello, world!");
}
