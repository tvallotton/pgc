
-- @name: one_row_one_column :one
select author from author;

-- @name: one_row_many_coulmns :one
select author, book, 1 as one from author
join book on book.author_id = author.id;

-- @name: many_rows_one_column :many
select author from author;

-- @name: many_rows_many_coulmns :many
select author, book, 1 as one from author
join book on book.author_id = author.id;


-- @name: insert :one
insert into book (
    id, title, author_id, year, isbn, is_best_seller, genre
) values (
    $(book.id),
    $(book.title),
    $(book.author_id),
    $(book.year),
    $(book.isbn),
    $(book.is_best_seller),
    $(book.genre)
)
returning book 
    