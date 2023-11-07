use std::{collections::HashMap, env};
use stream_processing::{EnrichedTrade, Generator, Message};

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().unwrap();

    let mut gen = Generator::default();

    let mut pipeline = Pipeline::new();
    for i in 0..n {
        let message = gen.generate(i);
        let _ = pipeline.process(message);
    }
}

struct Pipeline {
    instrument_map: HashMap<u32, String>,
}

impl Pipeline {
    fn new() -> Pipeline {
        Pipeline {
            instrument_map: HashMap::new(),
        }
    }
    fn process(&mut self, message: Message) -> Option<EnrichedTrade> {
        match message {
            Message::Instrument(instrument) => {
                self.instrument_map.insert(instrument.id, instrument.into());
            }
            Message::Trade(trade) => {
                if let Some(instrument) = self.instrument_map.get(&trade.insturment_id) {
                    return Some(EnrichedTrade {
                        insturment: instrument.clone(),
                        id: trade.id,
                        user_id: trade.user_id,
                        trade_px: trade.trade_px,
                        side: trade.side,
                    });
                }
            }
        };
        None
    }
}
