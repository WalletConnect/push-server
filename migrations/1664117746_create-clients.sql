CREATE TYPE public.provider AS ENUM ('fcm', 'apns', 'noop');

CREATE TABLE public.clients (
    id           varchar(255) PRIMARY KEY DEFAULT gen_random_uuid(),

    push_type    public.provider NOT NULL,
    device_token text            NOT NULL,

    created_at   timestamptz     NOT NULL DEFAULT now()
);
