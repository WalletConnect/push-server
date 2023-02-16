CREATE TYPE public.apns_type AS ENUM ('certificate', 'token');

ALTER TABLE public.tenants
    ADD COLUMN apns_type public.apns_type;

ALTER TABLE public.tenants
    ADD COLUMN apns_key_id public.apns_type;

ALTER TABLE public.tenants
    ADD COLUMN apns_team_id public.apns_type;

ALTER TABLE public.tenants
    ADD COLUMN apns_pkcs8_pem public.apns_type;