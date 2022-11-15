use std::sync::Arc;
use gen::model::{AddCrmLeadRequest, AddCrmLeadResponse};
use lib::handler::RequestHandler;
use lib::toolbox::{RequestContext, Toolbox};
use lib::ws::Connection;
use crate::pipedrive::PipeDriveSdk;

pub struct AddCrmLeadHandler {
    pub pipedrive_sdk: PipeDriveSdk,
}
impl RequestHandler for AddCrmLeadHandler {
    type Request = AddCrmLeadRequest;
    type Response = AddCrmLeadResponse;
    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _conn: Arc<Connection>,
        req: Self::Request,
    ) {
        // let db: DbClient = toolbox.get_db();
        let sdk = self.pipedrive_sdk.clone();
        toolbox.spawn_response(ctx, async move {
            let deal = sdk.create_deal(&req.email, &req.username, &req.title, &req.message)
                .await?;
            Ok(deal)
        })
    }
}