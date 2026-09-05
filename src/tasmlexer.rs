use std::fs::read_to_string;

use crate::inst::Inst;

#[derive(Debug)]
pub enum TokenType {
    Nop,
    Push,
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Dup,
    Indup,
    Swap,
    Inswap,
    Cmpe,
    Cmpne,
    Cmpg,
    Cmpl,
    Cmpge,
    Cmple,
    Jmp,
    Zjmp,
    Nzjmp,
    Print,
    Halt,
    Number(i32),
    LabelDef(String),
    Label(String),
    Char(char),
    Str(String),
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    text: String,
    line: usize,
    character: usize,
}

pub fn check_builtin_keywords(name: &str) -> Option<TokenType> {
    match name {
        "nop" => Some(TokenType::Nop),
        "push" => Some(TokenType::Push),
        "pop" => Some(TokenType::Pop),
        "add" => Some(TokenType::Add),
        "sub" => Some(TokenType::Sub),
        "mul" => Some(TokenType::Mul),
        "div" => Some(TokenType::Div),
        "mod" => Some(TokenType::Mod),
        "dup" => Some(TokenType::Dup),
        "indup" => Some(TokenType::Indup),
        "swap" => Some(TokenType::Swap),
        "inswap" => Some(TokenType::Inswap),
        "cmpe" => Some(TokenType::Cmpe),
        "cmpne" => Some(TokenType::Cmpne),
        "cmpg" => Some(TokenType::Cmpg),
        "cmpl" => Some(TokenType::Cmpl),
        "cmpge" => Some(TokenType::Cmpge),
        "cmple" => Some(TokenType::Cmple),
        "jmp" => Some(TokenType::Jmp),
        "zjmp" => Some(TokenType::Zjmp),
        "nzjmp" => Some(TokenType::Nzjmp),
        "print" => Some(TokenType::Print),
        "halt" => Some(TokenType::Halt),
        _ => None,
    }
}

pub fn open_file(file_path: &str) -> Vec<char> {
    let content = read_to_string(file_path).expect("Error: No se pudo abrir el archivo.");
    content.chars().collect()
}

fn generate_keyword(chars: &Vec<char>, i: &mut usize) -> Token {
    let mut text = String::new();

    while chars[*i].is_alphabetic() {
        text.push(chars[*i]);
        *i += 1;
    }

    let token_type = match check_builtin_keywords(&text) {
        Some(t) => t,
        None => check_label_type(chars, i, text.clone()),
    };

    Token {
        token_type,
        text,
        line: 0,
        character: 0,
    }
}

fn generate_number(chars: &Vec<char>, i: &mut usize) -> Token {
    let mut text = String::new();

    while chars[*i].is_numeric() {
        text.push(chars[*i]);
        *i += 1;
    }

    let value: i32 = text.parse().expect("Error: No se pudo parsear el número.");

    Token {
        token_type: TokenType::Number(value),
        text,
        line: 0,
        character: 0,
    }
}

fn generate_char(chars: &Vec<char>, i: &mut usize) -> Token {
    *i += 1;

    let mut c = chars[*i];
    if c == '\\' {
        *i += 1;
        c = valid_escape_character(chars[*i]).expect("Error: Carácter de escape inválido.");
    }
    *i += 1;

    if chars[*i] != '\'' {
        panic!("Error: Se esperaba una comilla simple de cierre.");
    }

    *i += 1;

    Token {
        token_type: TokenType::Char(c),
        text: c.to_string(),
        line: 0,
        character: 0,
    }
}

fn generate_string(chars: &Vec<char>, i: &mut usize) -> Token {
    *i += 1;
    let mut text = String::new();

    while *i < chars.len() && chars[*i] != '"' {
        let mut c = chars[*i];
        if c == '\\' {
            *i += 1;
            c = valid_escape_character(chars[*i]).expect("Error: Carácter de escape inválido.");
        }
        text.push(c);
        *i += 1;
    }

    if *i >= chars.len() {
        panic!("Error: Se esperaba una comilla doble de cierre.");
    }

    *i += 1;

    Token {
        token_type: TokenType::Str(text.clone()),
        text,
        line: 0,
        character: 0,
    }
}

fn check_label_type(chars: &Vec<char>, i: &usize, name: String) -> TokenType {
    if chars[*i] == ':' {
        TokenType::LabelDef(name)
    } else {
        TokenType::Label(name)
    }
}

fn skip_comment(chars: &Vec<char>, i: &mut usize) {
    while *i < chars.len() && chars[*i] != '\n' {
        *i += 1;
    }
}

fn valid_escape_character(c: char) -> Option<char> {
    match c {
        'n' => Some('\n'),
        't' => Some('\t'),
        'r' => Some('\r'),
        '0' => Some('\0'),
        '\\' => Some('\\'),
        '\'' => Some('\''),
        '"' => Some('"'),
        _ => None,
    }
}

pub fn generate_instructions(tokens: &Vec<Token>) -> Vec<Inst> {
    let mut instructions: Vec<Inst> = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i].token_type {
            TokenType::Nop => instructions.push(Inst::Nop),
            TokenType::Push => {
                i += 1;
                match &tokens[i].token_type {
                    TokenType::Number(value) => {
                        instructions.push(Inst::Push(*value));
                    }
                    TokenType::Char(value) => {
                        instructions.push(Inst::Push(*value as i32));
                    }
                    TokenType::Str(value) => {
                        for c in value.chars() {
                            instructions.push(Inst::Push(c as i32));
                        }
                    }
                    _ => panic!(
                        "Error: Se esperaba un número, carácter o string después de 'push' en la posición {}",
                        i
                    ),
                }
            }
            TokenType::Pop => instructions.push(Inst::Pop),
            TokenType::Dup => instructions.push(Inst::Dup),
            TokenType::Indup => {
                i += 1;
                let value = expect_number(tokens, i);
                instructions.push(Inst::Indup(value));
            }
            TokenType::Swap => instructions.push(Inst::Swap),
            TokenType::Inswap => {
                i += 1;
                let value = expect_number(tokens, i);
                instructions.push(Inst::Inswap(value));
            }
            TokenType::Add => instructions.push(Inst::Add),
            TokenType::Sub => instructions.push(Inst::Sub),
            TokenType::Mul => instructions.push(Inst::Mul),
            TokenType::Div => instructions.push(Inst::Div),
            TokenType::Mod => instructions.push(Inst::Mod),
            TokenType::Cmpe => instructions.push(Inst::Cmpe),
            TokenType::Cmpne => instructions.push(Inst::Cmpne),
            TokenType::Cmpg => instructions.push(Inst::Cmpg),
            TokenType::Cmpl => instructions.push(Inst::Cmpl),
            TokenType::Cmpge => instructions.push(Inst::Cmpge),
            TokenType::Cmple => instructions.push(Inst::Cmple),
            TokenType::Jmp => {
                i += 1;
                let value = expect_number(tokens, i);
                instructions.push(Inst::Jmp(value));
            }
            TokenType::Zjmp => {
                i += 1;
                let value = expect_number(tokens, i);
                instructions.push(Inst::Zjmp(value));
            }
            TokenType::Nzjmp => {
                i += 1;
                let value = expect_number(tokens, i);
                instructions.push(Inst::Nzjmp(value));
            }
            TokenType::Print => instructions.push(Inst::Print),
            TokenType::Halt => instructions.push(Inst::Halt),
            TokenType::Number(_) => {
                panic!("Error: Number suelto detectado, se esperaba un 'push' antes.");
            }
            TokenType::LabelDef(_) => {
                panic!("Error: Las definiciones de etiquetas no están implementadas todavía.");
            }
            TokenType::Label(_) => {
                panic!("Error: Las definiciones de etiquetas no están implementadas todavía.");
            }
            TokenType::Char(_) => {
                panic!("Error: Carácter suelto detectado, se esperaba un 'push' antes.");
            }
            TokenType::Str(_) => {
                panic!("Error: String suelto detectado, se esperaba un 'push' antes.");
            }
        }
        i += 1;
    }

    instructions
}

fn expect_number(tokens: &Vec<Token>, i: usize) -> i32 {
    if i >= tokens.len() {
        panic!("Error: Se esperaba un número, pero el archivo terminó antes.");
    }
    match tokens[i].token_type {
        TokenType::Number(value) => value,
        TokenType::Char(c) => c as i32,
        _ => panic!(
            "Error: Se esperaba un número en la posición {}, pero se encontró otro token.",
            i
        ),
    }
}

pub fn lexer(file_name: &str) -> Vec<Token> {
    let chars = open_file(file_name);
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_alphabetic() {
            tokens.push(generate_keyword(&chars, &mut i));
        } else if chars[i].is_numeric() {
            tokens.push(generate_number(&chars, &mut i));
        } else if chars[i] == '\'' {
            tokens.push(generate_char(&chars, &mut i));
        } else if chars[i] == '"' {
            tokens.push(generate_string(&chars, &mut i));
        } else if chars[i] == '#' {
            skip_comment(&chars, &mut i);
        } else {
            i += 1;
        }
    }

    tokens
}
