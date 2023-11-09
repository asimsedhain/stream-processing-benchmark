use rtrb::RingBuffer;
use std::thread;
use std::time::Duration;
use stream_processing::pipeline::Pipeline;
use stream_processing::utils::{get_channel_size, get_size_arg};
use stream_processing::Generator;

fn main() {
    let n = get_size_arg();
    let channel_size = get_channel_size(n);

    let mut gen = Generator::default();
    let pipeline = Pipeline::new();
    let (mut tx, mut rx) = RingBuffer::new(channel_size);

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..n {
                'inner: loop {
                    if tx.push(gen.generate(i)).is_ok() {
                        break 'inner;
                    }
                }
            }
            drop(tx);
        });

        s.spawn(move || {
            let mut pipeline = pipeline;
            'inner: loop {
                match rx.pop() {
                    Ok(message) => {
                        let _ = pipeline.process(message);
                    }
                    Err(_) => {
                        thread::sleep(Duration::from_micros(100));
                        if rx.is_abandoned() {
                            break 'inner;
                        }
                    }
                }
            }
        });
    });
}
