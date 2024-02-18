use std::sync::mpsc;
use std::thread;
use stream_processing::pipeline::HashbrownPipeline;
use stream_processing::utils::{get_channel_size, get_size_arg};
use stream_processing::{default_generator, Generator};

fn main() {
    let n = get_size_arg();
    let channel_size = get_channel_size(n);

    let mut gen = default_generator(n);
    let pipeline = HashbrownPipeline::default();
    let (tx, rx) = mpsc::sync_channel(channel_size);

    thread::scope(|s| {
        s.spawn(move || {
            for i in 0..n {
                let _ = tx.send(gen.generate(i));
            }
            drop(tx);
        });

        s.spawn(move || {
            let mut pipeline = pipeline;
            while let Ok(message) = rx.recv() {
                let _ = pipeline.process(message);
            }
        });
    });
}
