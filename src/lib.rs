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

enum Instruction {
    IncrValue,
    DecrValue,
    IncrPointer,
    DecrPointer,
    OutputChar,
    InputChar,
    JZ,
    JNZ,
}

fn parse_brackets(program: &str) -> Option<HashMap<usize, usize>> {
    let mut brackets_mapping: HashMap<usize, usize> = HashMap::new();
    let mut par_stack: Vec<(char, usize)> = Vec::new();
    for (pos, c) in program.chars().enumerate() {
        match c {
            '[' => {
                par_stack.push(('[' as char, pos));
            }
            ']' => {
                match par_stack.pop() {
                    Some(matching_open) => {
                        let opening_pos = matching_open.1;
                        let closing_pos = pos;
                        brackets_mapping.insert(opening_pos, closing_pos);
                        brackets_mapping.insert(closing_pos, opening_pos);
                        // brackets_mapping[*pos] = opening_pos;
                    }
                    None => return None,
                }
            }
            _ => {}
        }
    }
    Some(brackets_mapping)
}

pub trait BrainfuckIo {
    fn output_char(&mut self, c: &char);
    fn next_input(&mut self) -> char;
}

#[derive(Debug)]
pub struct InMemoryIO {
    output: Vec<char>,
    inputs: Vec<char>,
}

impl Default for InMemoryIO {
    fn default() -> Self {
        InMemoryIO {
            output: Vec::new(),
            inputs: Vec::new(),
        }
    }
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

pub fn run_program(program: &str, io: &mut impl BrainfuckIo) {
    let mut memory = [0u8; 256]; // hardcoded 256 cells
    let mut cell_id: usize = 0;
    let mut ip = 0usize;
    let brackets_mapping = parse_brackets(&program).unwrap();
    loop {
        if ip < program.len() {
            match program.chars().nth(ip) {
                Some('+') => {
                    // + Increment the value of the cell by 1
                    memory[cell_id] += 1;
                }
                Some('-') => {
                    // - Decrement the value of the cell by 1
                    memory[cell_id] -= 1;
                }
                Some('>') => {
                    // > Move the pointer to the next cell to the right
                    cell_id += 1;
                }
                Some('<') => {
                    // < Move the pointer to the next cell to the left
                    cell_id -= 1;
                }
                Some('.') => {
                    // . Output the ASCII character corresponding to the value of the current cell
                    let v: char = memory[cell_id] as char;
                    io.output_char(&v);
                }
                Some(',') => {
                    // , Input a character and store its ASCII value in the current cell
                    unimplemented!(
                        ", Input a character and store its ASCII value in the current cell"
                    );
                }
                Some('[') => {
                    // [ If the value of the cell is zero, jump to the corresponding ] character
                    if memory[cell_id] == 0 {
                        ip = *brackets_mapping.get(&ip).unwrap();
                    }
                }
                Some(']') => {
                    // ] if the value of the current cell is non-zero, jump back to the corresponding [
                    if memory[cell_id] != 0 {
                        ip = *brackets_mapping.get(&ip).unwrap();
                    }
                }
                _ => panic!("Should not happen"),
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
