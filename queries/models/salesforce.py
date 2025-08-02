import dataclasses
import datetime

@dataclasses.dataclass
class LastSync:
    id: int
    datetime: datetime.datetime | None

@dataclasses.dataclass
class PostAdapter:
    post_id: str
    salesforce_id: str
    sha1: bytes | None

@dataclasses.dataclass
class StorageAdapter:
    storage_id: str
    url: str