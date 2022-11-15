use crate::model::*;
use eyre::*;
use lib::ws::WsClient;

pub struct UserClient {
    pub client: WsClient,
}
impl UserClient {
    pub fn new(client: WsClient) -> Self {
        Self { client }
    }
}
impl From<WsClient> for UserClient {
    fn from(client: WsClient) -> Self {
        Self::new(client)
    }
}

impl UserClient {
    pub async fn add_crm_lead(&mut self, req: &AddCrmLeadRequest) -> Result<AddCrmLeadResponse> {
        self.client.request(20660, req).await
    }
}
