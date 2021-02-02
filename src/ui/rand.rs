use rand::{self, random}; //Random stuff
pub fn rand_round(input: f64) -> u64 {
    if input % 1.0 < random() {
        input.floor() as u64
    } else {
        input.ceil() as u64
    }
} //Rounds stuff randomly: eg
  //11.15 has a 15% chance of being 12 and an 85% chance of being 11
  //12.5 has a 50% chance of being 13 and a 50% chance of being 12
  //etc
