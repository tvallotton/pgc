create table foo (
    id uuid primary key,
    content text not null,
    bar uuid default gen_random_uuid()

);

create table bar (
    id uuid primary key,
    foo_id uuid references foo(id)
);


create type roles as enum (
    'client', 'staff', 'admin'
);
