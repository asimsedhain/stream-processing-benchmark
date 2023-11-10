use crate::{EnrichedTrade, Message};
use dashmap::DashMap;

pub struct Pipeline {
    instrument_map: DashMap<u32, String>,
    user_map: DashMap<u32, String>,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            instrument_map: DashMap::new(),
            user_map: DashMap::new(),
        }
    }
    pub fn process(&self, message: Message) -> Option<EnrichedTrade> {
        match message {
            Message::Instrument(instrument) => {
                self.instrument_map.insert(instrument.id, instrument.into());
            }
            Message::User(user) => {
                self.user_map.insert(user.id, user.username);
            }
            Message::Trade(trade) => {
                let Some(instrument) = self.instrument_map.get(&trade.insturment_id) else {
                    return None;
                };
                let Some(user) = self.user_map.get(&trade.user_id) else {
                    return None;
                };

                return Some(EnrichedTrade {
                    insturment: instrument.clone(),
                    id: trade.id,
                    user: user.clone(),
                    trade_px: trade.trade_px,
                    side: trade.side,
                });
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
