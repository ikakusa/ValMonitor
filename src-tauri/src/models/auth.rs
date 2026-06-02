use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct AuthSnapshot {
    pub access_token: String,
    pub entitlements_token: String,
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
}

impl AuthSnapshot {
    pub fn full_username(&self) -> String {
        format!("{}#{}", self.game_name, self.tag_line)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementsResponse {
    pub access_token: String,
    pub token: String,
    pub subject: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub acct: AccountInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub game_name: String,
    pub tag_line: String,
}
