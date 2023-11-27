use std::collections::HashMap;

use crate::{EnrichedTrade, Message};
use dashmap::DashMap;

pub struct PipelineDash {
    instrument_map: DashMap<u32, String>,
}

impl PipelineDash {
    pub fn new() -> PipelineDash {
        PipelineDash {
            instrument_map: DashMap::new(),
        }
    }
    pub fn process(&self, message: Message) -> Option<EnrichedTrade> {
        match message {
            Message::Instrument(instrument) => {
                self.instrument_map.insert(instrument.id, instrument.into());
            }

            Message::Trade(trade) => {
                let Some(instrument) = self.instrument_map.get(&trade.insturment_id) else {
                    return None;
                };

                return Some(EnrichedTrade {
                    insturment: instrument.clone(),
                    id: trade.id,
                    user_id: trade.user_id,
                    trade_px: trade.trade_px,
                    side: trade.side,
                });
            }
        };
        None
    }
}

impl Default for PipelineDash {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PipelineStd {
    instrument_map: HashMap<u32, String>,
}

impl PipelineStd {
    pub fn new() -> PipelineStd {
        PipelineStd {
            instrument_map: HashMap::new(),
        }
    }
    pub fn process(&mut self, message: Message) -> Option<EnrichedTrade> {
        match message {
            Message::Instrument(instrument) => {
                self.instrument_map.insert(instrument.id, instrument.into());
            }

            Message::Trade(trade) => {
                let Some(instrument) = self.instrument_map.get(&trade.insturment_id) else {
                    return None;
                };

                return Some(EnrichedTrade {
                    insturment: instrument.clone(),
                    id: trade.id,
                    user_id: trade.user_id,
                    trade_px: trade.trade_px,
                    side: trade.side,
                });
            }
        };
        None
    }
}

impl Default for PipelineStd {
    fn default() -> Self {
        Self::new()
    }
}
