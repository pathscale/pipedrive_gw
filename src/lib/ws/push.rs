use crate::toolbox::Toolbox;
use crate::ws::{ConnectionId, WsResponseGeneric, WsStreamResponseGeneric};
use dashmap::DashMap;
use serde::Serialize;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default)]
pub struct SubscriberContext {
    pub connection_id: ConnectionId,
    pub stream_seq: u32,
}
#[derive(Default)]
pub struct Subscribers {
    pub subscribers: HashMap<ConnectionId, SubscriberContext>,
    pub method: u32,
}

pub struct SubscribeManager<Key: Eq + Hash> {
    pub topics: DashMap<Key, Subscribers>,
    pub toolbox: Toolbox,
}
impl<Key: Hash + Eq + ToString> SubscribeManager<Key> {
    pub fn new(toolbox: Toolbox) -> Self {
        Self {
            topics: DashMap::new(),
            toolbox,
        }
    }
    // TODO: change method code to stream code
    pub fn add_topics(&self, topics: Vec<Key>, method: u32) {
        for topic in topics {
            self.add_topic(topic, method);
        }
    }
    pub fn add_topic(&self, topic: Key, method: u32) {
        self.topics.entry(topic).or_insert_with(|| Subscribers {
            subscribers: HashMap::new(),
            method,
        });
    }
    pub fn subscribe_multi(&self, topics: Vec<Key>, connection_id: ConnectionId) {
        for topic in topics {
            self.subscribe(topic, connection_id);
        }
    }
    pub fn subscribe(&self, topic: Key, connection_id: ConnectionId) {
        let mut subscribers = self.topics.entry(topic).or_default();
        subscribers.subscribers.insert(
            connection_id,
            SubscriberContext {
                connection_id,
                stream_seq: 0,
            },
        );
    }
    pub fn unsubscribe_multi(&self, topics: Vec<Key>, connection_id: ConnectionId) {
        for topic in topics {
            self.unsubscribe(topic, connection_id);
        }
    }
    pub fn unsubscribe(&self, topic: Key, connection_id: ConnectionId) {
        let mut subscribers = self.topics.entry(topic).or_default();
        subscribers.subscribers.remove(&connection_id);
    }
    pub fn publish(&self, topic: Key, msg: &impl Serialize) {
        if let Some(mut topic) = self.topics.get_mut(&topic) {
            let data = serde_json::to_value(msg).unwrap();
            let resource = topic.key().to_string();
            let mut dead_connections = vec![];
            let method = topic.method;
            for sub in topic.subscribers.values_mut() {
                let msg = WsResponseGeneric::Stream(WsStreamResponseGeneric {
                    method,
                    stream_seq: sub.stream_seq,
                    resource: resource.clone(),
                    data: data.clone(),
                });
                sub.stream_seq += 1;
                if !self.toolbox.send(sub.connection_id, msg) {
                    dead_connections.push(sub.connection_id);
                }
            }
            for conn_id in dead_connections {
                topic.subscribers.remove(&conn_id);
            }
        }
    }
}
