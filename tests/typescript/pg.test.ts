import { faker } from "@faker-js/faker";
import { assert, assertEquals, assertExists, assertInstanceOf } from "jsr:@std/assert";
import * as pg from "pg";
import { initClient, Queries } from "./out_pg/queries.ts";

import {
  PostgreSqlContainer,
  StartedPostgreSqlContainer,
} from "@testcontainers/postgresql";
import { randomUUID } from "node:crypto";
import { Genre } from "./out_pg/models/models.ts";

let CONTAINER: StartedPostgreSqlContainer | undefined = undefined;
let queries: Queries;
Deno.test.beforeAll(async () => {
  CONTAINER = await new PostgreSqlContainer("postgres:16-alpine")
    .start();

  const client = new pg.Client({
    connectionString: CONTAINER.getConnectionUri(),
  });

  await client.connect();
  await client.query(await Deno.readTextFile("tests/schema.sql"));
  await initClient(client);
  queries = new Queries(client);

  const author = await queries.author.insert({
    author: { 
      id: randomUUID(),
      birthday: faker.date.birthdate(),
      fullName: faker.book.author(),
    }
  });

  const book = {
      authorId: author!.id,
      year: faker.date.anytime().getFullYear(),
      genre: Genre.SCIENCE_FICTION,
      isBestSeller: false,
      isbn: faker.number.int().toString(),
      title: faker.book.title(),
      id: randomUUID(),
  }
  await queries.book.insert({ book });
  
});

// async function getQueries() {
//   const client = new pg.Client({
//     connectionString: CONTAINER!.getConnectionUri(),
//   });
//   await client.connect();
//   await initClient(client);
//   const queries = new Queries(client);

//   await queries.book.insert({ book });
//   return queries;
// }

Deno.test("test_one_row_one_column", async () => {
  // const queries = await getQueries();
  const author = await queries.author.oneRowOneColumn();

  assert(typeof (author?.fullName) == "string", JSON.stringify(author));
  assertInstanceOf(author.birthday, Date);

  // await queries.client.end();
});

Deno.test("test_one_row_many_columns", async () => {
  // const queries = await getQueries();
  const data = await queries.author.oneRowManyColumns();
  assertInstanceOf(data?.author.birthday, Date);
  assertEquals(data.one, 1);
  // await queries.client.end();
})


Deno.test("test_many_rows_one_column", async () => {
  // const queries = await getQueries();
  const data = await queries.author.manyRowsOneColumn();
  assertInstanceOf(data[0].birthday, Date, `${JSON.stringify(data)}`);
  // await queries.client.end();
})


Deno.test("test_many_rows_many_columns", async () => {
  // const queries = await getQueries();
  const data = await queries.author.manyRowsManyColumns();
  assertInstanceOf(data[0].author.birthday, Date);
  assert(!!data[0].book.id);
  assertEquals(data[0].one, 1);
  // await queries.client.end();
})


Deno.test("test_required_parameter", async () => {
  // const queries = await getQueries();
  const foo = await queries.author.requiredParameter({ val: "foo" });
  assertEquals(foo, "foo");
  // await queries.client.end();
})



Deno.test("test_optional_parameter", async () => {
  // const queries = await getQueries();
  const nullValue = await queries.author.optionalParameter({});
  assertEquals(nullValue, null);
  // await queries.client.end();
})


Deno.test("test_query_view", async () => {
  // const queries = await getQueries();
  const fullAuthor = await queries.author.fullAuthor();
  assertExists(fullAuthor[0].author);
  assertInstanceOf(fullAuthor[0].books, Array, `${fullAuthor[0].books}`);
  assertExists(fullAuthor[0].books[0].id);
  // await queries.client.end();
});







Deno.test.afterAll(async () => {
  await queries.client.end();
  await CONTAINER?.stop();

});
