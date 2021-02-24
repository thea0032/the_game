use rand::{self, random}; //Random stuff
pub fn rand_round<T, P>(input: f64, mut cvt: P) -> T
where
    T: Copy,
    P: FnMut(f64) -> T, {
    if input % 1.0 < random() {
        cvt(input.floor())
    } else {
        cvt(input.ceil())
    }
} //Rounds stuff randomly: eg
  //11.15 has a 15% chance of being 12 and an 85% chance of being 11
  //12.5 has a 50% chance of being 13 and a 50% chance of being 12
  //etc
