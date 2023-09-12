namespace rs volo.example

struct Item {
    1: required string key,
    2: optional string value,
}

struct KeyRequest {
    1: required string key,
}

struct ItemRequest {
    1: required Item item,
}

struct ItemResponse {
    1: required Item item,
}

service ItemService {
    ItemResponse Get (1: KeyRequest req),
    ItemResponse Set (1: ItemRequest req),
    ItemResponse Del (1: KeyRequest req),
    ItemResponse Ping (1: ItemRequest req),
}