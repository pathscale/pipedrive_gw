use eyre::*;

use crate::handler::RequestHandlerErased;
use crate::toolbox::{RequestContext, Toolbox};
use crate::ws::{Connection, WsEndpoint};
use chrono::Utc;
use convert_case::Case;
use convert_case::Casing;
use futures::future::BoxFuture;
use futures::FutureExt;
use model::endpoint::*;
use model::types::Type;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_tungstenite::tungstenite::handshake::server::{
    Callback, ErrorResponse, Request, Response,
};
use tracing::*;

pub struct VerifyProtocol {
    pub addr: SocketAddr,
    pub tx: tokio::sync::mpsc::Sender<String>,
}

impl Callback for VerifyProtocol {
    fn on_request(
        self,
        request: &Request,
        mut response: Response,
    ) -> Result<Response, ErrorResponse> {
        let addr = self.addr;
        debug!(?addr, "handshake request: {:?}", request);
        let protocol = request
            .headers()
            .get("Sec-WebSocket-Protocol")
            .or_else(|| request.headers().get("sec-websocket-protocol"));
        let protocol_str = match protocol {
            Some(protocol) => protocol
                .to_str()
                .map_err(|_| {
                    ErrorResponse::new(Some("Sec-WebSocket-Protocol is not valid utf-8".to_owned()))
                })?
                .to_string(),
            None => "".to_string(),
        };
        self.tx.try_send(protocol_str.clone()).unwrap();
        response
            .headers_mut()
            .append("Date", Utc::now().to_rfc2822().parse().unwrap());
        response.headers_mut().insert(
            "Sec-WebSocket-Protocol",
            protocol_str
                .split(",")
                .next()
                .unwrap_or("")
                .parse()
                .unwrap(),
        );
        response
            .headers_mut()
            .insert("Server", "RustWebsocketServer/1.0".parse().unwrap());

        debug!(?addr, "Responding handshake with: {:?}", response);
        Ok(response)
    }
}
pub trait AuthController: Sync + Send {
    fn auth(
        self: Arc<Self>,
        toolbox: &Toolbox,
        header: String,
        conn: Arc<Connection>,
    ) -> BoxFuture<'static, Result<()>>;
}
pub struct SimpleAuthContoller;
impl AuthController for SimpleAuthContoller {
    fn auth(
        self: Arc<Self>,
        _toolbox: &Toolbox,
        _header: String,
        _conn: Arc<Connection>,
    ) -> BoxFuture<'static, Result<()>> {
        async move { Ok(()) }.boxed()
    }
}

pub struct EndpointAuthController {
    pub auth_endpoints: HashMap<String, WsEndpoint>,
}
impl EndpointAuthController {
    pub fn new() -> Self {
        Self {
            auth_endpoints: Default::default(),
        }
    }
    pub fn add_auth_endpoint(
        &mut self,
        schema: EndpointSchema,
        handler: impl RequestHandlerErased + 'static,
    ) {
        self.auth_endpoints.insert(
            schema.name.to_ascii_lowercase(),
            WsEndpoint {
                schema,
                handler: Arc::new(handler),
            },
        );
    }
}

impl AuthController for EndpointAuthController {
    fn auth(
        self: Arc<Self>,
        toolbox: &Toolbox,
        header: String,
        conn: Arc<Connection>,
    ) -> BoxFuture<'static, Result<()>> {
        let toolbox = toolbox.clone();

        async move {
            let splits = header
                .split(",")
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| (&x[..1], &x[1..]))
                .collect::<HashMap<&str, &str>>();

            let method = splits.get("0").context("Could not find method")?;
            let endpoint = self
                .auth_endpoints
                .get(*method)
                .with_context(|| format!("Could not find endpoint for method {}", method))?;
            let mut params = serde_json::Map::new();
            for (index, param) in endpoint.schema.parameters.iter().enumerate() {
                let index = index + 1;
                match splits.get(&index.to_string().as_str()) {
                    Some(value) => {
                        params.insert(
                            param.name.to_case(Case::Camel),
                            match &param.ty {
                                Type::String => {
                                    let decoded = urlencoding::decode(value)?;
                                    serde_json::Value::String(decoded.to_string())
                                }
                                Type::Int => serde_json::Value::Number(
                                    value
                                        .parse::<i64>()
                                        .with_context(|| {
                                            format!("Failed to parse integer: {}", value)
                                        })?
                                        .into(),
                                ),
                                Type::Boolean => serde_json::Value::Bool(
                                    value
                                        .parse::<bool>()
                                        .with_context(|| {
                                            format!("Failed to parse boolean: {}", value)
                                        })?
                                        .into(),
                                ),
                                Type::Enum { name, .. } if name == "service" => match *value {
                                    "2" => serde_json::Value::String("User".to_string()),
                                    "3" => serde_json::Value::String("Admin".to_string()),
                                    x => serde_json::Value::String(x.to_string()),
                                },
                                Type::Enum { .. } => serde_json::Value::String(value.to_string()),
                                Type::UUID => serde_json::Value::String(value.to_string()),
                                _ => todo!("Implement other types"),
                            },
                        );
                    }
                    None => {
                        bail!("Could not find param {} {}", param.name, index);
                    }
                }
            }

            endpoint.handler.handle(
                &toolbox,
                RequestContext {
                    connection_id: conn.connection_id,
                    user_id: 0,
                    seq: 0,
                    method: endpoint.schema.code,
                    log_id: conn.log_id,
                },
                conn,
                serde_json::Value::Object(params),
            );

            Ok(())
        }
        .boxed()
    }
}
