-- NOTE: Previous file is broken but migrations already run -> this is patched in this migration!

-- Delete old columns with incorrect types
ALTER TABLE public.tenants
    DROP COLUMN apns_key_id;

ALTER TABLE public.tenants
    DROP COLUMN apns_team_id;

ALTER TABLE public.tenants
    DROP COLUMN apns_pkcs8_pem;

-- Recreate with correct types
ALTER TABLE public.tenants
    ADD COLUMN apns_key_id text;

ALTER TABLE public.tenants
    ADD COLUMN apns_team_id text;

ALTER TABLE public.tenants
    ADD COLUMN apns_pkcs8_pem text;