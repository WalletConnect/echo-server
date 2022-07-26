CREATE TABLE IF NOT EXISTS public.notifications
(
    id                varchar(255) primary key,
    client_id         varchar(255) not null,

    last_payload      jsonb        not null default '{}'::jsonb,
    previous_payloads jsonb[]      not null default array []::jsonb[],

    last_received_at  timestamptz  not null default now(),
    created_at        timestamptz  not null default now(),

    CONSTRAINT fk_notifications_client_id FOREIGN KEY (client_id)
        REFERENCES public.clients (id)
);