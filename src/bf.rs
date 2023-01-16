use crate::io::BrainfuckIo;
use std::collections::HashMap;

enum BrainfuckInstruction {
    IncrValue,
    DecrValue,
    IncrPointer,
    DecrPointer,
    OutputChar,
    InputChar,
    JzFront,
    JnzBack,
}

fn parse_brackets(program: &[BrainfuckInstruction]) -> Option<HashMap<usize, usize>> {
    let mut brackets_mapping: HashMap<usize, usize> = HashMap::new();
    let mut par_stack: Vec<usize> = Vec::new();
    for (pos, c) in program.iter().enumerate() {
        match c {
            BrainfuckInstruction::JzFront => {
                par_stack.push(pos);
            }
            BrainfuckInstruction::JnzBack => match par_stack.pop() {
                Some(opening_pos) => {
                    let closing_pos = pos;
                    brackets_mapping.insert(opening_pos, closing_pos);
                    brackets_mapping.insert(closing_pos, opening_pos);
                }
                None => return None,
            },
            _ => {}
        }
    }
    Some(brackets_mapping)
}

fn parse(program: &str) -> Vec<BrainfuckInstruction> {
    let mut instructions: Vec<BrainfuckInstruction> = Vec::new();
    for c in program.chars() {
        match c {
            '+' => instructions.push(BrainfuckInstruction::IncrValue),
            '-' => instructions.push(BrainfuckInstruction::DecrValue),
            '>' => instructions.push(BrainfuckInstruction::IncrPointer),
            '<' => instructions.push(BrainfuckInstruction::DecrPointer),
            '[' => instructions.push(BrainfuckInstruction::JzFront),
            ']' => instructions.push(BrainfuckInstruction::JnzBack),
            '.' => instructions.push(BrainfuckInstruction::OutputChar),
            ',' => instructions.push(BrainfuckInstruction::InputChar),
            _ => {
                // ignore everything else
            }
        }
    }
    instructions
}

pub struct BrainfuckVM {
    memory: Vec<u8>, // hardcoded 256 cells
    cell_id: usize,
    ip: usize,
}

impl BrainfuckVM {
    pub fn new() -> BrainfuckVM{
        BrainfuckVM {
            memory: [0; 30_000].into(),
            cell_id: 0usize,
            ip: 0usize
        }
    }
}

pub struct BrainfuckInterpreter {
    vm: BrainfuckVM,
    instructions: Vec<BrainfuckInstruction>,
    brackets_mapping: HashMap<usize, usize>,
}

impl BrainfuckInterpreter {
    pub fn new(program: &str) -> BrainfuckInterpreter {
        let instructions = parse(program);
        let brackets_mapping = parse_brackets(&instructions).unwrap();
        BrainfuckInterpreter {
            vm:BrainfuckVM::new(),
            instructions,
            brackets_mapping,
        }
    }

    pub fn run(&mut self, io: &mut impl BrainfuckIo) {
        loop {
            if self.vm.ip < self.instructions.len() {
                match self.instructions[self.vm.ip] {
                    BrainfuckInstruction::IncrValue => {
                        // + Increment the value of the cell by 1
                        self.vm.memory[self.vm.cell_id] += 1;
                    }
                    BrainfuckInstruction::DecrValue => {
                        // - Decrement the value of the cell by 1
                        self.vm.memory[self.vm.cell_id] -= 1;
                    }
                    BrainfuckInstruction::IncrPointer => {
                        // > Move the pointer to the next cell to the right
                        self.vm.cell_id += 1;
                    }
                    BrainfuckInstruction::DecrPointer => {
                        // < Move the pointer to the next cell to the left
                        self.vm.cell_id -= 1;
                    }
                    BrainfuckInstruction::OutputChar => {
                        // . Output the ASCII character corresponding to the value of the current cell
                        let v: char = self.vm.memory[self.vm.cell_id] as char;
                        io.output_char(&v);
                    }
                    BrainfuckInstruction::InputChar => {
                        // , Input a character and store its ASCII value in the current cell
                        let value = io.next_input();
                        self.vm.memory[self.vm.cell_id] = value as u8;
                    }
                    BrainfuckInstruction::JzFront => {
                        // [ If the value of the cell is zero, jump to the corresponding ] character
                        if self.vm.memory[self.vm.cell_id] == 0 {
                            self.vm.ip = *self.brackets_mapping.get(&self.vm.ip).unwrap();
                        }
                    }
                    BrainfuckInstruction::JnzBack => {
                        // ] if the value of the current cell is non-zero, jump back to the corresponding [
                        if self.vm.memory[self.vm.cell_id] != 0 {
                            self.vm.ip = *self.brackets_mapping.get(&self.vm.ip).unwrap();
                        }
                    }
                }
                self.vm.ip += 1;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::InMemoryIO;
    use std::panic;

    #[test]
    fn test_z_simple() {
        // prints "Z" by incrementing 90 times
        let sample = "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.";
        let mut io = InMemoryIO::default();
        let mut bf = BrainfuckInterpreter::new(&sample);
        bf.run(&mut io);
        assert_eq!(io.output, vec!['Z']);
    }

    #[test]
    fn test_z_with_loop() {
        // prints "Z" with a loop
        let sample = "+++++++++[>++++++++++<-]>.";
        let mut io = InMemoryIO::default();
        let mut bf = BrainfuckInterpreter::new(&sample);
        bf.run(&mut io);
        assert_eq!(io.output, vec!['Z']);
    }

    fn it_does_not_crash_with(sample:&str){
        let res = panic::catch_unwind(|| {
            let mut bf = BrainfuckInterpreter::new(&sample);
            let mut io = InMemoryIO::default();
            bf.run(&mut io)
        });
        // Ensure the BF interpreter does not crash when running the program
        assert!(res.is_ok());
    }

    #[test]
    fn test_loop_0_to_99() {
        let sample = include_str!("../samples/0-to-99.bf");
        it_does_not_crash_with(sample);
    }

    #[test]
    fn test_sample_brainfuck() {
        let sample = include_str!("../samples/brainfuck.bf");
        it_does_not_crash_with(sample);
    }

    #[test]
    fn test_has_enough_memory(){
        // The VM should have at least 30k cells
        let sample = "++++[>++++++<-]>[>+++++>+++++++<<-]>>++++<[[>[[>>+<<-]<]>>>-]>-[>+>+<<-]>]+++++[>+++++++<<++>-]>.<<.";
        let mut io = InMemoryIO::default();
        let mut bf = BrainfuckInterpreter::new(&sample);
        bf.run(&mut io);
        assert_eq!(io.output, vec!['#', '\n']);
    }

    #[test]
    fn test_decrease(){

        let sample = ",[-]";
        let res = panic::catch_unwind(|| {
            let mut io = InMemoryIO::new_with_inputs(vec![4.into()]);
            let mut bf = BrainfuckInterpreter::new(&sample);
            bf.run(&mut io);
        });
        assert!(res.is_ok());
    }

    #[ignore]
    #[test]
    fn test_sierpinski() {
        let sample = include_str!("../samples/sierpinski.bf");
        it_does_not_crash_with(sample);
    }
}
