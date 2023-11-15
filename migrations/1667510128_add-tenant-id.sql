alter table public.clients
    add tenant_id varchar(255) not null default '0000-0000-0000-0000';

alter table public.notifications
    add tenant_id varchar(255) not null default '0000-0000-0000-0000';
