pub trait BrainfuckIo {
    fn output_char(&mut self, c: &char);
    fn next_input(&mut self) -> char;
}

#[derive(Debug, Default)]
pub struct InMemoryIO {
    pub output: Vec<char>,
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
