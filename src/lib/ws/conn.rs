use crossbeam::queue::SegQueue;
use dashmap::DashMap;
use futures::stream::SplitSink;
use std::sync::Arc;

use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use crate::ws::basics::Connection;
use crate::ws::WsResponse;

pub struct WebsocketStates<S> {
    connection: DashMap<u32, Arc<WsStreamSink<S>>>,
    states: Arc<DashMap<u32, Arc<WsStreamState>>>,
}
impl<S> WebsocketStates<S> {
    pub fn new() -> Self {
        Self {
            connection: Default::default(),
            states: Default::default(),
        }
    }
    pub fn remove(&self, connection_id: u32) {
        self.connection.remove(&connection_id);
        self.states.remove(&connection_id);
    }
    pub fn get_connection(&self, connection_id: u32) -> Option<Arc<WsStreamSink<S>>> {
        self.connection
            .get(&connection_id)
            .map(|x| x.value().clone())
    }
    pub fn get_state(&self, connection_id: u32) -> Option<Arc<WsStreamState>> {
        self.states.get(&connection_id).map(|x| x.value().clone())
    }
    pub fn clone_states(&self) -> Arc<DashMap<u32, Arc<WsStreamState>>> {
        Arc::clone(&self.states)
    }
    pub fn insert(
        &self,
        connection_id: u32,
        ws_stream: SplitSink<WebSocketStream<S>, Message>,
        conn: Arc<Connection>,
    ) {
        self.connection.insert(
            connection_id,
            Arc::new(WsStreamSink {
                ws_sink: tokio::sync::Mutex::new(ws_stream),
            }),
        );
        self.states.insert(
            connection_id,
            Arc::new(WsStreamState {
                conn,
                message_queue: SegQueue::new(),
            }),
        );
    }
}

pub struct WsStreamSink<S> {
    pub ws_sink: tokio::sync::Mutex<SplitSink<WebSocketStream<S>, Message>>,
}
pub struct WsStreamState {
    pub conn: Arc<Connection>,
    pub message_queue: SegQueue<WsResponse>,
}
