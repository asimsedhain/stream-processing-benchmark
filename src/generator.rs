use crate::{Instrument, Message, OptionType, Side, Trade};
use fake::{Fake, Faker};
use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

pub const MAPPING_SIZE: usize = 10000;

pub trait Generator {
    fn generate(&mut self, size: usize) -> Message;
}

pub struct StaticGenerator {
    /// The class generates data as follows:
    ///
    instrument_mapping: [Instrument; MAPPING_SIZE],
    seen_count: usize,
    max_size: usize,
}

impl StaticGenerator {
    pub fn new(max_size: usize) -> StaticGenerator {
        let tickers = [
            "AAPL", "MSFT", "AMZN", "NVDA", "GOOGL", "TSLA", "GOOG", "META", "UNH", "XOM",
        ];

        let mapping: [Instrument; MAPPING_SIZE] = std::array::from_fn(|i| {
            let ticker = tickers[i % tickers.len()];

            let option_t = if i % 2 == 0 {
                OptionType::Call
            } else {
                OptionType::Put
            };
            Instrument {
                id: i as u32,
                ticker: ticker.to_string(),
                strike: i as u32 * 113,
                expiry: 231105,
                call_or_put: option_t,
            }
        });
        StaticGenerator {
            instrument_mapping: mapping,
            seen_count: 0,
            max_size,
        }
    }
}

impl Generator for StaticGenerator {
    fn generate(&mut self, size: usize) -> Message {
        if size % MAPPING_SIZE == 0 {
            self.seen_count += 1;
        }
        if size % (self.max_size / 10) == 0 {
            let i = (size % self.seen_count) % MAPPING_SIZE;
            let instrument = self.instrument_mapping[i].clone();
            Message::Instrument(instrument)
        } else {
            let size = size as u32;
            Message::Trade(Trade {
                id: size,
                insturment_id: size % self.seen_count as u32,
                user_id: size % 113,
                trade_px: 1,
                side: Side::Sell,
            })
        }
    }
}

pub struct FakeGenerator {
    uniform: Uniform<u8>,
    rng: StdRng,
}

impl Default for FakeGenerator {
    fn default() -> Self {
        let rng = StdRng::from_seed([
            34, 67, 90, 78, 23, 0, 17, 0, 200, 1, 68, 34, 210, 30, 9, 103, 90, 10, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);

        FakeGenerator {
            rng,
            uniform: Uniform::from(1..=100),
        }
    }
}

impl Generator for FakeGenerator {
    fn generate(&mut self, size: usize) -> Message {
        // generate 1000 instruments in the begining

        if size < 1000 {
            return Message::Instrument(Faker.fake_with_rng(&mut self.rng));
        }

        let gen = self.rng.sample(self.uniform);

        // generate instrument 10% of the time
        match gen {
            1..=10 => Message::Instrument(Faker.fake_with_rng(&mut self.rng)),
            11..=100 => Message::Trade(Faker.fake_with_rng(&mut self.rng)),
            _ => panic!(),
        }
    }
}
