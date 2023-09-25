DELETE FROM public.clients
WHERE (device_token, created_at) NOT IN 
(
    SELECT device_token, MAX(created_at)
    FROM public.clients
    GROUP BY device_token
);
ALTER TABLE public.clients 
ADD CONSTRAINT device_token_unique UNIQUE(device_token);
