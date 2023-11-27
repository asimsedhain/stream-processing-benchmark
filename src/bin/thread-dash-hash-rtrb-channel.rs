use rtrb::RingBuffer;
use std::thread;
use std::time::Duration;
use stream_processing::pipeline::PipelineDash;
use stream_processing::utils::{get_channel_size, get_size_arg, push};
use stream_processing::{default_generator, Generator};

fn main() {
    let n = get_size_arg();
    let channel_size = get_channel_size(n);

    let mut gen = default_generator(n);
    let pipeline = PipelineDash::new();
    let (mut tx, mut rx) = RingBuffer::new(channel_size);

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..n {
                let gen_value = gen.generate(i);
                push(&mut tx, gen_value);
            }
            drop(tx);
        });

        s.spawn(move || {
            let pipeline = pipeline;
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
