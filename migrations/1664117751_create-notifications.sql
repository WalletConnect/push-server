CREATE TABLE public.notifications (
    id                varchar(255) PRIMARY KEY,
    client_id         varchar(255) NOT NULL,

    last_payload      jsonb        NOT NULL DEFAULT '{}'::jsonb,
    previous_payloads jsonb[]      NOT NULL DEFAULT ARRAY[]::jsonb[],

    last_received_at  timestamptz  NOT NULL DEFAULT now(),
    created_at        timestamptz  NOT NULL DEFAULT now(),

    CONSTRAINT fk_notifications_client_id FOREIGN KEY (client_id)
        REFERENCES public.clients (id)
);
