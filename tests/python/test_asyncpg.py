import os
from datetime import datetime
from uuid import UUID, uuid4

import faker
import pytest
import pytest_asyncio
from out_asyncpg import Queries
from out_asyncpg.author import FullAuthorRow
from out_asyncpg.models import Author, Book

fake = faker.Faker()


@pytest_asyncio.fixture
async def queries(asyncpg_conn):
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
    return queries


@pytest.mark.asyncio
async def test_one_row_one_column(queries: Queries):
    author = await queries.author.one_row_one_column()
    assert isinstance(author, Author)


@pytest.mark.asyncio
async def test_one_row_one_column(queries: Queries):
    data = await queries.author.one_row_many_coulmns()

    assert isinstance(data.author, Author)
    assert isinstance(data.book, Book)
    assert data.one == 1


@pytest.mark.asyncio
async def test_many_rows_one_column(queries: Queries):
    rows = await queries.author.many_rows_one_column()

    assert isinstance(rows[0], Author)


@pytest.mark.asyncio
async def test_many_rows_many_columns(queries: Queries):
    rows = await queries.author.many_rows_many_coulmns()

    assert isinstance(rows[0].author, Author)
    assert isinstance(rows[0].book, Book)
    assert rows[0].one == 1


@pytest.mark.asyncio
async def test_required_parameter(queries: Queries):
    assert "foo" == await queries.author.required_parameter("foo")


@pytest.mark.asyncio
async def test_optional_parameter(queries: Queries):
    assert (await queries.author.optional_parameter(None)) is None
    assert (await queries.author.optional_parameter("foo")) == "foo"


@pytest.mark.asyncio
async def test_query_view(queries: Queries):
    full_author = await queries.author.full_author()
    assert isinstance(full_author[0], FullAuthorRow)
    assert isinstance(full_author[0].author, Author)
    assert isinstance(full_author[0].books, list)
    assert isinstance(full_author[0].books[0], Book)
