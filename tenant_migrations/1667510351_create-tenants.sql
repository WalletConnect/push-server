CREATE TABLE public.tenants
(
    id                        varchar(255) primary key default gen_random_uuid(),

    fcm_api_key               text,
    apns_topic                text,
    apns_certificate          text,
    apns_certificate_password text,

    created_at                timestamptz not null     default now(),
    updated_at                timestamptz not null     default now()
);