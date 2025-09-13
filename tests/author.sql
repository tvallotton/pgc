
-- @name: one_row_one_column :one
select author from author;

-- @name: one_row_many_coulmns :one
select author, book, 1 as one from author
join book on book.author_id = author.id;

-- @name: many_rows_one_column :many
select author from author;

-- @name: many_rows_many_coulmns :many
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


-- @name: required_parameter :one
select $val;

-- @name: optional_parameter :one
select ?val;

-- @name: full_author :many
select * from full_author;