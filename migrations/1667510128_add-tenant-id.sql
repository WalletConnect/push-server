ALTER TABLE public.clients
    ADD tenant_id varchar(255) NOT NULL DEFAULT '0000-0000-0000-0000';

ALTER table public.notifications
    ADD tenant_id varchar(255) NOT NULL DEFAULT '0000-0000-0000-0000';
