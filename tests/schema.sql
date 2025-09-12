create table if not exists author (
  id uuid primary key default gen_random_uuid(),
  full_name text not null,
  birthday date
);

create table if not exists genre (
    id text primary key
);

create table if not exists book (
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
    ('biography')
on conflict do nothing;
