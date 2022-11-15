use crate::error_code::ErrorCode;
use crate::handler::RequestHandlerErased;
use crate::log::LogLevel;
use crate::toolbox::RequestContext;
use eyre::*;
use model::endpoint::EndpointSchema;
use serde::*;
use serde_json::Value;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicI64, AtomicU32};
use std::sync::Arc;
use tracing::*;

pub type ConnectionId = u32;
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WsRequestGeneric<Req> {
    pub method: u32,
    pub seq: u32,
    pub params: Req,
}
pub type WsRequest = WsRequestGeneric<serde_json::Value>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WsResponseError {
    pub method: u32,
    pub code: u32,
    pub seq: u32,
    pub log_id: String,
    pub params: Value,
}

#[derive(Debug)]
pub struct Connection {
    pub connection_id: ConnectionId,
    pub user_id: AtomicI64,
    pub role: AtomicU32,
    pub address: SocketAddr,
    pub log_id: u64,
}
impl Connection {
    pub fn get_user_id(&self) -> i64 {
        self.user_id.load(std::sync::atomic::Ordering::Relaxed)
    }
}

pub type WsSuccessResponse = WsSuccessResponseGeneric<serde_json::Value>;
pub type WsStreamResponse = WsStreamResponseGeneric<serde_json::Value>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsForwardedResponse {
    pub method: u32,
    pub seq: u32,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsSuccessResponseGeneric<Params> {
    pub method: u32,
    pub seq: u32,
    pub params: Params,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsStreamResponseGeneric<Params> {
    pub method: u32,
    pub stream_seq: u32,
    pub resource: String,
    pub data: Params,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsLogResponse {
    pub seq: u32,
    pub log_id: u64,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum WsResponseGeneric<Resp> {
    Immediate(WsSuccessResponseGeneric<Resp>),
    Stream(WsStreamResponseGeneric<Resp>),
    Error(WsResponseError),
    Log(WsLogResponse),
    Forwarded(WsForwardedResponse),
    Close,
}

pub type WsResponse = WsResponseGeneric<serde_json::Value>;

pub struct WsEndpoint {
    pub schema: EndpointSchema,
    pub handler: Arc<dyn RequestHandlerErased>,
}

pub fn internal_error_to_resp(ctx: &RequestContext, code: ErrorCode, err0: Error) -> WsResponse {
    let log_id = ctx.log_id.to_string();
    let err = WsResponseError {
        method: ctx.method,
        code: code.to_u32(),
        seq: ctx.seq,
        log_id,
        params: Value::Null,
    };
    error!("Internal error: {:?} {:?}", err, err0);
    WsResponse::Error(err)
}

pub fn request_error_to_resp(
    ctx: &RequestContext,
    code: ErrorCode,
    params: impl Into<Value>,
) -> WsResponse {
    let log_id = ctx.log_id.to_string();
    let params = params.into();
    let err = WsResponseError {
        method: ctx.method,
        code: code.to_u32(),
        seq: ctx.seq,
        log_id,
        params,
    };
    warn!("Request error: {:?}", err);
    WsResponse::Error(err)
}
