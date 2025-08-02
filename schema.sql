create extension pg_trgm;
create table author (
  id uuid primary key default gen_random_uuid(),
  name text not null,
  birthday date
);

create table genre (
    id text primary key
);

create type genre2 as enum (
    'comedy',
    'drama',
    'science fiction',
    'fantasy',
    'biography'
);


create table book (
    id uuid primary key default gen_random_uuid(),
    title text not null,
    author_id uuid not null references author(id),
    year int not null,
    isbn text not null unique,
    is_best_seller bool default false,
    genre text not null references genre(id)
);

insert into genre values
    ('comedy'),
    ('drama'),
    ('science fiction'),
    ('fantasy'),
    ('biography');
