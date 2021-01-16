
pub fn fill<T>(len:usize, zero:T) -> Vec<T> where T:Copy{
    let mut res:Vec<T> = vec![];
    for _ in 0..len{
        res.push(zero.clone());
    }
    return res;
}
pub fn filter(mut input:usize, filter:&Vec<bool>) -> usize{
    for i in 0..filter.len(){
        if filter[i]{
            if input == 0{
                return i;
            }
            input -= 1;
        }
    }
    panic!("The option selected was too high!");
}