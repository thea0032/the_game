pub fn fill<T>(len: usize, value: T) -> Vec<T>
where
    T: Copy, {
    let mut res: Vec<T> = Vec::new();
    for _ in 0..len {
        res.push(value);
    }
    res
}
pub fn filter(mut input: usize, filter: &Vec<bool>) -> usize {
    for (i, item) in filter.iter().enumerate() {
        if *item {
            if input == 0 {
                return i;
            }
            input -= 1;
        }
    }
    panic!("The option selected was too high!");
}
