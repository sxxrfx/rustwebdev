use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub exp: DateTime<Utc>,
    pub account_id: AccountId,
    pub nbf: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<AccountId>,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountId(pub i32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAccount {
    pub email: String,
    pub password: String,
}
