fn main() {
    let file_contents = std::fs::read_to_string("rune-like.stuffed").unwrap();
    let structures = stuffed_parser::compile_structures_from_string(file_contents);

    println!("{:#?}", structures);
}
