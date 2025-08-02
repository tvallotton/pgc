import dataclasses
import datetime

@dataclasses.dataclass
class Details:
    id: str
    has_party_room: bool | None
    has_attic: bool | None
    has_gym: bool | None
    has_tap_water: bool | None
    has_electric_gate_opener: bool | None
    has_telephone_line: bool | None
    has_guest_parking: bool | None
    has_dining_room: bool | None
    has_paddle_court: bool | None
    has_swimming_pool: bool | None
    has_alarm: bool | None
    has_bedroom_suite: bool | None
    has_cable_tv: bool | None
    has_jacuzzi: bool | None
    has_laundry: bool | None
    has_boiler: bool | None
    has_basketball_court: bool | None
    has_kitchen: bool | None
    has_security: bool | None
    has_study: bool | None
    has_tennis_court: bool | None
    has_dressing_room: bool | None
    has_sauna: bool | None
    has_terrace: bool | None
    has_grill: bool | None
    has_balcony: bool | None
    has_controlled_access: bool | None
    has_fridge: bool | None
    has_patio: bool | None
    has_playground: bool | None
    has_living_room: bool | None
    has_half_bathroom: bool | None
    has_garden: bool | None
    has_internet_access: bool | None
    has_heating: bool | None
    has_indoor_fireplace: bool | None
    has_electric_generator: bool | None
    has_maid_room: bool | None
    has_lift: bool | None
    has_cistern: bool | None
    has_playroom: bool | None
    has_air_conditioning: bool | None
    has_natural_gas: bool | None
    has_cinema_hall: bool | None
    has_closet: bool | None
    has_elevator: bool | None

@dataclasses.dataclass
class Draft:
    id: str
    data: dict
    user_id: str

@dataclasses.dataclass
class DraftImage:
    id: str
    storage_id: str
    ordering: int
    draft_id: str

@dataclasses.dataclass
class Status:
    id: str

@dataclasses.dataclass
class View:
    id: str
    created_at: datetime.datetime
    user_id: str | None
    post_id: str | None