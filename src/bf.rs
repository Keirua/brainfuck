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
    let mut out: Vec<BrainfuckInstruction> = Vec::new();
    for c in program.chars() {
        match c {
            '+' => out.push(BrainfuckInstruction::IncrValue),
            '-' => out.push(BrainfuckInstruction::DecrValue),
            '>' => out.push(BrainfuckInstruction::IncrPointer),
            '<' => out.push(BrainfuckInstruction::DecrPointer),
            '[' => out.push(BrainfuckInstruction::JzFront),
            ']' => out.push(BrainfuckInstruction::JnzBack),
            '.' => out.push(BrainfuckInstruction::OutputChar),
            ',' => out.push(BrainfuckInstruction::InputChar),
            _ => {
                // ignore everything else
            }
        }
    }
    out
}

pub struct BrainfuckInterpreter {
    memory: Vec<u8>, // hardcoded 256 cells
    cell_id: usize,
    ip: usize,
    instructions: Vec<BrainfuckInstruction>,
    brackets_mapping: HashMap<usize, usize>,
}

impl BrainfuckInterpreter {
    pub fn new(program: &str) -> BrainfuckInterpreter {
        let instructions = parse(program);
        let brackets_mapping = parse_brackets(&instructions).unwrap();
        BrainfuckInterpreter {
            memory: [0; 256].into(),
            cell_id: 0usize,
            ip: 0usize,
            instructions,
            brackets_mapping,
        }
    }

    pub fn run(&mut self, io: &mut impl BrainfuckIo) {
        loop {
            if self.ip < self.instructions.len() {
                match self.instructions[self.ip] {
                    BrainfuckInstruction::IncrValue => {
                        // + Increment the value of the cell by 1
                        self.memory[self.cell_id] += 1;
                    }
                    BrainfuckInstruction::DecrValue => {
                        // - Decrement the value of the cell by 1
                        self.memory[self.cell_id] -= 1;
                    }
                    BrainfuckInstruction::IncrPointer => {
                        // > Move the pointer to the next cell to the right
                        self.cell_id += 1;
                    }
                    BrainfuckInstruction::DecrPointer => {
                        // < Move the pointer to the next cell to the left
                        self.cell_id -= 1;
                    }
                    BrainfuckInstruction::OutputChar => {
                        // . Output the ASCII character corresponding to the value of the current cell
                        let v: char = self.memory[self.cell_id] as char;
                        io.output_char(&v);
                    }
                    BrainfuckInstruction::InputChar => {
                        // , Input a character and store its ASCII value in the current cell
                        unimplemented!(
                            ", Input a character and store its ASCII value in the current cell"
                        );
                    }
                    BrainfuckInstruction::JzFront => {
                        // [ If the value of the cell is zero, jump to the corresponding ] character
                        if self.memory[self.cell_id] == 0 {
                            self.ip = *self.brackets_mapping.get(&self.ip).unwrap();
                        }
                    }
                    BrainfuckInstruction::JnzBack => {
                        // ] if the value of the current cell is non-zero, jump back to the corresponding [
                        if self.memory[self.cell_id] != 0 {
                            self.ip = *self.brackets_mapping.get(&self.ip).unwrap();
                        }
                    }
                }
                self.ip += 1;
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
}
