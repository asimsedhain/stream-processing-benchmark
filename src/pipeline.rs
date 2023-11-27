use std::collections::HashMap;

use crate::{EnrichedTrade, Message};
use dashmap::DashMap;

#[derive(Default)]
pub struct PipelineDash {
    instrument_map: DashMap<u32, String>,
    user_map: DashMap<u32, String>,
}

impl PipelineDash {
    pub fn new() -> PipelineDash {
        PipelineDash::default()
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

#[derive(Default)]
pub struct PipelineStd {
    instrument_map: HashMap<u32, String>,
    user_map: HashMap<u32, String>,
}

impl PipelineStd {
    pub fn new() -> PipelineStd {
        PipelineStd::default()
    }

    pub fn process(&mut self, message: Message) -> Option<EnrichedTrade> {
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
