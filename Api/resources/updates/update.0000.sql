CREATE TYPE public.rstatus AS ENUM (
    'Active',
    'Removed',
    'Blocked',
    'Suspended',
    'Other'
);

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;

CREATE TABLE IF NOT EXISTS public.project (
    uuid UUID PRIMARY KEY,
    updu UUID NOT NULL,
    updt TIMESTAMP NOT NULL,
    ver INTEGER NOT NULL,
    rstatus public.rstatus NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS public.solution (
    uuid UUID PRIMARY KEY,
    updu UUID NOT NULL,
    updt TIMESTAMP NOT NULL,
    ver INTEGER NOT NULL,
    rstatus public.rstatus NOT NULL,
    project UUID NOT NULL REFERENCES public.project (uuid),
    input_path VARCHAR NOT NULL,
    output_path VARCHAR,
    name VARCHAR NOT NULL,
    version VARCHAR NOT NULL
    );
