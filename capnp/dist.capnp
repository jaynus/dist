@0xb312981b2552afff;

struct Id {
    id @0 : UInt64;
}

struct EntityData {
    id @0 : Id;
    components @1: List(Id);
}

struct Vector {
    x @0 : UInt64;
    y @1 : UInt64;
    z @2 : UInt64;
}
struct Box {
    origin @0 : Vector;
    extent @1 : UInt64;
}

interface Entity {
    getId @0 () -> (id: Id);
    getComponents @1 () -> (components: List(Id));
}

interface Component {}

