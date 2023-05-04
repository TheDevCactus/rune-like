struct _MoveTo {
    X: u8,
    Y: u8,
}

enum _MessageType {
    Connect = 0,
    Disconnect = 1,
    MoveTo = 2,
    InteractWith = 3,
}

enum _Action {
    Examine = 0,
    TalkTo = 1,
    Attack = 2,
    Use = 3,
    Drop = 4,
}

struct _InteractWith {
    entity_id: i32,
    action: _Action,
}

struct Stuffed {
    message_type: _MessageType,
}

fn main() {
    let file_contents = std::fs::read_to_string("rune-like.stuffed").unwrap();
    let structures = stuffed_parser::compile_structures_from_string(file_contents);

    for structure in structures {
        println!("{:?}", structure);
    }
}
