CREATE TABLE referral_owners (
  id BIGSERIAL PRIMARY KEY,
  meta JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE referral_codes (
  owner_id BIGINT NOT NULL REFERENCES referral_owners(id) ON DELETE CASCADE,
  code TEXT PRIMARY KEY,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  use_count INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT referral_codes_use_count_nonnegative CHECK (use_count >= 0)
);

CREATE TABLE referral_redemptions (
  id BIGSERIAL PRIMARY KEY,
  code TEXT NOT NULL REFERENCES referral_codes(code) ON DELETE CASCADE,
  meta JSONB,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX referral_owners_meta_unique_idx ON referral_owners (meta);
CREATE INDEX referral_owners_meta_gin_idx ON referral_owners USING GIN (meta);
CREATE INDEX referral_codes_owner_id_idx ON referral_codes (owner_id);
CREATE INDEX referral_redemptions_code_idx ON referral_redemptions (code);

SELECT diesel_manage_updated_at('referral_owners');
SELECT diesel_manage_updated_at('referral_codes');
