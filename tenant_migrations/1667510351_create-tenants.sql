CREATE TABLE public.tenants
(
    id                        varchar(255) primary key default gen_random_uuid(),
    nickname                  varchar(255), -- Nickname to help identify the tenant

    fcm_api_key               text,
    apns_certificate          text,
    apns_certificate_password text,

    created_at                timestamptz not null     default now(),
    updated_at                timestamptz not null     default now()
);