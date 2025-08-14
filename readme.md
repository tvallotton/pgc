## Pgc – PostgreSQL Query Compiler
*Note pgc is still under development and does not yet ship a stable release.*


Pgc is a type-safe SQL code generator for PostgreSQL, inspired by sqlc. It parses SQL queries, validates them against your schema, and generates strongly-typed models and async methods to execute them from your application code.

# Features
* Namespacing of queries
* Row types
* Grouping query arguments
* Foreign key enums
* Wasm plugin support


# Examples
for the following examples, this reference schema is used:
```sql
create table author (
  id uuid primary key default gen_random_uuid(),
  name text not null,
  birthday date
);

create table genre (
    id text primary key
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

```

## Introduction
Pgc is a compiler for postgres (heavily inspired on sqlc), which aims to type check postgres queries and generate models and methods to perform such queries.


## Namespaced queries

Queries are grouped by file name or an explicit `@namespace` directive:

```sql
-- book.sql
-- by default queries on this file will be found at queries.book.*

-- @name: get_by_id :one
select book.* from book where $id = id;

-- @namespace: author
-- @name: get_books :many
select book.* from book
join author on author.id = book.id
where author.id = $author_id
```
Now if we want to access each query we can use:
```python
await queries.book.get_by_id(book_id)
await queries.author.get_books(author_id)
```
Nested namespaces are also supported:
```sql
-- @namespace: book.metrics
-- @name: get_best_sellers :many
select book from book where book.is_best_seller;
```
Then this method can be accessed as:
```python
await queries.book.metrics.get_best_sellers()
```


## Row types

PostgreSQL supports returning composite row types directly. Pgc takes advantage of this to provide rich typed results for joined queries:
```sql
-- author.sql
-- @name: get_author_with_books :one
select author, array_agg(book) as books
from author
join book from on author.id = book.author_id
where book.id = $book_id
group by author.id
```

```py
row = await queries.author.get_author_with_books(author.id)
assert isinstance(Author, row.author)
assert isinstance(Book, row.books[0])
```
This saves us the need to construct an instance of `Book` and `Author` in our application from the resulting row.

## Argument grouping
When passing multiple arguments (e.g., in INSERT or UPDATE), use field path syntax for clarity and grouping:
* `$(record.field)`: for required agruments
* `?(record.field)`: for optional agruments

```sql
-- @name: upsert
insert into book
values (
    $(book.title),
    $(book.author_id),
    $(book.year),
    $(book.isbn),
    $(book.is_best_seller),
    $(book.genre)
)
on conflict (id) do update set
    title =          $(book.title),
    author_id =      $(book.author_id),
    year =           $(book.year),
    isbn =           $(book.isbn),
    is_best_seller = $(book.is_best_seller),
    genre =          $(book.genre)
returning book;
```

```py
await queries.book.upsert(book=book)
```


## Optional parameters
You may use `?` instead of `$` to declare an optional parameter:
```sql
select * from book
offset coalesce(?offset, 0)
limit coalesce(?limit, 24)
```

## Foreign key enums
Instead of using raw enum types in Postgres, prefer foreign-key-backed enums for extensibility:
```sql
create table genre (
    id text primary key
);

insert into genre values
    ('science fiction'),
    ('fantasy'),
    ('biography');
```
Mark these as enums in your config:
```yaml
codegen:
  options:
    enums:
      - genre
```

This generates:
```python
class Genre(enum.StrEnum):
    SCIENCE_FICTION = 'science fiction'
    FANTASY = 'fantasy'
    BIOGRAPHY = 'biography'
```
However, if you don't specify your values in your schema, you may specify them in your config file
```yaml
enums:
  - genre:
    - "science fiction"
    - "fantasy"
    - "biography"
```
