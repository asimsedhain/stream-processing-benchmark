use stream_processing::pipeline::Pipeline;
use stream_processing::utils::get_size_arg;
use stream_processing::{default_generator, Generator};

fn main() {
    let n = get_size_arg();

    let mut gen = default_generator(n);

    let pipeline = Pipeline::new();
    for i in 0..n {
        let message = gen.generate(i);
        let _ = pipeline.process(message);
    }
}
