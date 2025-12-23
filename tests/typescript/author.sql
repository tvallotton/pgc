
-- @name: oneRowOneColumn :one
select author from author;

-- @name: oneRowManyColumns :one
select author, book, 1 as one from author
join book on book.author_id = author.id;

-- @name: manyRowsOneColumn :many
select author from author;

-- @name: manyRowsManyColumns :many
select 1 as one, author, book from author
join book on book.author_id = author.id;


-- @name: insert :one
insert into author (id, full_name, birthday)
values (
    $(author.id),
    $(author.full_name),
    $(author.birthday)
)
returning author;


-- @name: requiredParameter :one
select $val;

-- @name: optionalParameter :one
select ?val;

-- @name: fullAuthor :many
select * from full_author;
