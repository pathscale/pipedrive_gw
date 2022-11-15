mod method;

use crate::endpoints::*;
use crate::method::*;
use eyre::*;
use lib::config::{load_config, Config};
use lib::log::setup_logs;
use pipedrive::PipeDriveSdk;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use lib::http::HttpServer;

pub mod endpoints;
pub mod pipedrive;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct UserConfig {
    pipedrive_company: String,
    pipedrive_api_token: String,
}

impl Debug for UserConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserConfig")
            .field("securosys_hsm_token", &"***")
            .finish()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ActivityType {
    TransferProposed,
    TransferApproved,
    TransferStarted,
    TransferCompleted,
    Mint,
    Approve,
}
impl Display for ActivityType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config<UserConfig> = load_config("user".to_owned())?;
    setup_logs(config.app.log_level)?;

    let pipedrive_sdk = PipeDriveSdk::new(
        &config.app.extra.pipedrive_api_token,
        &config.app.extra.pipedrive_company,
    );
    let mut server = HttpServer::new(config.app.clone());

    server.add_handler(
        endpoint_user_add_crm_lead(),
        AddCrmLeadHandler { pipedrive_sdk },
    );

    server.listen().await?;
    Ok(())
}
