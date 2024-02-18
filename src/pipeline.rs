use crate::{EnrichedLog, EnrichedMessage, EnrichedTrade, Message};
use dashmap::DashMap;
use hashbrown::HashMap;

#[derive(Default)]
pub struct DashmapPipeline {
    instrument_map: DashMap<u32, String>,
    user_map: DashMap<u32, String>,
}

impl DashmapPipeline {
    pub fn process(&self, message: Message) -> Option<EnrichedMessage> {
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

                return Some(EnrichedMessage::Trade(EnrichedTrade {
                    insturment: instrument.clone(),
                    id: trade.id,
                    user: user.clone(),
                    trade_px: trade.trade_px,
                    side: trade.side,
                }));
            }
            Message::Log(log) => {
                let Some(instrument) = self.instrument_map.get(&log.insturment_id) else {
                    return None;
                };

                return Some(EnrichedMessage::Log(EnrichedLog {
                    insturment: instrument.clone(),
                    message: log.message,
                }));
            }
        };
        None
    }
}

#[derive(Default)]
pub struct HashbrownPipeline {
    instrument_map: HashMap<u32, String>,
    user_map: HashMap<u32, String>,
}

impl HashbrownPipeline {
    pub fn process(&mut self, message: Message) -> Option<EnrichedMessage> {
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

                return Some(EnrichedMessage::Trade(EnrichedTrade {
                    insturment: instrument.clone(),
                    id: trade.id,
                    user: user.clone(),
                    trade_px: trade.trade_px,
                    side: trade.side,
                }));
            }
            Message::Log(log) => {
                let Some(instrument) = self.instrument_map.get(&log.insturment_id) else {
                    return None;
                };

                return Some(EnrichedMessage::Log(EnrichedLog {
                    insturment: instrument.clone(),
                    message: log.message,
                }));
            }
        };
        None
    }
}
