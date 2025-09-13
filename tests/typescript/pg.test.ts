import * as pg from "pg";
import { assert, assertEquals, assertInstanceOf } from "jsr:@std/assert";
import { initClient, Queries } from "./out_pg/queries.ts";
import { faker } from "@faker-js/faker";

import {
  PostgreSqlContainer,
  StartedPostgreSqlContainer,
} from "@testcontainers/postgresql";
import { randomUUID } from "node:crypto";
import { Genre } from "./out_pg/models/models.ts";
import { json } from "node:stream/consumers";

let CONTAINER: StartedPostgreSqlContainer | undefined = undefined;

Deno.test.beforeAll(async () => {
  CONTAINER = await new PostgreSqlContainer("postgres:16-alpine")
    .start();

  const client = new pg.Client({
    connectionString: CONTAINER.getConnectionUri(),
  });

  await client.connect();
  await client.query(await Deno.readTextFile("tests/schema.sql"));
});

async function getQueries() {
  const client = new pg.Client({
    connectionString: CONTAINER!.getConnectionUri(),
  });
  await client.connect();
  await initClient(client);
  const queries = new Queries(client);

  const author = await queries.author.insert({
    birthday: faker.date.birthdate(),
    full_name: faker.book.author(),
    id: randomUUID(),
  });

  await queries.book.insert({
    author_id: author!.id,
    year: faker.date.anytime().getFullYear(),
    genre: Genre.SCIENCE_FICTION,
    is_best_seller: false,
    isbn: faker.number.int().toString(),
    title: faker.book.title(),
    id: randomUUID(),
  });
  return queries;
}

Deno.test("test_one_row_one_column", async () => {
  const queries = await getQueries();
  const author = await queries.author.one_row_one_column();

  assert(typeof (author?.fullName) == "string", JSON.stringify(author));
  assertInstanceOf(author.birthday, Date);

  await queries.client.end();
});

Deno.test.afterAll(async () => {
  await CONTAINER?.stop();
});

// @pytest.mark.asyncio
// async def test_one_row_one_column(queries: Queries):
//     data = await queries.author.one_row_many_coulmns()

//     assert isinstance(data.author, Author)
//     assert isinstance(data.book, Book)
//     assert data.one == 1

// @pytest.mark.asyncio
// async def test_many_rows_one_column(queries: Queries):
//     rows = await queries.author.many_rows_one_column()

//     assert isinstance(rows[0], Author)

// @pytest.mark.asyncio
// async def test_many_rows_many_columns(queries: Queries):
//     rows = await queries.author.many_rows_many_coulmns()

//     assert isinstance(rows[0].author, Author)
//     assert isinstance(rows[0].book, Book)
//     assert rows[0].one == 1

// @pytest.mark.asyncio
// async def test_required_parameter(queries: Queries):
//     assert "foo" == await queries.author.required_parameter("foo")

// @pytest.mark.asyncio
// async def test_optional_parameter(queries: Queries):
//     assert (await queries.author.optional_parameter(None)) is None
//     assert (await queries.author.optional_parameter("foo")) == "foo"

// @pytest.mark.asyncio
// async def test_query_view(queries: Queries):
//     full_author = await queries.author.full_author()
//     assert isinstance(full_author[0], FullAuthorRow)
//     assert isinstance(full_author[0].author, Author)
//     assert isinstance(full_author[0].books, list)
//     assert isinstance(full_author[0].books[0], Book)
