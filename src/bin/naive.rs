use stream_processing::pipeline::Pipeline;
use stream_processing::utils::get_size_arg;
use stream_processing::Generator;

fn main() {
    let n = get_size_arg();

    let mut gen = Generator::default();

    let mut pipeline = Pipeline::new();
    for i in 0..n {
        let message = gen.generate(i);
        let _ = pipeline.process(message);
    }
}
