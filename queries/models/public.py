import dataclasses
import asyncpg
import datetime
import decimal
import typing

@dataclasses.dataclass
class SqlxMigrations:
    version: int
    success: bool
    installed_on: datetime.datetime
    execution_time: int
    checksum: bytes
    description: str

@dataclasses.dataclass
class Author:
    id: str
    name: str
    birthday: datetime.date | None

@dataclasses.dataclass
class Book:
    id: str
    title: str
    isbn: str
    is_best_seller: bool | None
    year: int
    author_id: str
    genre: str

@dataclasses.dataclass
class Commune:
    id: str
    province: str
    region: str
    name: str | None
    geometry: typing.Any | None

@dataclasses.dataclass
class Complex:
    imag: float
    real: float

@dataclasses.dataclass
class Currency:
    id: str

@dataclasses.dataclass
class CurrencyPrice:
    id: str
    price: decimal.Decimal

@dataclasses.dataclass
class CurrencyPriceHistory:
    valid_until: datetime.datetime | None
    id: str
    valid_since: datetime.datetime
    price: decimal.Decimal

@dataclasses.dataclass
class Foo:
    a: int | None
    c: str | None
    b: float | None

@dataclasses.dataclass
class Genre:
    id: str

@dataclasses.dataclass
class GeographyColumns:
    srid: int | None
    f_table_name: typing.Any | None
    type: str | None
    f_geography_column: typing.Any | None
    f_table_catalog: typing.Any | None
    f_table_schema: typing.Any | None
    coord_dimension: int | None

@dataclasses.dataclass
class GeometryColumns:
    f_table_schema: typing.Any | None
    coord_dimension: int | None
    srid: int | None
    f_table_catalog: str | None
    f_geometry_column: typing.Any | None
    type: str | None
    f_table_name: typing.Any | None

@dataclasses.dataclass
class GeometryDump:
    geom: typing.Any | None
    path: list[int] | None

@dataclasses.dataclass
class ListingType:
    id: str

@dataclasses.dataclass
class Orientation:
    id: str

@dataclasses.dataclass
class Place:
    id: int
    street: str
    coordinates: asyncpg.types.Point
    number: str | None

@dataclasses.dataclass
class Post:
    id: str
    title: str
    address: str
    condominium: bool
    currency: str
    orientation: str | None
    description: str
    realtor_id: str | None
    coordinates: asyncpg.types.Point
    price: decimal.Decimal
    property_type: str
    warehouses: int
    highlighted: bool | None
    year: int | None
    service_bedroom: bool
    land_area: int
    listing_type: str
    bathrooms: int
    floor: int
    bedrooms: int
    service_bathroom: bool
    parking_lots: int
    author_id: str
    built_area: int
    numeric_id: int
    status: str

@dataclasses.dataclass
class PostImage:
    id: str
    storage_id: str
    ordering: int
    post_id: str

@dataclasses.dataclass
class PropertyType:
    id: str

@dataclasses.dataclass
class Realtor:
    id: str
    first_name: str
    user_id: str | None
    last_name: str
    phone: str

@dataclasses.dataclass
class RealtorHistory:
    phone: str
    valid_since: datetime.datetime
    first_name: str
    user_id: str | None
    id: str
    valid_until: datetime.datetime | None
    last_name: str

@dataclasses.dataclass
class SpatialRefSys:
    srid: int
    auth_srid: int | None
    srtext: str | None
    proj4text: str | None
    auth_name: str | None

@dataclasses.dataclass
class Storage:
    id: str
    bucket: str
    sha1: bytes
    extension: str

@dataclasses.dataclass
class Suggestion:
    number: str | None
    coordinates: asyncpg.types.Point | None
    id: int | None
    address: str | None
    street: str | None

@dataclasses.dataclass
class User:
    id: str
    role: str
    email: str

@dataclasses.dataclass
class UserHistory:
    id: str
    email: str
    valid_since: datetime.datetime
    role: str
    valid_until: datetime.datetime | None

@dataclasses.dataclass
class UserRole:
    id: str

@dataclasses.dataclass
class ValidDetail:
    valid: bool | None
    location: typing.Any | None
    reason: str | None

@dataclasses.dataclass
class VisitRequest:
    id: str
    rut: str
    post_id: str | None
    uid: str | None
    availability: list[asyncpg.types.Range[datetime.datetime]]
    comment: str
    email: str
    created_at: datetime.datetime
    full_name: str
    phone: str