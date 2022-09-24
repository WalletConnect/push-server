CREATE TABLE IF NOT EXISTS public.clients
(
    id           uuid primary key     default gen_random_uuid(),

    push_type    varchar(4)  not null, -- fcm or apns
    device_token text        not null,

    created_at   timestamptz not null default now()
);