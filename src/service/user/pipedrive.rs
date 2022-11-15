use eyre::*;
use serde::*;
use tracing::*;

#[derive(Clone)]
pub struct PipeDriveSdk {
    token: String,
    company: String,
    client: reqwest::Client,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipeDriveResponse<T> {
    pub success: bool,
    pub data: T,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipeDriveUser {
    pub id: u64,
    pub email: String,
    pub name: String,
}
impl PipeDriveSdk {
    pub fn new(token: impl Into<String>, company: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            company: company.into(),
            client: reqwest::Client::new(),
        }
    }
    pub fn get_url(&self, path: &str) -> String {
        if path.contains("?") {
            format!(
                "https://{}.pipedrive.com/v1/{}&api_token={}",
                self.company, path, self.token
            )
        } else {
            format!(
                "https://{}.pipedrive.com/v1/{}?api_token={}",
                self.company, path, self.token
            )
        }
    }
    pub async fn create_user(&self, email: &str, username: &str) -> Result<PipeDriveUser> {
        let url = self.get_url(&format!("users"));
        let body = serde_json::json!({
            "email": email,
            "name": username
        });
        let response: PipeDriveResponse<PipeDriveUser> = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.data)
    }
    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<PipeDriveUser>> {
        let url = self.get_url(&format!("users/find?term={}&search_by_email=1", email));
        let result: serde_json::Value = self.client.get(url).send().await?.json().await?;
        info!("find_user_by_email {:?}", result);

        let user: PipeDriveResponse<Option<Vec<PipeDriveUser>>> = serde_json::from_value(result)?;
        if let Some(user) = user.data.into_iter().flatten().next() {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
    pub async fn ensure_user(&self, email: &str, username: &str) -> Result<PipeDriveUser> {
        if let Some(user) = self.find_user_by_email(email).await? {
            return Ok(user);
        }
        self.create_user(email, username).await
    }

    pub async fn create_deal(
        &self,
        email: &str,
        username: &str,
        title: &str,
        message: &str,
    ) -> Result<serde_json::Value> {
        let user = self.ensure_user(email, username).await?;
        let url = self.get_url(&format!("leads"));
        let body = serde_json::json!({
            "title": format!("Lead of {} {} {}", username, email, title),
            "person_id": user.id,
            "value": message
        });
        let resp: PipeDriveResponse<serde_json::Value> = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.data)
    }
}
