use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountData {
    pub puuid: String,
    pub region: String,
    pub account_level: i32,
    pub name: String,
    pub tag: String,
    pub card: String,
    pub title: String,
    pub platforms: Vec<String>,
    pub updated_at: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub status: i32,
    pub data: AccountData
}