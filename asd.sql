
-- @name: foo :one
select 1 as foo, '2' as bar, '{}'::jsonb as myjson;

-- @name: asd :one
select 1;

-- @name: get_user_id :many
select * from "user" where id = $id;
