use common::{ReferralOwnerNew, ReferralOwnerWithCode, ReferralRedemption, ReferralRedemptionNew};
use database::referral_repo::ReferralRepo;
use referral::referral_code::map_to_referral_code;
use serde_json::Value;

pub struct ReferralService {
    repo: ReferralRepo,
}

#[derive(Debug)]
pub enum ReferralServiceError {
    DatabaseError,
    CodeGenerationError,
}

impl ReferralService {
    pub fn new(repo: ReferralRepo) -> Self {
        ReferralService { repo }
    }

    pub fn create_owner(
        &self,
        new_owner: ReferralOwnerNew,
    ) -> Result<ReferralOwnerWithCode, ReferralServiceError> {
        let result = self
            .repo
            .create_owner_with_code(new_owner, |owner_id| map_to_referral_code(owner_id as u32))
            .map_err(|err| match err {
                database::referral_repo::ReferralRepoError::CodeGeneration(_) => {
                    ReferralServiceError::CodeGenerationError
                }
                _ => ReferralServiceError::DatabaseError,
            })?;

        Ok(result)
    }

    pub fn get_owner_with_code(
        &self,
        meta: Value,
    ) -> Result<Option<ReferralOwnerWithCode>, ReferralServiceError> {
        self.repo
            .get_owner_with_code(meta)
            .map_err(|_| ReferralServiceError::DatabaseError)
    }

    pub fn create_redemption(
        &self,
        redemption: ReferralRedemptionNew,
    ) -> Result<ReferralRedemption, ReferralServiceError> {
        self.repo
            .create_redemption(redemption)
            .map_err(|_| ReferralServiceError::DatabaseError)
    }
}
