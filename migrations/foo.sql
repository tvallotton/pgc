create table user_role (id text not null primary key);

create table "user" (
    id uuid primary key,
    email text not null unique,
    role text not null references user_role(id)
);
