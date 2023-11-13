ALTER TABLE public.notifications
DROP CONSTRAINT notifications_pkey;

ALTER TABLE public.notifications
ALTER COLUMN id SET not null;

ALTER TABLE public.notifications
ADD PRIMARY KEY (id, client_id);
