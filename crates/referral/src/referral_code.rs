use crate::building_blocks::{ADJECTIVES, ANIMALS, COLORS};

const ANIM_COUNT: u32 = 223;
const COL_COUNT: u32 = 147;
const ADJ_COUNT: u32 = 90;
pub const MAX_ID: u32 = (ADJ_COUNT * COL_COUNT * ANIM_COUNT) - 1;
const ADJ_OFFSET: u32 = 17;
const COLOR_OFFSET: u32 = 29;
const COLOR_OFFSET_STEP: u32 = 37;
const ANIMAL_OFFSET: u32 = 41;
const ANIMAL_OFFSET_STEP: u32 = 83;
const ANIMAL_ADJ_STEP: u32 = 97;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferralCodeGenerationError {
    IdTooLarge(u32),
}

impl std::fmt::Display for ReferralCodeGenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReferralCodeGenerationError::IdTooLarge(id) => {
                write!(f, "id too large: {}", id)
            }
        }
    }
}

pub fn map_to_referral_code(id: u32) -> Result<String, ReferralCodeGenerationError> {
    if id > MAX_ID {
        return Err(ReferralCodeGenerationError::IdTooLarge(id));
    }

    let mut value = id;
    let base_adjective = value % ADJ_COUNT;
    value /= ADJ_COUNT;
    let base_color = value % COL_COUNT;
    value /= COL_COUNT;
    let base_animal = value % ANIM_COUNT;

    let adjective_idx = ((base_adjective + ADJ_OFFSET) % ADJ_COUNT) as usize;
    let color_idx =
        ((base_color + (base_adjective * COLOR_OFFSET_STEP) + COLOR_OFFSET) % COL_COUNT) as usize;
    let animal_idx = ((base_animal
        + (base_color * ANIMAL_OFFSET_STEP)
        + (base_adjective * ANIMAL_ADJ_STEP)
        + ANIMAL_OFFSET)
        % ANIM_COUNT) as usize;

    Ok(format!(
        "{}-{}-{}",
        ADJECTIVES[adjective_idx], COLORS[color_idx], ANIMALS[animal_idx]
    ))
}

#[cfg(test)]
mod tests {
    use super::{MAX_ID, map_to_referral_code};
    use std::collections::HashSet;

    #[test]
    fn all_codes_are_unique() {
        let total = (MAX_ID as usize) + 1;
        let mut codes = HashSet::with_capacity(total);

        for id in 0..=MAX_ID {
            let code = map_to_referral_code(id).expect("valid id");
            assert!(codes.insert(code), "duplicate code for id {}", id);
        }

        assert_eq!(codes.len(), total);
    }
}
