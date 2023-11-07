use std::env;

pub fn get_size_arg() -> usize {
    let args: Vec<String> = env::args().collect();
    args[1].parse().unwrap()
}
