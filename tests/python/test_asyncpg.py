import os
from datetime import datetime
from uuid import UUID, uuid4

import faker
import pytest
import pytest_asyncio
from dist_asyncpg import Queries
from dist_asyncpg.models import Author, Book

fake = faker.Faker()


@pytest_asyncio.fixture
async def populate_db(asyncpg_conn):
    queries = Queries(asyncpg_conn)

    for _ in range(2):
        author = await queries.author.insert(
            Author(
                id=uuid4(),
                full_name=fake.name(),
                birthday=fake.date_of_birth(),
            )
        )
        await queries.book.insert(
            Book(
                id=uuid4(),
                title=fake.text(),
                author_id=author.id,
                year=int(fake.year()),
                isbn=fake.isbn10(),
                is_best_seller=fake.boolean(),
                genre="science fiction",
            )
        )


@pytest.mark.asyncio
async def test_one_row_one_column(asyncpg_conn, populate_db):
    queries = Queries(asyncpg_conn)
    author = await queries.author.one_row_one_column()
    assert isinstance(author, Author)


@pytest.mark.asyncio
async def test_one_row_one_column(asyncpg_conn, populate_db):
    queries = Queries(asyncpg_conn)
    data = await queries.author.one_row_many_coulmns()

    assert isinstance(data.author, Author)
    assert isinstance(data.book, Book)
    assert data.one == 1


@pytest.mark.asyncio
async def test_many_rows_one_column(asyncpg_conn, populate_db):
    queries = Queries(asyncpg_conn)
    rows = await queries.author.many_rows_one_column()

    assert isinstance(rows[0], Author)


@pytest.mark.asyncio
async def test_many_rows_many_columns(asyncpg_conn, populate_db):
    queries = Queries(asyncpg_conn)

    rows = await queries.author.many_rows_many_coulmns()

    assert isinstance(rows[0].author, Author)
    assert isinstance(rows[0].book, Book)
    assert rows[0].one == 1


@pytest.mark.asyncio
async def test_required_parameter(asyncpg_conn):
    queries = Queries(asyncpg_conn)

    assert "foo" == await queries.author.required_parameter("foo")


@pytest.mark.asyncio
async def test_optional_parameter(asyncpg_conn):
    queries = Queries(asyncpg_conn)

    assert (await queries.author.optional_parameter(None)) is None
    assert (await queries.author.optional_parameter("foo")) == "foo"


# def test_optional_parameter(asyncpg_conn):
#     pass
