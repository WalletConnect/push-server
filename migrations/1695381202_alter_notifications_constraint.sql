ALTER TABLE public.notifications
DROP CONSTRAINT fk_notifications_client_id,
ADD CONSTRAINT fk_notifications_client_id 
    FOREIGN KEY (client_id)
    REFERENCES public.clients (id)
    ON DELETE CASCADE;
