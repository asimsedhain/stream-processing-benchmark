use std::{env, thread, time::Duration};

pub fn get_size_arg() -> usize {
    let args: Vec<String> = env::args().collect();
    args[1].parse().unwrap()
}

pub fn get_channel_size(n: usize) -> usize {
    std::cmp::max(std::cmp::min(n / 1000, 100_000), 1000)
}

pub fn push<T>(tx: &mut rtrb::Producer<T>, mut value: T) {
    loop {
        match tx.push(value) {
            Ok(_) => break,
            Err(rtrb::PushError::Full(v)) => value = v,
        }
        thread::sleep(Duration::from_micros(100));
    }
}
