CREATE TABLE IF NOT EXISTS public.message
(
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    message character varying(512)[] COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT message_pkey PRIMARY KEY (id)
)
