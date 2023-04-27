struct _MoveTo {
    X: i32,
    Y: i32,
}

enum _MessageType {
    Connect,
    Disconnect,
    MoveTo(_MoveTo),
    InteractWith
}

enum _Action {
    Examine,
    TalkTo,
    Attack,
    Use,
    Drop,
}

struct _InteractWith {
    entity_id: i32,
    action: _Action, 
}

struct Stuffed {
   message_type: _MessageType,
}

pub fn parse_bytes_into_stuffed_structure

fn main() {
    let file_contents = std::fs::read_to_string("rune-like.stuffed").unwrap();
    let structures = stuffed_parser::compile_structures_from_string(file_contents);

    for structure in structures {
        println!("{:?}", structure);
    }
}
