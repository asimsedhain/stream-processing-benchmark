use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use stream_processing::uitls::get_size_arg;
use stream_processing::{EnrichedTrade, Generator, Message};

fn main() {
    let n = get_size_arg();

    let mut gen = Generator::default();
    let pipeline = Pipeline::new();
    let channel_size = std::cmp::max(std::cmp::min(n / 1000, 100_000), 1000);
    let (tx, rx) = mpsc::sync_channel(channel_size);

    thread::scope(|s| {
        s.spawn(|| {
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
