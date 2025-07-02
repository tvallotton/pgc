

# Intended features


## Embedded structs

```sql
-- @name: get_by_id
select author, book
from author
join book from on author.id = book.author_id
where author.id = $id
```

```py
await queries.author.get_by_id(id)
```

## Argument grouping
```sql
-- @name: save
insert into book
values (
    $(book.title),
    $(book.author_id),
    $(book.year),
)
on conflict (id) do update set
    title = $(book.title),
    author_id = $(book.author_id),
    year = $(book.year),
returning *;
```

```py
await queries.book.save(id)
```

## Automatic CRUD
enabling CRUD will create for each model the following
queries:

* get_by_<unique key>
* delete_by_<unique key>
* save

```yaml
codegen:
  crud:
    include: all
    exclude:
      -
```

## Foreign key enums
It is a better practice to use foregin keys instead of enums in postgres, since this makes the program more extensible, and future changes to the enum easier to implement. For this reason you can mark a table as an enum in your config file. For example, here we have a schema with a foreign key enum:
```sql
create table user_role (
    id text primary key
);


insert into user_role values ('admin'), ('staff'), ('consumer');

create table "user" (
    id uuid primary key,
    email text not null unique,
    role text not null references user_role(id),
)
```
```yaml
codegen:
  options:
    enums:
      user_role:
```
```python
class UserRole(StrEnum):
    ADMIN = 'admin'
    STAFF = 'staff'
    CONSUMER = 'consumer'
```
However, if you don't specify your values in your schema, you may specify them in your config file
```yaml
user_role:
  - 'admin'
  - 'staff'
  - 'consumer'
```
