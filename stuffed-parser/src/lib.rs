const ENTRY: &str = "ENTRY";
const PRECEDES: &str = "precedes";
const ONE_OF: &str = "one_of";
const COLON: &str = ":";
const SEMICOLON: &str = ";";
const COMMA: &str = ",";
const ROLL_OUT: &str = ">";
const ARRAY: &str = "[]";

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Entry,
    Ident(String),
    Value(u64),
    ContextRequired(String),
    OneOf,
    Precedes,
    Colon,
    Semicolon,
    Comma,
    RollOut,
    Array,
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        match s.as_str() {
            ENTRY => Token::Entry,
            ONE_OF => Token::OneOf,
            PRECEDES => Token::Precedes,
            COLON => Token::Colon,
            SEMICOLON => Token::Semicolon,
            COMMA => Token::Comma,
            ROLL_OUT => Token::RollOut,
            ARRAY => Token::Array,
            _ => Token::ContextRequired(s),
        }
    }
}

impl Into<String> for Token {
    fn into(self) -> String {
        match self {
            Token::Entry => ENTRY.to_string(),
            Token::RollOut => ROLL_OUT.to_string(),
            Token::OneOf => ONE_OF.to_string(),
            Token::Precedes => PRECEDES.to_string(),
            Token::Colon => COLON.to_string(),
            Token::Comma => COMMA.to_string(),
            Token::Semicolon => SEMICOLON.to_string(),
            Token::Array => ARRAY.to_string(),
            Token::ContextRequired(s) => s,
            Self::Ident(s) => s,
            Self::Value(n) => n.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Property {
    Precedes(String, String),
    OneOf(String),
}

#[derive(Debug, Clone)]
pub struct Structure {
    name: String,
    nodes: Vec<Property>,
    array_indexes: Vec<bool>,
}

impl Structure {
    fn new() -> Self {
        Self {
            name: String::new(),
            nodes: Vec::new(),
            array_indexes: Vec::new(),
        }
    }
}

fn is_terminating_char(c: char) -> bool {
    return c == ':' || c == ';' || c == ',' || c == '>';
}

pub fn generate_raw_symbols(contents: String) -> Vec<String> {
    let mut tokens = Vec::new();
    for line in contents.lines() {
        let mut token = String::new();

        for c in line.chars() {
            if c.is_whitespace() {
                if !token.is_empty() {
                    tokens.push(token);
                    token = String::new();
                }
                continue;
            }

            if is_terminating_char(c) {
                if !token.is_empty() {
                    tokens.push(token);
                    token = String::new();
                }
                tokens.push(c.to_string());
                continue;
            }

            token.push(c);
        }

        if !token.is_empty() {
            tokens.push(token);
        }
    }

    return tokens;
}

pub fn generate_structures_from_tokens(tokens: &mut impl Iterator<Item = Token>) -> Vec<Structure> {
    let mut prev_token = tokens.next().expect("Empty program");
    if prev_token != Token::Entry {
        panic!("Expected 'ENTRY' token, got {:?}", prev_token);
    }

    let final_tokens: Vec<Token> = tokens
        .map(|x| {
            let mut out: Token = x;
            if let Token::ContextRequired(s) = out {
                out = match prev_token {
                    Token::Ident(_) => Token::Value(s.parse().unwrap()),
                    _ => Token::Ident(s),
                }
            }

            prev_token = out.clone();
            return out;
        })
        .collect();

    let mut structures: Vec<Structure> = Vec::new();
    let mut current_structure = Structure::new();
    let mut current_property_type = Property::OneOf(String::new());
    let mut current_property = Token::Ident(String::new());

    for token in final_tokens {
        match token {
            Token::Entry => {
                current_structure = Structure::new();
            }
            Token::Ident(s) => {
                if current_structure.name.is_empty() {
                    current_structure.name = s;
                    continue;
                }

                match current_property_type {
                    Property::Precedes(_, _) => current_property = Token::Ident(s),
                    Property::OneOf(_) => current_structure.nodes.push(Property::OneOf(s)),
                };
            }
            Token::RollOut => {
                current_structure.nodes.push(Property::Precedes(
                    current_property.clone().into(),
                    ROLL_OUT.to_string(),
                ));
            }
            Token::Value(n) => {
                if let Property::Precedes(_, _) = current_property_type {
                    current_structure.nodes.push(Property::Precedes(
                        current_property.clone().into(),
                        n.to_string(),
                    ));
                    continue;
                }

                panic!("Unexpected 'BYTE_COUNT' token");
            }
            Token::OneOf => {
                current_property_type = Property::OneOf(String::new());
            }
            Token::Precedes => {
                current_property_type = Property::Precedes(String::new(), 0.to_string());
            }
            Token::Array => {
                current_structure.array_indexes.push(true);
            }
            Token::Comma => {
                if current_structure.array_indexes.len() < current_structure.nodes.len() {
                    current_structure.array_indexes.push(false);
                }
            }
            Token::Semicolon => {
                if current_structure.array_indexes.len() < current_structure.nodes.len() {
                    current_structure.array_indexes.push(false);
                }
                structures.push(current_structure);
                current_structure = Structure::new();
            }
            _ => {}
        };
    }
    return structures;
}

pub fn compile_structures_from_string(contents: String) -> Vec<Structure> {
    let symbols = generate_raw_symbols(contents);
    let mut tokens = symbols.iter().map(|s| Token::from(s.to_string()));
    return generate_structures_from_tokens(&mut tokens);
}
