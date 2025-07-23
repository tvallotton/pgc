
-- @name: get_foo_by_id :one
select * from foo where id = $id;


-- @name: get_all_foos :many
select * from foo;

-- @name: get_foo_with_nullable :one
select * from foo where id = ?id;

-- name: get_foo_from_record :one
select * from foo where id = ?(foo.id);


-- name: get_foo_from_record2 :row
select * from foo where id = $(foo.id);
