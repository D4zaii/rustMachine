use crate::inst::Inst;

const MAX_STACK_SIZE: usize = 1024;

pub struct Machine {
    stack: Vec<i32>,
    pub instructions: Vec<Inst>,
}

impl Machine {
    pub fn new(instructions: Vec<Inst>) -> Machine {
        Machine {
            stack: Vec::new(),
            instructions: instructions,
        }
    }

    pub fn push(&mut self, value: i32) {
        if self.stack.len() >= MAX_STACK_SIZE {
            panic!("Error: Stack Overflow!");
        }

        self.stack.push(value);
    }

    pub fn pop(&mut self) -> i32 {
        match self.stack.pop() {
            Some(value) => value,
            None => panic!("Error: Stack Underflow!"),
        }
    }

    pub fn index_dup(&mut self, index: i32) {
        let index = index as usize;
        if index >= self.stack.len() {
            panic!("Error: Fuera del index.")
        }

        self.push(self.stack[index]);
    }

    pub fn index_swap(&mut self, index: i32) {
        let index = index as usize;
        if index >= self.stack.len() {
            panic!("Error: Fuera del index.")
        }

        let aux = self.stack[index];
        self.stack[index] = self.pop();
        self.push(aux);
    }

    pub fn print_stack(&self) {
        for value in self.stack.iter().rev() {
            println!("{}", *value);
        }
    }
}
