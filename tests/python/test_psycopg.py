import os
from datetime import datetime
from uuid import UUID, uuid4

import faker
import pytest
from out_psycopg import Queries, init_connection
from out_psycopg.author import FullAuthorRow
from out_psycopg.models import Author, Book

fake = faker.Faker()


@pytest.fixture(scope="module")
def queries(psycopg_conn) -> Queries:
    init_connection(psycopg_conn)
    queries = Queries(psycopg_conn)

    for _ in range(2):
        author = queries.author.insert(
            Author(
                id=uuid4(),
                full_name=fake.name(),
                birthday=fake.date_of_birth(),
            )
        )
        queries.book.insert(
            Book(
                id=uuid4(),
                title=fake.text(),
                author_id=author.id,
                year=fake.year(),
                isbn=fake.isbn10(),
                is_best_seller=fake.boolean(),
                genre="science fiction",
            )
        )

    return queries


def test_one_row_one_column(queries: Queries):
    author = queries.author.one_row_one_column()
    assert isinstance(author, Author)


def test_one_row_one_column(queries: Queries):
    data = queries.author.one_row_many_coulmns()

    assert isinstance(data.author, Author)
    assert isinstance(data.book, Book)
    assert data.one == 1


def test_many_rows_one_column(queries: Queries):
    rows = queries.author.many_rows_one_column()

    assert isinstance(rows[0], Author)


def test_many_rows_many_columns(queries: Queries):
    rows = queries.author.many_rows_many_coulmns()

    assert isinstance(rows[0].author, Author)
    assert isinstance(rows[0].book, Book)
    assert rows[0].one == 1


def test_required_parameter(queries: Queries):
    assert "foo" == queries.author.required_parameter("foo")


def test_optional_parameter(queries: Queries):
    assert queries.author.optional_parameter(None) is None
    assert queries.author.optional_parameter("foo") == "foo"


def test_query_view(queries: Queries):
    full_author = queries.author.full_author()
    assert isinstance(full_author[0], FullAuthorRow)
    assert isinstance(full_author[0].author, Author)
    assert isinstance(full_author[0].books, list)
    assert isinstance(full_author[0].books[0], Book)
