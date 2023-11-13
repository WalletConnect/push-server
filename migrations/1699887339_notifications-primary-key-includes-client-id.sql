ALTER TABLE public.notifications
    DROP CONSTRAINT notifications_pkey;

ALTER TABLE public.notifications
    ALTER COLUMN id SET NOT NULL;

ALTER TABLE public.notifications
    ADD PRIMARY KEY (id, client_id);
