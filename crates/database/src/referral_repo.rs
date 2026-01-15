use common::{
    ReferralCode, ReferralCodeNew, ReferralOwner, ReferralOwnerNew, ReferralOwnerWithCode,
    ReferralRedemption, ReferralRedemptionNew,
};
use diesel::prelude::*;
use diesel::sql_types::{Bool, Int4, Int8, Jsonb, Nullable, Text, Timestamptz};
use diesel::{QueryableByName, sql_query};
use diesel::r2d2::{ConnectionManager, Error as PoolError, PooledConnection};
use diesel::{OptionalExtension, SelectableHelper};
use chrono::NaiveDateTime;
use serde_json::Value;

use crate::PgPool;
use crate::models::{
    DbReferralCode, DbReferralCodeNew, DbReferralOwner, DbReferralOwnerNew, DbReferralRedemption,
    DbReferralRedemptionNew,
};
use crate::schema::{referral_codes, referral_owners, referral_redemptions};

pub struct ReferralRepo {
    pool: PgPool,
}

#[derive(Debug, QueryableByName)]
pub struct ProjectExportRow {
    #[diesel(sql_type = Int8)]
    pub owner_id: i64,
    #[diesel(sql_type = Jsonb)]
    pub owner_meta: Value,
    #[diesel(sql_type = Timestamptz)]
    pub owner_created_at: NaiveDateTime,
    #[diesel(sql_type = Timestamptz)]
    pub owner_updated_at: NaiveDateTime,
    #[diesel(sql_type = Text)]
    pub code: String,
    #[diesel(sql_type = Bool)]
    pub code_is_active: bool,
    #[diesel(sql_type = Int4)]
    pub code_use_count: i32,
    #[diesel(sql_type = Timestamptz)]
    pub code_created_at: NaiveDateTime,
    #[diesel(sql_type = Timestamptz)]
    pub code_updated_at: NaiveDateTime,
    #[diesel(sql_type = Nullable<Int8>)]
    pub redemption_id: Option<i64>,
    #[diesel(sql_type = Nullable<Jsonb>)]
    pub redemption_meta: Option<Value>,
    #[diesel(sql_type = Nullable<Timestamptz>)]
    pub redemption_created_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub enum ReferralRepoError {
    Pool(PoolError),
    Diesel(diesel::result::Error),
    MultipleCodes { owner_id: i64, count: usize },
    CodeGeneration(String),
}

impl From<PoolError> for ReferralRepoError {
    fn from(error: PoolError) -> Self {
        Self::Pool(error)
    }
}

impl From<diesel::result::Error> for ReferralRepoError {
    fn from(error: diesel::result::Error) -> Self {
        Self::Diesel(error)
    }
}

impl ReferralRepo {
    pub fn new(pool: PgPool) -> Self {
        ReferralRepo { pool }
    }

    fn conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, PoolError> {
        self.pool.get().map_err(|_| {
            PoolError::ConnectionError(ConnectionError::BadConnection(
                "Failed to get connection".into(),
            ))
        })
    }

    pub fn create_owner(
        &self,
        owner: ReferralOwnerNew,
    ) -> Result<ReferralOwner, ReferralRepoError> {
        let mut conn = self.conn()?;
        let new_owner: DbReferralOwnerNew = owner.into();
        let inserted: DbReferralOwner = diesel::insert_into(referral_owners::table)
            .values(&new_owner)
            .get_result(&mut conn)?;
        Ok(inserted.into())
    }

    pub fn create_owner_with_code<F, E>(
        &self,
        owner: ReferralOwnerNew,
        build_code: F,
    ) -> Result<ReferralOwnerWithCode, ReferralRepoError>
    where
        F: FnOnce(i64) -> Result<String, E>,
        E: std::fmt::Display,
    {
        let mut conn = self.conn()?;
        conn.transaction(|conn| {
            let new_owner: DbReferralOwnerNew = owner.into();
            let inserted_owner: DbReferralOwner = diesel::insert_into(referral_owners::table)
                .values(&new_owner)
                .get_result(conn)?;

            let code = build_code(inserted_owner.id)
                .map_err(|err| ReferralRepoError::CodeGeneration(err.to_string()))?;
            let new_code = DbReferralCodeNew {
                owner_id: inserted_owner.id,
                code,
                is_active: true,
                use_count: 0,
            };
            let inserted_code: DbReferralCode = diesel::insert_into(referral_codes::table)
                .values(&new_code)
                .get_result(conn)?;

            Ok(ReferralOwnerWithCode {
                owner: inserted_owner.into(),
                code: inserted_code.into(),
            })
        })
    }

    pub fn create_code(&self, code: ReferralCodeNew) -> Result<ReferralCode, ReferralRepoError> {
        let mut conn = self.conn()?;
        let new_code: DbReferralCodeNew = code.into();
        let inserted: DbReferralCode = diesel::insert_into(referral_codes::table)
            .values(&new_code)
            .get_result(&mut conn)?;
        Ok(inserted.into())
    }

    pub fn create_redemption(
        &self,
        redemption: ReferralRedemptionNew,
    ) -> Result<ReferralRedemption, ReferralRepoError> {
        let mut conn = self.conn()?;
        conn.transaction(|conn| {
            let new_redemption: DbReferralRedemptionNew = redemption.into();
            let inserted: DbReferralRedemption = diesel::insert_into(referral_redemptions::table)
                .values(&new_redemption)
                .get_result(conn)?;

            let updated = diesel::update(referral_codes::table)
                .filter(referral_codes::code.eq(&inserted.code))
                .set(referral_codes::use_count.eq(referral_codes::use_count + 1))
                .execute(conn)?;

            if updated == 0 {
                return Err(ReferralRepoError::Diesel(diesel::result::Error::NotFound));
            }

            Ok(inserted.into())
        })
    }

    pub fn get_owner(&self, meta: Value) -> Result<Option<ReferralOwner>, ReferralRepoError> {
        let mut conn = self.conn()?;
        let owner = referral_owners::table
            .filter(referral_owners::meta.eq(meta))
            .select(DbReferralOwner::as_select())
            .first(&mut conn)
            .optional()?;
        Ok(owner.map(Into::into))
    }

    pub fn get_code(&self, owner_id: i64) -> Result<ReferralCode, ReferralRepoError> {
        let mut conn = self.conn()?;
        let codes: Vec<DbReferralCode> = referral_codes::table
            .filter(referral_codes::owner_id.eq(owner_id))
            .limit(2)
            .select(DbReferralCode::as_select())
            .load(&mut conn)?;
        match codes.len() {
            1 => Ok(codes.into_iter().next().unwrap().into()),
            0 => Err(ReferralRepoError::Diesel(diesel::result::Error::NotFound)),
            count => Err(ReferralRepoError::MultipleCodes { owner_id, count }),
        }
    }

    pub fn get_owner_with_code(
        &self,
        meta: Value,
    ) -> Result<Option<ReferralOwnerWithCode>, ReferralRepoError> {
        let owner = match self.get_owner(meta)? {
            Some(owner) => owner,
            None => return Ok(None),
        };
        let code = self.get_code(owner.id)?;
        Ok(Some(ReferralOwnerWithCode { owner, code }))
    }

    pub fn get_project_export_rows(
        &self,
        project: &str,
    ) -> Result<Vec<ProjectExportRow>, ReferralRepoError> {
        let mut conn = self.conn()?;
        let rows = sql_query(
            r#"
            SELECT
                o.id AS owner_id,
                o.meta AS owner_meta,
                o.created_at AS owner_created_at,
                o.updated_at AS owner_updated_at,
                c.code AS code,
                c.is_active AS code_is_active,
                c.use_count AS code_use_count,
                c.created_at AS code_created_at,
                c.updated_at AS code_updated_at,
                r.id AS redemption_id,
                r.meta AS redemption_meta,
                r.created_at AS redemption_created_at
            FROM referral_owners o
            JOIN referral_codes c ON c.owner_id = o.id
            LEFT JOIN referral_redemptions r ON r.code = c.code
            WHERE o.meta->>'project' = $1
            ORDER BY o.id, c.code, r.created_at
            "#,
        )
        .bind::<Text, _>(project)
        .load::<ProjectExportRow>(&mut conn)?;
        Ok(rows)
    }
}
