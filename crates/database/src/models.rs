use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::schema::{referral_codes, referral_owners, referral_redemptions};
use common::{
    ReferralCode as CommonReferralCode, ReferralCodeNew as CommonReferralCodeNew,
    ReferralOwner as CommonReferralOwner, ReferralOwnerNew as CommonReferralOwnerNew,
    ReferralRedemption as CommonReferralRedemption,
    ReferralRedemptionNew as CommonReferralRedemptionNew,
};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = referral_owners)]
pub struct DbReferralOwner {
    pub id: i64,
    pub meta: Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = referral_owners)]
pub struct DbReferralOwnerNew {
    pub meta: Value,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = referral_codes)]
#[diesel(primary_key(code))]
#[diesel(belongs_to(DbReferralOwner, foreign_key = owner_id))]
pub struct DbReferralCode {
    pub owner_id: i64,
    pub code: String,
    pub is_active: bool,
    pub use_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = referral_codes)]
pub struct DbReferralCodeNew {
    pub owner_id: i64,
    pub code: String,
    pub is_active: bool,
    pub use_count: i32,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = referral_redemptions)]
#[diesel(belongs_to(DbReferralCode, foreign_key = code))]
pub struct DbReferralRedemption {
    pub id: i64,
    pub code: String,
    pub meta: Option<Value>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = referral_redemptions)]
pub struct DbReferralRedemptionNew {
    pub code: String,
    pub meta: Option<Value>,
}

impl From<DbReferralOwner> for CommonReferralOwner {
    fn from(owner: DbReferralOwner) -> Self {
        Self {
            id: owner.id,
            meta: owner.meta,
            created_at: owner.created_at,
            updated_at: owner.updated_at,
        }
    }
}

impl From<CommonReferralOwnerNew> for DbReferralOwnerNew {
    fn from(owner: CommonReferralOwnerNew) -> Self {
        Self { meta: owner.meta }
    }
}

impl From<DbReferralCode> for CommonReferralCode {
    fn from(code: DbReferralCode) -> Self {
        Self {
            owner_id: code.owner_id,
            code: code.code,
            is_active: code.is_active,
            use_count: code.use_count,
            created_at: code.created_at,
            updated_at: code.updated_at,
        }
    }
}

impl From<CommonReferralCodeNew> for DbReferralCodeNew {
    fn from(code: CommonReferralCodeNew) -> Self {
        Self {
            owner_id: code.owner_id,
            code: code.code,
            is_active: code.is_active,
            use_count: code.use_count,
        }
    }
}

impl From<DbReferralRedemption> for CommonReferralRedemption {
    fn from(redemption: DbReferralRedemption) -> Self {
        Self {
            id: redemption.id,
            code: redemption.code,
            meta: redemption.meta,
            created_at: redemption.created_at,
        }
    }
}

impl From<CommonReferralRedemptionNew> for DbReferralRedemptionNew {
    fn from(redemption: CommonReferralRedemptionNew) -> Self {
        Self {
            code: redemption.code,
            meta: redemption.meta,
        }
    }
}
