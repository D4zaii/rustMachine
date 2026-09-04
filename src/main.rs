#[warn(unused)]
enum Inst {
    Push(i32),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Print,
}
const MAX_STACK_SIZE: usize = 1024;

fn push(stack: &mut Vec<i32>, value: i32) {
    if stack.len() >= MAX_STACK_SIZE {
        panic!("Error: Stack Overflow!");
    }

    stack.push(value);
}

fn pop(stack: &mut Vec<i32>) -> i32 {
    match stack.pop() {
        Some(valor) => valor,
        None => panic!("Error: Stack Underflow!"),
    }
}

fn print_stack(stack: &Vec<i32>) {
    for value in stack.iter().rev() {
        println!("{}", *value);
    }
}

fn main() {
    let mut stack: Vec<i32> = Vec::new();

    let program: Vec<Inst> = vec![Inst::Push(10), Inst::Push(5), Inst::Add];

    for inst in &program {
        match inst {
            Inst::Push(value) => {
                push(&mut stack, *value);
            }
            Inst::Pop => {
                pop(&mut stack);
            }
            Inst::Add => {
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                push(&mut stack, a + b);
            }
            Inst::Sub => {
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                push(&mut stack, b - a);
            }
            Inst::Mul => {
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                push(&mut stack, a * b);
            }
            Inst::Div => {
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                push(&mut stack, b / a);
            }
            Inst::Print => {
                let a = pop(&mut stack);
                println!("{}", a);
            }
        }
    }

    print_stack(&stack);
}
