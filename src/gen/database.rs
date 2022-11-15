use crate::model::*;
use eyre::*;
use lib::database::*;
use serde::*;

#[derive(Clone)]
pub struct DbClient {
    pub client: SimpleDbClient,
}
impl DbClient {
    pub fn new(client: SimpleDbClient) -> Self {
        Self { client }
    }
}
impl From<SimpleDbClient> for DbClient {
    fn from(client: SimpleDbClient) -> Self {
        Self::new(client)
    }
}
