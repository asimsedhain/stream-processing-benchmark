use rtrb::RingBuffer;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use stream_processing::pipeline::DashmapPipeline;
use stream_processing::utils::push;
use stream_processing::utils::{get_channel_size, get_size_arg};
use stream_processing::{default_generator, Generator, Message};

fn main() {
    let n = get_size_arg();
    let channel_size = get_channel_size(n);

    let mut gen = default_generator(n);
    let pipeline = Arc::new(DashmapPipeline::default());
    let (mut trade_tx, mut trade_rx) = RingBuffer::new(channel_size / 2);
    let (meta_tx, mut meta_rx) = RingBuffer::new(channel_size / 10);

    thread::scope(|s| {
        s.spawn(|| {
            let mut meta_tx = meta_tx;
            for i in 0..n {
                let gen_value = gen.generate(i);

                match gen_value {
                    msg @ (Message::Instrument(_) | Message::User(_) | Message::Log(_)) => {
                        push(&mut meta_tx, msg)
                    }
                    msg @ Message::Trade(_) => push(&mut trade_tx, msg),
                }
            }
            drop(meta_tx);
            drop(trade_tx);
        });

        s.spawn({
            let pipeline = pipeline.clone();
            move || 'inner: loop {
                match trade_rx.pop() {
                    Ok(message) => {
                        let _ = pipeline.process(message);
                    }
                    Err(_) => {
                        if trade_rx.is_abandoned() {
                            break 'inner;
                        }
                        thread::sleep(Duration::from_micros(100));
                    }
                }
            }
        });

        s.spawn(move || 'inner: loop {
            match meta_rx.pop() {
                Ok(message) => {
                    let _ = pipeline.process(message);
                }
                Err(_) => {
                    thread::sleep(Duration::from_micros(100));
                    if meta_rx.is_abandoned() {
                        break 'inner;
                    }
                }
            }
        });
    });
}
