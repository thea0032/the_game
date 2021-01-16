use rand::{self, random};
pub fn rand_round(input:f64) -> u128{
    if input % 1.0 < random(){
        input.floor() as u128
    } else {
        input.ceil() as u128
    }
}