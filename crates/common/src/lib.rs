use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ReferralOwner {
    pub id: i64,
    pub meta: Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralOwnerNew {
    pub meta: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralCode {
    pub owner_id: i64,
    pub code: String,
    pub is_active: bool,
    pub use_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralCodeNew {
    pub owner_id: i64,
    pub code: String,
    pub is_active: bool,
    pub use_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralRedemption {
    pub id: i64,
    pub code: String,
    pub meta: Option<Value>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralRedemptionNew {
    pub code: String,
    pub meta: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ReferralOwnerWithCode {
    pub owner: ReferralOwner,
    pub code: ReferralCode,
}
