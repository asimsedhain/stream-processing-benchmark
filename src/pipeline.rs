use crate::{EnrichedTrade, Message};
use std::collections::HashMap;

pub struct Pipeline {
    instrument_map: HashMap<u32, String>,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            instrument_map: HashMap::new(),
        }
    }
    pub fn process(&mut self, message: Message) -> Option<EnrichedTrade> {
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

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}
