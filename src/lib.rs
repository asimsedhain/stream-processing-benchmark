pub mod pipeline;
pub mod utils;

pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Clone)]
pub struct Instrument {
    pub id: u32,
    pub ticker: String,
    pub strike: u32,
    pub expiry: u32,
    pub call_or_put: OptionType,
}

impl From<Instrument> for String {
    fn from(instrument: Instrument) -> Self {
        let option_t = if let OptionType::Call = instrument.call_or_put {
            'C'
        } else {
            'P'
        };
        format!(
            "{}{}{}{}",
            instrument.ticker, instrument.expiry, option_t, instrument.strike
        )
    }
}

pub struct Trade {
    pub id: u32,
    pub insturment_id: u32,
    pub user_id: u32,
    pub trade_px: u32,
    pub side: Side,
}

pub enum Message {
    Instrument(Instrument),
    Trade(Trade),
}

pub struct EnrichedTrade {
    pub id: u32,
    pub insturment: String,
    pub user_id: u32,
    pub trade_px: u32,
    pub side: Side,
}

const MAPPING_SIZE: usize = 10000;
pub struct Generator {
    instrument_mapping: [Instrument; MAPPING_SIZE],
    seen_count: usize,
    max_size: usize,
}

impl Generator {
    pub fn new(max_size: usize) -> Generator {
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
        Generator {
            instrument_mapping: mapping,
            seen_count: 0,
            max_size,
        }
    }
    pub fn generate(&mut self, size: usize) -> Message {
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
