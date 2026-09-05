#[derive(Clone)]
pub enum Inst {
    Nop,
    Push(i32),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Dup,
    Indup(i32),
    Swap,
    Inswap(i32),
    Cmpe,
    Cmpne,
    Cmpg,
    Cmpl,
    Cmpge,
    Cmple,
    Jmp(i32),
    Zjmp(i32),
    Nzjmp(i32),
    Print,
    Halt,
}

impl Inst {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Inst::Nop => {
                vec![0u8]
            }
            Inst::Push(value) => {
                let mut bytes = vec![1u8];
                bytes.extend(value.to_le_bytes());
                bytes
            }
            Inst::Pop => {
                vec![2u8]
            }
            Inst::Add => {
                vec![3u8]
            }
            Inst::Sub => {
                vec![4u8]
            }
            Inst::Mul => {
                vec![5u8]
            }
            Inst::Div => {
                vec![6u8]
            }
            Inst::Mod => {
                vec![7u8]
            }
            Inst::Dup => {
                vec![8u8]
            }
            Inst::Indup(value) => {
                let mut bytes = vec![9u8];
                bytes.extend(value.to_le_bytes());
                bytes
            }
            Inst::Swap => {
                vec![10u8]
            }
            Inst::Inswap(value) => {
                let mut bytes = vec![11u8];
                bytes.extend(value.to_le_bytes());
                bytes
            }
            Inst::Cmpe => {
                vec![12u8]
            }
            Inst::Cmpne => {
                vec![13u8]
            }
            Inst::Cmpg => {
                vec![14u8]
            }
            Inst::Cmpl => {
                vec![15u8]
            }
            Inst::Cmpge => {
                vec![16u8]
            }
            Inst::Cmple => {
                vec![17u8]
            }
            Inst::Jmp(value) => {
                let mut bytes = vec![18u8];
                bytes.extend(value.to_le_bytes());
                bytes
            }
            Inst::Zjmp(value) => {
                let mut bytes = vec![19u8];
                bytes.extend(value.to_le_bytes());
                bytes
            }
            Inst::Nzjmp(value) => {
                let mut bytes = vec![20u8];
                bytes.extend(value.to_le_bytes());
                bytes
            }
            Inst::Print => {
                vec![21u8]
            }
            Inst::Halt => {
                vec![22u8]
            }
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> (Inst, usize) {
        let tag = bytes[0];
        match tag {
            0 => (Inst::Nop, 1),
            1 => {
                let value_bytes: [u8; 4] = bytes[1..5].try_into().unwrap();
                let value = i32::from_le_bytes(value_bytes);
                (Inst::Push(value), 5)
            }
            2 => (Inst::Pop, 1),
            3 => (Inst::Add, 1),
            4 => (Inst::Sub, 1),
            5 => (Inst::Mul, 1),
            6 => (Inst::Div, 1),
            7 => (Inst::Mod, 1),
            8 => (Inst::Dup, 1),
            9 => {
                let value_bytes: [u8; 4] = bytes[1..5].try_into().unwrap();
                let value = i32::from_le_bytes(value_bytes);
                (Inst::Indup(value), 5)
            }
            10 => (Inst::Swap, 1),
            11 => {
                let value_bytes: [u8; 4] = bytes[1..5].try_into().unwrap();
                let value = i32::from_le_bytes(value_bytes);
                (Inst::Inswap(value), 5)
            }
            12 => (Inst::Cmpe, 1),
            13 => (Inst::Cmpne, 1),
            14 => (Inst::Cmpg, 1),
            15 => (Inst::Cmpl, 1),
            16 => (Inst::Cmpge, 1),
            17 => (Inst::Cmple, 1),
            18 => {
                let value_bytes: [u8; 4] = bytes[1..5].try_into().unwrap();
                let value = i32::from_le_bytes(value_bytes);
                (Inst::Jmp(value), 5)
            }
            19 => {
                let value_bytes: [u8; 4] = bytes[1..5].try_into().unwrap();
                let value = i32::from_le_bytes(value_bytes);
                (Inst::Zjmp(value), 5)
            }
            20 => {
                let value_bytes: [u8; 4] = bytes[1..5].try_into().unwrap();
                let value = i32::from_le_bytes(value_bytes);
                (Inst::Nzjmp(value), 5)
            }
            21 => (Inst::Print, 1),
            22 => (Inst::Halt, 1),
            _ => panic!("Error: Tag de instrucción desconocido: {}", tag),
        }
    }
}
