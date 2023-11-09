use std::env;

pub fn get_size_arg() -> usize {
    let args: Vec<String> = env::args().collect();
    args[1].parse().unwrap()
}

pub fn get_channel_size(n: usize) -> usize {
    std::cmp::max(std::cmp::min(n / 1000, 100_000), 1000)
}
