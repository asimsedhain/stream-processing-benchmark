use fake::faker::company::en::CompanySuffix;
use fake::Dummy;

pub mod generator;
pub mod pipeline;
pub mod utils;

pub use generator::{FakeGenerator, Generator, StaticGenerator, MAPPING_SIZE};

pub fn default_generator(_max_size: usize) -> impl Generator {
    FakeGenerator::default()
}

#[derive(Dummy)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Dummy)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Clone, Dummy)]
pub struct Instrument {
    #[dummy(faker = "0..10000")]
    pub id: u32,
    #[dummy(faker = "CompanySuffix()")]
    pub ticker: String,
    #[dummy(faker = "0..1000")]
    pub strike: u32,
    #[dummy(faker = "100000..999999")]
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

#[derive(Dummy)]
pub struct Trade {
    #[dummy(faker = "0..10000000")]
    pub id: u32,
    #[dummy(faker = "0..10000")]
    pub insturment_id: u32,
    #[dummy(faker = "0..10000")]
    pub user_id: u32,
    #[dummy(faker = "0..100")]
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
