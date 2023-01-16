pub trait BrainfuckIo {
    fn output_char(&mut self, c: &char);
    fn next_input(&mut self) -> char;
}

#[derive(Debug, Default)]
pub struct InMemoryIO {
    pub output: Vec<char>,
    inputs: Vec<char>,
}

impl InMemoryIO {
    pub fn new_with_inputs(inputs: Vec<char>) -> InMemoryIO {
        InMemoryIO {
            output: Vec::new(),
            inputs,
        }
    }
}

impl BrainfuckIo for InMemoryIO {
    fn output_char(&mut self, c: &char) {
        self.output.push(*c);
    }

    fn next_input(&mut self) -> char {
        self.inputs.rotate_right(1);
        // requires the caller to have enough inputs
        self.inputs.pop().unwrap()
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
