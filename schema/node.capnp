@0xe39fd070ba1f2227;

interface Node {
    struct InsertRequest {
        name @0 :Text;
    }

    struct InsertReply {
        message @0 :Text;
    }

    insert @0 (request: InsertRequest) -> (reply: InsertReply);
}