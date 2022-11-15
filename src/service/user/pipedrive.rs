use std::collections::HashMap;
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
    // #[serde(default)]
    // pub additional_data: Option<AdditionalData>,
    // #[serde(default)]
    // pub related_objects: Option<RelatedObjects>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub error_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {
    pub id: i64,
    pub item_type: String,
    pub item_id: i64,
    pub active_flag: bool,
    pub add_time: String,
    pub update_time: String,
    pub added_by_user_id: i64,
    pub pictures: HashMap<u64, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipeDriveUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub has_pic: i64,
    pub pic_hash: String,
    pub active_flag: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub people_count: i64,
    pub owner_id: i64,
    pub address: String,
    pub active_flag: bool,
    pub cc_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedObjects {
    pub organization: HashMap<u64, Organization>,
    pub user: HashMap<u64, PipeDriveUser>,
    pub picture: HashMap<u64, Picture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub start: i64,
    pub limit: i64,
    pub more_items_in_collection: bool,
    pub next_start: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalData {
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PictureId {
    pub item_type: String,
    pub item_id: i64,
    pub active_flag: bool,
    pub add_time: String,
    pub update_time: String,
    pub added_by_user_id: i64,
    pub pictures: HashMap<u64, String>,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct1 {
    pub value: String,
    pub primary: bool,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgId {
    pub name: String,
    pub people_count: i64,
    pub owner_id: i64,
    pub address: String,
    pub active_flag: bool,
    pub cc_email: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerId {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub has_pic: i64,
    pub pic_hash: String,
    pub active_flag: bool,
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipeDrivePerson {
    pub id: i64,
    // pub company_id: i64,
    // pub owner_id: OwnerId,
    // pub org_id: OrgId,
    pub name: String,

    // pub first_name: String,
    // pub last_name: String,
    // pub open_deals_count: i64,
    // pub related_open_deals_count: i64,
    // pub closed_deals_count: i64,
    // pub related_closed_deals_count: i64,
    // pub participant_open_deals_count: i64,
    // pub participant_closed_deals_count: i64,
    // pub email_messages_count: i64,
    // pub activities_count: i64,
    // pub done_activities_count: i64,
    // pub undone_activities_count: i64,
    // pub files_count: i64,
    // pub notes_count: i64,
    // pub followers_count: i64,
    // pub won_deals_count: i64,
    // pub related_won_deals_count: i64,
    // pub lost_deals_count: i64,
    // pub related_lost_deals_count: i64,
    // pub active_flag: bool,
    // pub phone: Vec<String>,
    // pub emails: Vec<String>,
    pub primary_email: String,
    // pub first_char: String,
    // pub update_time: String,
    // pub add_time: String,
    // pub visible_to: String,
    // pub marketing_status: String,
    // pub picture_id: PictureId,
    // pub next_activity_date: String,
    // pub next_activity_time: String,
    // pub next_activity_id: i64,
    // pub last_activity_id: i64,
    // pub last_activity_date: String,
    // pub last_incoming_mail_time: String,
    // pub last_outgoing_mail_time: String,
    // pub label: i64,
    // pub org_name: String,
    // pub owner_name: String,
    // pub cc_email: String,
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
        info!("Creating user {} with email {}", username, email);
        let url = self.get_url(&format!("users"));
        let body = serde_json::json!({
            "email": email,
            "name": username
        });
        let response: PipeDriveResponse<PipeDriveUser> = self
            .client
            .post(url)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        if response.success {
            Ok(response.data)
        } else {
            Err(eyre!("Failed to create user: {}", response.error.unwrap_or_default()))
        }
    }
    pub async fn create_person(&self, email: &str, username: &str) -> Result<PipeDrivePerson> {
        info!("Creating person {} with email {}", username, email);
        let url = self.get_url(&format!("persons"));
        let body = serde_json::json!({
            "email": email,
            "name": username
        });
        let response = self
            .client
            .post(url)
            .json(&body)
            .send()
            .await?;
        let response = response.text().await?;
        info!("Response: {}", response);
        let response: PipeDriveResponse<PipeDrivePerson> = serde_json::from_str(&response)?;
        if response.success {
            Ok(response.data)
        } else {
            Err(eyre!("Failed to create person: {}", response.error.unwrap_or_default()))
        }
    }
    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<PipeDriveUser>> {
        info!("Finding user with email {}", email);
        let url = self.get_url(&format!("users/find?term={}&search_by_email=1", email));
        let result: serde_json::Value = self.client.get(url).send().await?.json().await?;
        info!("find_user_by_email {:?}", result);

        let user: PipeDriveResponse<Option<Vec<PipeDriveUser>>> = serde_json::from_value(result)?;
        if user.success {
            Ok(user.data.unwrap_or_default().pop())
        } else {
            Err(eyre!("Failed to find user: {}", user.error.unwrap_or_default()))
        }
    }
    pub async fn find_person_by_email(&self, email: &str) -> Result<Option<PipeDrivePerson>> {
        info!("Finding user with email {}", email);
        // TODO: this should be url encoded
        let url = self.get_url(&format!("persons/search?term={}&fields=email", email));
        let result = self.client.get(url).send().await?.text().await?;
        info!("find_user_by_email {}", result);

        #[derive(Debug, Serialize, Deserialize)]
        struct SearchResult {
            result_score: f64,
            item: PipeDrivePerson,
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct Persons {
            items: Vec<SearchResult>
        }
        let mut user: PipeDriveResponse<Persons> = serde_json::from_str(&result)?;
        if user.success {
            Ok(user.data.items.pop().map(|x| x.item))
        } else {
            Err(eyre!("Failed to find user: {}", user.error.unwrap_or_default()))
        }
    }
    pub async fn ensure_user(&self, email: &str, username: &str) -> Result<PipeDriveUser> {
        if let Some(user) = self.find_user_by_email(email).await? {
            return Ok(user);
        }
        self.create_user(email, username).await
    }
    pub async fn ensure_person(&self, email: &str, username: &str) -> Result<PipeDrivePerson> {
        if let Some(user) = self.find_person_by_email(email).await? {
            return Ok(user);
        }
        self.create_person(email, username).await
    }

    pub async fn create_deal(
        &self,
        email: &str,
        username: &str,
        title: &str,
        message: &str,
    ) -> Result<serde_json::Value> {
        let user = self.ensure_person(email, username).await?;
        info!("Creating deal {} for user {:?}", title, user);
        let url = self.get_url(&format!("leads"));
        let body = serde_json::json!({
            "title": format!("Lead of {} {} {} -- {}", username, email, title, message),
            "person_id": user.id,
            // "value": {
            //     "content": message
            // }
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
        info!("create_deal {:?}", resp);
        if resp.success {
            Ok(resp.data)
        } else {
            Err(eyre!("Failed to create deal {} {}", resp.error.unwrap_or_default(), resp.error_info.unwrap_or_default()))
        }
    }
}
