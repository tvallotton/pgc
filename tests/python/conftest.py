import os
from typing import AsyncIterator

import asyncpg
import psycopg
import pytest
import pytest_asyncio
from psycopg.sql import Iterator, LiteralString
from testcontainers.postgres import PostgresContainer


@pytest.fixture(scope="session")
def schema():

    with open("tests/schema.sql") as f:
        schema = f.read()
    return schema


@pytest.fixture(scope="module")
def pg_container() -> Iterator[PostgresContainer]:
    """
    Session-scoped PostgreSQL testcontainer.
    Starts once for the whole test session and tears down at the end.
    """
    container = PostgresContainer("postgres:16-alpine")
    container.start()
    try:
        yield container
    finally:
        container.stop()


@pytest_asyncio.fixture(scope="function")
async def asyncpg_conn(
    pg_container: PostgresContainer, schema: str
) -> AsyncIterator[asyncpg.Connection]:
    dsn = pg_container.get_connection_url().replace(
        "postgresql+psycopg2://", "postgresql://"
    )

    conn = await asyncpg.connect(dsn=dsn)
    try:
        await conn.execute(schema)
        yield conn
    finally:
        await conn.close()


@pytest.fixture(scope="module")
def psycopg_conn(
    pg_container: PostgresContainer, schema: LiteralString
) -> Iterator[psycopg.Connection]:
    dsn = pg_container.get_connection_url().replace(
        "postgresql+psycopg2://", "postgresql://"
    )
    conn = psycopg.connect(dsn)
    try:
        conn.execute(schema)
        conn.commit()
        yield conn
    finally:
        conn.close()
