use crate::error_code::ErrorCode;
use crate::toolbox::{RequestContext, Toolbox};
use crate::ws::*;
use core::marker::{Send, Sync};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;
use std::sync::Arc;

pub trait RequestHandler: Send + Sync {
    type Request: DeserializeOwned;
    type Response: Serialize + 'static;
    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    );
}

pub trait RequestHandlerErased: Send + Sync {
    fn handle(&self, toolbox: &Toolbox, ctx: RequestContext, conn: Arc<Connection>, req: Value);
}

impl<T: RequestHandler> RequestHandlerErased for T {
    fn handle(&self, toolbox: &Toolbox, ctx: RequestContext, conn: Arc<Connection>, req: Value) {
        let data: T::Request = match serde_json::from_value(req) {
            Ok(data) => data,
            Err(err) => {
                toolbox.send(
                    ctx.connection_id,
                    request_error_to_resp(
                        &ctx,
                        ErrorCode::new(100400), // Bad Request
                        err.to_string(),
                    ),
                );
                return;
            }
        };

        RequestHandler::handle(self, toolbox, ctx, conn, data)
    }
}
