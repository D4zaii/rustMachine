mod inst;
mod machine;
mod tasmlexer;

use inst::Inst;
use machine::Machine;

use std::env::args;
use std::fs::write;
use std::process::exit;

fn write_program_to_file(instructions: &Vec<Inst>, file_path: &str) {
    let mut buffer: Vec<u8> = Vec::new();
    for inst in instructions {
        buffer.extend(inst.to_bytes());
    }

    write(file_path, buffer).expect("Error: No se pudo escribir el archivo.");
}

fn read_program_from_file(file_path: &str) -> Vec<Inst> {
    let bytes = std::fs::read(file_path).expect("Error: No se pudo abrir el archivo.");

    let mut instructions: Vec<Inst> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let (inst, size) = Inst::from_bytes(&bytes[i..]);
        instructions.push(inst);
        i += size;
    }
    instructions
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("[!] Error.\n\tUsage: {} <file_name.tasm>", args[0]);
        exit(1);
    }

    let file_name = &args[1];

    let tokens = tasmlexer::lexer(file_name);
    let program = tasmlexer::generate_instructions(&tokens);
    println!("{:?}", tokens);

    let mut ip: usize = 0;

    // let program: Vec<Inst> = vec![Inst::Push(10), Inst::Push(5), Inst::Add];

    /*
    let program: Vec<Inst> = vec![
        Inst::Push(3),  // 0
        Inst::Dup,      // 1  <- acá vuelve el salto
        Inst::Print,    // 2
        Inst::Push(1),  // 3
        Inst::Sub,      // 4
        Inst::Dup,      // 5
        Inst::Nzjmp(1), // 6  <- si no es 0, saltar a la posición 1
        Inst::Pop,      // 7
        Inst::Halt,     // 8
    ];
    */

    write_program_to_file(&program, "program.test");

    // let mut machine = Machine::new(program);

    let loaded_instructions = read_program_from_file("program.test");
    let mut machine = Machine::new(loaded_instructions);

    let instructions = machine.instructions.clone();
    while ip < instructions.len() {
        match &instructions[ip] {
            Inst::Nop => {}
            Inst::Push(value) => {
                machine.push(*value);
            }
            Inst::Pop => {
                machine.pop();
            }
            Inst::Add => {
                let a = machine.pop();
                let b = machine.pop();
                machine.push(a + b);
            }
            Inst::Sub => {
                let a = machine.pop();
                let b = machine.pop();
                machine.push(b - a);
            }
            Inst::Mul => {
                let a = machine.pop();
                let b = machine.pop();
                machine.push(a * b);
            }
            Inst::Div => {
                let a = machine.pop();
                let b = machine.pop();
                if a == 0 {
                    panic!("Error: No es posible dividir por 0.");
                }
                machine.push(b / a);
            }
            Inst::Mod => {
                let a = machine.pop();
                let b = machine.pop();
                machine.push(a % b);
            }
            Inst::Print => {
                let a = machine.pop();
                println!("{}", a);
            }
            Inst::Dup => {
                let a = machine.pop();
                machine.push(a);
                machine.push(a);
            }
            Inst::Swap => {
                let a = machine.pop();
                let b = machine.pop();
                machine.push(a);
                machine.push(b);
            }
            Inst::Cmpe => {
                let a = machine.pop();
                let b = machine.pop();
                machine.push(b);
                machine.push(a);
                machine.push(if a == b { 0 } else { 1 });
            }
            Inst::Cmpne => {
                let a = machine.pop();
                let b = machine.pop();
                machine.push(b);
                machine.push(a);
                machine.push(if a != b { 0 } else { 1 });
            }
            Inst::Cmpg => {
                let a = machine.pop();
                let b = machine.pop();
                machine.push(b);
                machine.push(a);
                machine.push(if a > b { 0 } else { 1 });
            }
            Inst::Cmpl => {
                let a = machine.pop();
                let b = machine.pop();
                machine.push(b);
                machine.push(a);
                machine.push(if a < b { 0 } else { 1 });
            }
            Inst::Cmpge => {
                let a = machine.pop();
                let b = machine.pop();
                machine.push(b);
                machine.push(a);
                machine.push(if a >= b { 0 } else { 1 });
            }
            Inst::Cmple => {
                let a = machine.pop();
                let b = machine.pop();
                machine.push(b);
                machine.push(a);
                machine.push(if a <= b { 0 } else { 1 });
            }
            Inst::Jmp(target) => {
                ip = *target as usize;
                if ip >= instructions.len() {
                    panic!("Error: No se puede saltar fuera de los limites.");
                }
                continue;
            }
            Inst::Zjmp(target) => {
                if machine.pop() == 0 {
                    ip = *target as usize;
                    if ip >= instructions.len() {
                        panic!("Error: No se puede saltar fuera de los limites.")
                    }
                    continue;
                }
            }
            Inst::Nzjmp(target) => {
                if machine.pop() != 0 {
                    ip = *target as usize;
                    if ip >= instructions.len() {
                        panic!("Error: No se puede saltar fuera de los limites.")
                    }
                    continue;
                }
            }
            Inst::Halt => {
                break;
            }
            Inst::Indup(value) => {
                machine.index_dup(*value);
            }
            Inst::Inswap(value) => {
                machine.index_swap(*value);
            }
        }

        ip += 1;
    }

    machine.print_stack();
}
