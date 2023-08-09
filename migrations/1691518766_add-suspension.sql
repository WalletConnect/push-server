alter table public.tenants
    add suspended bool not null default false;

alter table public.tenants
    add suspended_reason text;