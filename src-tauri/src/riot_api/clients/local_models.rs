use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct Entitlements {
    pub access_token: String,
    pub token: String,
    pub subject: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountAlias {
    pub active: bool,
    pub created_datetime: i64,
    pub game_name: String,
    pub summoner: bool,
    pub tag_line: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Presence {

}