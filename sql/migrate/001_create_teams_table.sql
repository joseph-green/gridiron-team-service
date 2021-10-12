CREATE TABLE IF NOT EXISTS public.teams
(
    "user" uuid NOT NULL,
    name character varying(256) NOT NULL,
    id uuid NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE public.teams
    OWNER to postgres;