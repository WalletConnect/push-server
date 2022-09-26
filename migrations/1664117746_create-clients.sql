CREATE TYPE public.provider AS ENUM ('fcm', 'apns', 'noop');

CREATE TABLE IF NOT EXISTS public.clients
(
    id           varchar(255) primary key default gen_random_uuid(),

    push_type    public.provider not null,
    device_token text            not null,

    created_at   timestamptz     not null default now()
);