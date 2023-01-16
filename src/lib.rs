// Only 8 valid tokens
// +-<>[].,
// + Increment the value of the cell by 1
// - Decrement the value of the cell by 1
// > Move the pointer to the next cell to the right
// < Move the pointer to the next cell to the left
// . Output the ASCII character corresponding to the value of the current cell
// , Input a character and store its ASCII value in the current cell
// [ If the value of the cell is zero, jump to the corresponding ] character
// ] if the value of the current cell is non-zero, jump back to the corresponding [
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

fn parse_brackets(program: &Vec<BrainfuckInstruction>) -> Option<HashMap<usize, usize>> {
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

pub trait BrainfuckIo {
    fn output_char(&mut self, c: &char);
    fn next_input(&mut self) -> char;
}

#[derive(Debug, Default)]
pub struct InMemoryIO {
    output: Vec<char>,
    _inputs: Vec<char>,
}

impl BrainfuckIo for InMemoryIO {
    fn output_char(&mut self, c: &char) {
        self.output.push(*c);
    }

    fn next_input(&mut self) -> char {
        todo!()
    }
}

pub struct StdIO {}
impl BrainfuckIo for StdIO {
    fn output_char(&mut self, c: &char) {
        let c = *c;
        print!("{c}")
    }

    fn next_input(&mut self) -> char {
        todo!()
    }
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
//

pub fn run_program(program: &str, io: &mut impl BrainfuckIo) {
    let instructions = parse(program);
    let mut memory = [0u8; 256]; // hardcoded 256 cells
    let mut cell_id: usize = 0;
    let mut ip = 0usize;
    let brackets_mapping = parse_brackets(&instructions).unwrap();
    println!("{:?}", brackets_mapping);
    loop {
        if ip < instructions.len() {
            match instructions[ip] {
                BrainfuckInstruction::IncrValue => {
                    // + Increment the value of the cell by 1
                    memory[cell_id] += 1;
                }
                BrainfuckInstruction::DecrValue => {
                    // - Decrement the value of the cell by 1
                    memory[cell_id] -= 1;
                }
                BrainfuckInstruction::IncrPointer => {
                    // > Move the pointer to the next cell to the right
                    cell_id += 1;
                }
                BrainfuckInstruction::DecrPointer => {
                    // < Move the pointer to the next cell to the left
                    cell_id -= 1;
                }
                BrainfuckInstruction::OutputChar => {
                    // . Output the ASCII character corresponding to the value of the current cell
                    let v: char = memory[cell_id] as char;
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
                    if memory[cell_id] == 0 {
                        ip = *brackets_mapping.get(&ip).unwrap();
                    }
                }
                BrainfuckInstruction::JnzBack => {
                    // ] if the value of the current cell is non-zero, jump back to the corresponding [
                    if memory[cell_id] != 0 {
                        ip = *brackets_mapping.get(&ip).unwrap();
                    }
                }
            }
            ip += 1;
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_simple() {
        // prints "Z" by incrementing 90 times
        let sample = "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.";
        let mut io = InMemoryIO::default();
        run_program(&sample, &mut io);
        assert_eq!(io.output, vec!['Z']);
    }

    #[test]
    fn test_z_with_loop() {
        // prints "Z" with a loop
        let sample = "+++++++++[>++++++++++<-]>.";
        let mut io = InMemoryIO::default();
        run_program(&sample, &mut io);
        assert_eq!(io.output, vec!['Z']);
    }
}
